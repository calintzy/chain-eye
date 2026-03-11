use crate::chain::types::{BlockInfo, TxInfo};

/// WebSocket 연결 상태
#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Reconnecting { attempt: u32 },
    Disconnected { reason: String },
}

/// 체인에서 수신하는 이벤트
#[derive(Debug)]
pub enum ChainEvent {
    NewBlock(BlockInfo),
    NewTransactions(Vec<TxInfo>),
    ConnectionStatus(ConnectionState),
}
