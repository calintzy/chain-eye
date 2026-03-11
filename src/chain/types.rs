use std::fmt;

use crate::chain::chains::ChainId;
use crate::token::decode::TokenTransfer;

/// 트랜잭션 유형
#[derive(Debug, Clone, PartialEq)]
pub enum TxType {
    Transfer,
    ContractCall,
    ContractCreation,
}

impl fmt::Display for TxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TxType::Transfer => write!(f, "Transfer"),
            TxType::ContractCall => write!(f, "Contract Call"),
            TxType::ContractCreation => write!(f, "Contract Create"),
        }
    }
}

/// 화면에 표시할 트랜잭션 정보 (간소화)
#[derive(Debug, Clone)]
pub struct TxInfo {
    pub hash: String,
    pub hash_full: String,
    pub from: String,
    pub from_full: String,
    pub to: Option<String>,
    pub to_full: Option<String>,
    pub value_wei: String,
    pub value_eth: f64,
    pub gas_price_gwei: f64,
    pub gas_used: u64,
    pub block_number: u64,
    pub timestamp: u64,
    pub input_size: usize,
    pub tx_type: TxType,
    pub chain_id: ChainId,
    pub token_transfer: Option<TokenTransfer>,
}

/// 블록 정보
#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub number: u64,
    pub timestamp: u64,
    pub tx_count: usize,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub base_fee_gwei: Option<f64>,
}

/// 주소를 축약 형태로 표시 (0xabcd..ef01)
pub fn abbreviate_address(addr: &str, len: usize) -> String {
    if addr.len() <= len * 2 + 4 {
        return addr.to_string();
    }
    format!("{}..{}", &addr[..len + 2], &addr[addr.len() - len..])
}

/// wei를 ETH로 변환
pub fn wei_to_eth(wei: &str) -> f64 {
    // U256 hex 문자열을 파싱
    let wei_val = u128::from_str_radix(wei.trim_start_matches("0x"), 16).unwrap_or(0);
    wei_val as f64 / 1e18
}

/// wei를 Gwei로 변환
pub fn wei_to_gwei(wei: u128) -> f64 {
    wei as f64 / 1e9
}
