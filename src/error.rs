/// 애플리케이션 에러 타입
#[derive(thiserror::Error, Debug)]
pub enum ChainEyeError {
    #[error("WebSocket 연결 실패: {0}")]
    WsConnection(String),

    #[error("RPC 요청 실패: {0}")]
    RpcRequest(String),

    #[error("최대 재연결 시도 초과 ({0}회)")]
    MaxReconnectExceeded(u32),

    #[error("설정 파일 로딩 실패: {0}")]
    ConfigLoad(String),

    #[error("트랜잭션 파싱 실패: {0}")]
    TxParse(String),

    #[error("TUI 렌더링 오류: {0}")]
    UiRender(String),
}
