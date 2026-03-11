use std::collections::HashMap;
use std::time::Duration;

use alloy::consensus::Transaction as TxTrait;
use alloy::network::TransactionResponse;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures_util::StreamExt;
use tokio::sync::mpsc;

use crate::chain::chains::ChainId;
use crate::chain::types::{abbreviate_address, BlockInfo, TxInfo, TxType};
use crate::config::RpcConfig;
use crate::event::{ChainEvent, ConnectionState};
use crate::filter::TxFilter;
use crate::token::decode::{decode_token_transfer, TokenTransfer};
use crate::token::known::{self, TokenInfo};

/// 단일 WebSocket 연결 시도 (메인 URL + fallback)
macro_rules! try_connect {
    ($config:expr, $chain_id:expr, $event_tx:expr) => {{
        let ws = WsConnect::new(&$config.ws_url);
        match ProviderBuilder::new().connect_ws(ws).await {
            Ok(p) => {
                let _ = $event_tx
                    .send(ChainEvent::ConnectionStatus(
                        $chain_id,
                        ConnectionState::Connected,
                    ))
                    .await;
                Some(p)
            }
            Err(e) => {
                let _ = $event_tx
                    .send(ChainEvent::ConnectionStatus(
                        $chain_id,
                        ConnectionState::Disconnected {
                            reason: format!("[{}] 연결 실패: {}", $chain_id, e),
                        },
                    ))
                    .await;

                if let Some(ref fallback_url) = $config.fallback_ws_url {
                    let ws2 = WsConnect::new(fallback_url);
                    match ProviderBuilder::new().connect_ws(ws2).await {
                        Ok(p) => {
                            let _ = $event_tx
                                .send(ChainEvent::ConnectionStatus(
                                    $chain_id,
                                    ConnectionState::Connected,
                                ))
                                .await;
                            Some(p)
                        }
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
        }
    }};
}

/// Exponential Backoff 재연결 매크로
macro_rules! reconnect_with_backoff {
    ($config:expr, $chain_id:expr, $event_tx:expr) => {{
        let max_attempts = $config.max_reconnect_attempts;
        let base_delay = $config.reconnect_base_delay_ms;
        let mut result = None;

        for attempt in 1..=max_attempts {
            let _ = $event_tx
                .send(ChainEvent::ConnectionStatus(
                    $chain_id,
                    ConnectionState::Reconnecting { attempt },
                ))
                .await;

            let delay = std::cmp::min(base_delay * 2u64.pow(attempt - 1), 30_000);
            tokio::time::sleep(Duration::from_millis(delay)).await;

            if let Some(p) = try_connect!($config, $chain_id, $event_tx) {
                result = Some(p);
                break;
            }
        }

        if result.is_none() {
            let _ = $event_tx
                .send(ChainEvent::ConnectionStatus(
                    $chain_id,
                    ConnectionState::Disconnected {
                        reason: format!(
                            "[{}] 최대 재연결 시도 초과 ({}회)",
                            $chain_id, max_attempts
                        ),
                    },
                ))
                .await;
        }

        result
    }};
}

/// 블록 내 트랜잭션을 파싱하여 TxInfo 벡터로 변환
fn parse_transactions(
    block: &alloy::rpc::types::Block<alloy::rpc::types::Transaction>,
    chain_id: ChainId,
    filter: &TxFilter,
    addr_len: usize,
    known_tokens: &HashMap<(ChainId, String), TokenInfo>,
) -> Vec<TxInfo> {
    let mut txs = Vec::new();
    for tx in block.transactions.txns() {
        let hash_full = format!("{:#x}", tx.tx_hash());
        let from_full = format!("{:#x}", tx.from());
        let to_addr = tx.to();
        let to_full = to_addr.map(|a| format!("{:#x}", a));
        let value = tx.value();
        let value_eth = value.to::<u128>() as f64 / 1e18;
        let input = tx.input();
        let input_len = input.len();

        let tx_type = if to_full.is_none() {
            TxType::ContractCreation
        } else if input_len > 0 {
            TxType::ContractCall
        } else {
            TxType::Transfer
        };

        // 토큰 디코딩 시도
        let token_transfer = if input_len >= 4 {
            let mut transfer =
                decode_token_transfer(input, to_full.as_deref(), chain_id, known_tokens);
            // ERC-20 transfer의 from 필드 채우기
            if let Some(TokenTransfer::Erc20 { ref mut from, .. }) = transfer {
                if from.is_empty() {
                    *from = from_full.clone();
                }
            }
            transfer
        } else {
            None
        };

        let gas_price_gwei = tx
            .effective_gas_price
            .map(|p| p as f64 / 1e9)
            .unwrap_or(0.0);
        let gas_limit = tx.inner.gas_limit();

        let info = TxInfo {
            hash: abbreviate_address(&hash_full, addr_len),
            hash_full,
            from: abbreviate_address(&from_full, addr_len),
            from_full,
            to: to_full.as_ref().map(|a| abbreviate_address(a, addr_len)),
            to_full,
            value_wei: format!("{}", value),
            value_eth,
            gas_price_gwei,
            gas_used: gas_limit,
            block_number: block.header.number,
            timestamp: block.header.timestamp,
            input_size: input_len,
            tx_type,
            chain_id,
            token_transfer,
        };

        if filter.matches(&info) {
            txs.push(info);
        }
    }
    txs
}

/// 메인 체인 프로바이더 태스크 (자동 재연결 포함)
pub async fn chain_provider_task(
    chain_id: ChainId,
    config: RpcConfig,
    filter: TxFilter,
    addr_len: usize,
    event_tx: mpsc::Sender<ChainEvent>,
) {
    // 주요 토큰 목록 빌드
    let known_tokens = known::build_known_tokens();

    // 초기 연결
    let mut provider = match try_connect!(config, chain_id, event_tx) {
        Some(p) => p,
        None => match reconnect_with_backoff!(config, chain_id, event_tx) {
            Some(p) => p,
            None => return,
        },
    };

    loop {
        // 블록 구독
        let sub = match provider.subscribe_blocks().await {
            Ok(s) => s,
            Err(e) => {
                let _ = event_tx
                    .send(ChainEvent::ConnectionStatus(
                        chain_id,
                        ConnectionState::Disconnected {
                            reason: format!("[{}] 블록 구독 실패: {}", chain_id, e),
                        },
                    ))
                    .await;

                match reconnect_with_backoff!(config, chain_id, event_tx) {
                    Some(p) => {
                        provider = p;
                        continue;
                    }
                    None => return,
                }
            }
        };

        let mut stream = sub.into_stream();

        while let Some(header) = stream.next().await {
            let block_number = header.number;

            let block = match provider.get_block_by_number(block_number.into()).full().await {
                Ok(Some(block)) => block,
                Ok(None) => continue,
                Err(_) => continue,
            };

            // BlockInfo 전송
            let block_info = BlockInfo {
                number: block.header.number,
                timestamp: block.header.timestamp,
                tx_count: block.transactions.len(),
                gas_used: block.header.gas_used,
                gas_limit: block.header.gas_limit,
                base_fee_gwei: block.header.base_fee_per_gas.map(|fee| fee as f64 / 1e9),
            };
            let _ = event_tx
                .send(ChainEvent::NewBlock(chain_id, block_info))
                .await;

            // 트랜잭션 파싱 + 필터
            let txs =
                parse_transactions(&block, chain_id, &filter, addr_len, &known_tokens);
            if !txs.is_empty() {
                let _ = event_tx
                    .send(ChainEvent::NewTransactions(chain_id, txs))
                    .await;
            }
        }

        // 스트림 종료 = 연결 끊김 → 재연결 시도
        let _ = event_tx
            .send(ChainEvent::ConnectionStatus(
                chain_id,
                ConnectionState::Disconnected {
                    reason: format!("[{}] WebSocket 연결 끊김", chain_id),
                },
            ))
            .await;

        match reconnect_with_backoff!(config, chain_id, event_tx) {
            Some(p) => {
                provider = p;
            }
            None => return,
        }
    }
}
