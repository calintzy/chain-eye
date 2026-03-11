use tokio::sync::mpsc;

use crate::chain::chains::ChainId;
use crate::chain::provider::chain_provider_task;
use crate::config::{ChainConfig, RpcConfig};
use crate::event::ChainEvent;
use crate::filter::TxFilter;

/// 멀티체인 프로바이더 관리
pub struct ChainManager;

impl ChainManager {
    /// 활성 체인별로 provider task를 spawn
    pub fn spawn_providers(
        chains: &[(ChainId, ChainConfig)],
        global_rpc: &RpcConfig,
        filter: &TxFilter,
        addr_len: usize,
        event_tx: mpsc::Sender<ChainEvent>,
    ) {
        for (chain_id, chain_config) in chains {
            let rpc_config = RpcConfig {
                ws_url: chain_config
                    .ws_url
                    .clone()
                    .unwrap_or_else(|| chain_id.default_ws_url().to_string()),
                fallback_ws_url: chain_config.fallback_ws_url.clone(),
                max_reconnect_attempts: global_rpc.max_reconnect_attempts,
                reconnect_base_delay_ms: global_rpc.reconnect_base_delay_ms,
            };
            let chain_id = *chain_id;
            let filter = filter.clone();
            let tx = event_tx.clone();

            tokio::spawn(async move {
                chain_provider_task(chain_id, rpc_config, filter, addr_len, tx).await;
            });
        }
    }
}
