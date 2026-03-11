use crate::chain::chains::ChainId;
use crate::chain::types::{BlockInfo, TxInfo};

/// WebSocket 연결 상태
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Reconnecting { attempt: u32 },
    Disconnected { reason: String },
}

/// 체인에서 수신하는 이벤트 (체인 ID 포함)
#[derive(Debug)]
pub enum ChainEvent {
    NewBlock(ChainId, BlockInfo),
    NewTransactions(ChainId, Vec<TxInfo>),
    ConnectionStatus(ChainId, ConnectionState),
}
