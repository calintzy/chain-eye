use std::collections::HashMap;

use crate::chain::chains::ChainId;
use crate::token::known::{self, TokenInfo};

/// 토큰 전송 정보
#[derive(Debug, Clone)]
pub enum TokenTransfer {
    Erc20 {
        token_address: String,
        symbol: String,
        from: String,
        to: String,
        amount: f64,
        decimals: u8,
    },
    Erc721 {
        token_address: String,
        from: String,
        to: String,
        token_id: String,
    },
}

/// ERC-20 transfer(address,uint256) 함수 셀렉터: 0xa9059cbb
const ERC20_TRANSFER_SELECTOR: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
/// ERC-20/721 transferFrom(address,address,uint256) 함수 셀렉터: 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

/// 트랜잭션 input data에서 토큰 전송 디코딩 시도
pub fn decode_token_transfer(
    input: &[u8],
    to_address: Option<&str>,
    chain_id: ChainId,
    known_tokens: &HashMap<(ChainId, String), TokenInfo>,
) -> Option<TokenTransfer> {
    if input.len() < 4 {
        return None;
    }

    let selector = &input[..4];
    let token_addr = to_address?.to_lowercase();

    if selector == ERC20_TRANSFER_SELECTOR && input.len() >= 68 {
        // transfer(address to, uint256 value)
        let recipient = format!("0x{}", hex::encode(&input[16..36]));
        let value_bytes = &input[36..68];
        let (symbol, decimals) = match known::lookup_symbol(known_tokens, chain_id, &token_addr) {
            Some(info) => (info.symbol.to_string(), info.decimals),
            None => ("???".to_string(), 18),
        };
        let amount = bytes_to_f64(value_bytes, decimals);

        return Some(TokenTransfer::Erc20 {
            token_address: token_addr,
            symbol,
            from: String::new(), // tx.from에서 채워짐
            to: recipient,
            amount,
            decimals,
        });
    }

    if selector == TRANSFER_FROM_SELECTOR && input.len() >= 100 {
        // transferFrom(address from, address to, uint256 value)
        let from = format!("0x{}", hex::encode(&input[16..36]));
        let to = format!("0x{}", hex::encode(&input[48..68]));
        let value_bytes = &input[68..100];

        // ERC-721인지 ERC-20인지 구분: value가 매우 작으면 token ID로 간주
        let value_u128 = bytes_to_u128(value_bytes);

        if value_u128 < 1_000_000 && known::lookup_symbol(known_tokens, chain_id, &token_addr).is_none() {
            // NFT (ERC-721) 가능성 높음
            return Some(TokenTransfer::Erc721 {
                token_address: token_addr,
                from,
                to,
                token_id: value_u128.to_string(),
            });
        }

        let (symbol, decimals) = match known::lookup_symbol(known_tokens, chain_id, &token_addr) {
            Some(info) => (info.symbol.to_string(), info.decimals),
            None => ("???".to_string(), 18),
        };
        let amount = bytes_to_f64(value_bytes, decimals);

        return Some(TokenTransfer::Erc20 {
            token_address: token_addr,
            symbol,
            from,
            to,
            amount,
            decimals,
        });
    }

    None
}

/// 바이트 배열을 u128로 변환
fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut result: u128 = 0;
    for &b in bytes.iter().take(32) {
        result = result.checked_shl(8).unwrap_or(0) | (b as u128);
    }
    result
}

/// 바이트 배열을 소수점 금액으로 변환
fn bytes_to_f64(bytes: &[u8], decimals: u8) -> f64 {
    let raw = bytes_to_u128(bytes);
    raw as f64 / 10f64.powi(decimals as i32)
}
