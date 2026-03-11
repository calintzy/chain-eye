# Chain-Eye Design Document

> Feature: chain-eye
> Created: 2026-03-11
> Phase: Design
> Level: Dynamic
> Plan Reference: docs/01-plan/features/chain-eye.plan.md

---

## 1. 아키텍처 개요

### 1.1 시스템 아키텍처

```
┌─────────────────────────────────────────────────────────┐
│                      chain-eye                           │
│                                                          │
│  ┌──────────┐    ┌───────────┐    ┌──────────────────┐  │
│  │   CLI    │───▶│    App    │───▶│       UI         │  │
│  │  (clap)  │    │  (State)  │    │   (ratatui)      │  │
│  └──────────┘    └─────┬─────┘    └──────────────────┘  │
│                        │                                  │
│                  ┌─────┴─────┐                            │
│                  │  Channel  │                            │
│                  │ (mpsc tx) │                            │
│                  └─────┬─────┘                            │
│                        │                                  │
│  ┌──────────┐    ┌─────┴─────┐    ┌──────────────────┐  │
│  │  Config  │───▶│  Chain    │───▶│   Filter         │  │
│  │  (toml)  │    │ Provider  │    │  (value/addr)    │  │
│  └──────────┘    └───────────┘    └──────────────────┘  │
│                        │                                  │
│                        ▼                                  │
│               ┌────────────────┐                         │
│               │  Ethereum RPC  │                         │
│               │  (WebSocket)   │                         │
│               └────────────────┘                         │
└─────────────────────────────────────────────────────────┘
```

### 1.2 데이터 플로우

```
[Ethereum Node]
     │
     │ WebSocket (wss://)
     ▼
[ChainProvider] ── newHeads 구독 ──▶ 새 블록 감지
     │
     │ eth_getBlockByNumber (full=true)
     ▼
[Transaction Parser] ── 블록 내 트랜잭션 추출
     │
     │ Filter 적용 (value, from, to)
     ▼
[mpsc::channel] ── ChainEvent 전송
     │
     ▼
[App State] ── transactions: VecDeque<TxInfo> 업데이트
     │
     │ 60fps tick
     ▼
[UI Render] ── ratatui Terminal::draw()
```

### 1.3 비동기 구조

```
tokio::main
├── spawn: chain_provider_task (WebSocket 구독 + 블록 처리)
├── spawn: reconnect_monitor_task (연결 상태 감시)
└── main loop: TUI 이벤트 루프
    ├── crossterm::event::poll (키보드 입력)
    └── mpsc::Receiver::try_recv (체인 이벤트)
```

---

## 2. 모듈 상세 설계

### 2.1 프로젝트 구조

```
chain-eye/
├── Cargo.toml
├── src/
│   ├── main.rs                 # 엔트리포인트
│   ├── app.rs                  # App 상태 + 이벤트 루프
│   ├── event.rs                # 이벤트 타입 정의
│   ├── config.rs               # 설정 파일 로딩
│   ├── filter.rs               # 트랜잭션 필터링
│   ├── chain/
│   │   ├── mod.rs              # 체인 모듈 re-export
│   │   ├── provider.rs         # WebSocket 프로바이더
│   │   └── types.rs            # TxInfo, BlockInfo 타입
│   └── ui/
│       ├── mod.rs              # UI 모듈 re-export
│       ├── layout.rs           # 메인 레이아웃 분할
│       ├── header.rs           # 헤더 (체인명, 연결 상태)
│       ├── tx_list.rs          # 트랜잭션 목록 테이블
│       ├── tx_detail.rs        # 트랜잭션 상세 패널
│       ├── status_bar.rs       # 하단 상태바
│       └── help.rs             # 단축키 도움말 바
├── config/
│   └── default.toml            # 기본 설정 파일
└── README.md
```

### 2.2 모듈별 책임

| 모듈 | 파일 | 책임 |
|------|------|------|
| **main** | `main.rs` | CLI 파싱 (clap), Config 로딩, App 생성 및 실행 |
| **app** | `app.rs` | App 상태 관리, 메인 이벤트 루프, 키보드 입력 처리 |
| **event** | `event.rs` | ChainEvent, AppEvent enum 정의 |
| **config** | `config.rs` | TOML 설정 파일 로딩, CLI 인자 병합 |
| **filter** | `filter.rs` | 트랜잭션 필터 조건 적용 |
| **chain/provider** | `chain/provider.rs` | WebSocket 연결, 블록 구독, 트랜잭션 파싱 |
| **chain/types** | `chain/types.rs` | TxInfo, BlockInfo 데이터 구조체 |
| **ui/layout** | `ui/layout.rs` | 화면 영역 분할 (header, main, detail, status) |
| **ui/header** | `ui/header.rs` | 체인명, 블록 번호, 연결 상태 표시 |
| **ui/tx_list** | `ui/tx_list.rs` | 트랜잭션 목록 테이블 렌더링 |
| **ui/tx_detail** | `ui/tx_detail.rs` | 선택된 트랜잭션 상세 정보 |
| **ui/status_bar** | `ui/status_bar.rs` | 하단 상태 정보 (TX 수, 필터 등) |
| **ui/help** | `ui/help.rs` | 키보드 단축키 가이드 |

---

## 3. 핵심 데이터 구조

### 3.1 트랜잭션 정보

```rust
/// 화면에 표시할 트랜잭션 정보 (간소화)
pub struct TxInfo {
    pub hash: String,           // 0xabcd..ef01 (축약 표시)
    pub hash_full: String,      // 전체 해시
    pub from: String,           // 0xabcd..ef01 (축약)
    pub from_full: String,      // 전체 주소
    pub to: Option<String>,     // 수신자 (축약), None = 컨트랙트 생성
    pub to_full: Option<String>,
    pub value_wei: String,      // 원본 wei 값
    pub value_eth: f64,         // ETH 변환 값
    pub gas_price_gwei: f64,    // Gas Price (Gwei)
    pub gas_used: u64,          // Gas 사용량
    pub block_number: u64,      // 블록 번호
    pub timestamp: u64,         // Unix 타임스탬프
    pub input_size: usize,      // Input data 크기 (바이트)
    pub tx_type: TxType,        // 트랜잭션 유형
}

pub enum TxType {
    Transfer,           // 단순 ETH 전송
    ContractCall,       // 컨트랙트 함수 호출
    ContractCreation,   // 컨트랙트 배포
}
```

### 3.2 블록 정보

```rust
pub struct BlockInfo {
    pub number: u64,
    pub timestamp: u64,
    pub tx_count: usize,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub base_fee_gwei: Option<f64>,
}
```

### 3.3 체인 이벤트

```rust
pub enum ChainEvent {
    NewBlock(BlockInfo),
    NewTransactions(Vec<TxInfo>),
    ConnectionStatus(ConnectionState),
}

pub enum ConnectionState {
    Connected,
    Reconnecting { attempt: u32 },
    Disconnected { reason: String },
}
```

### 3.4 앱 상태

```rust
pub struct App {
    // 데이터
    pub transactions: VecDeque<TxInfo>,  // 최대 1000개 유지
    pub current_block: Option<BlockInfo>,
    pub connection_state: ConnectionState,

    // UI 상태
    pub selected_index: usize,           // 목록 선택 위치
    pub show_detail: bool,               // 상세 패널 표시 여부
    pub scroll_offset: usize,            // 스크롤 오프셋

    // 필터
    pub filter: TxFilter,

    // 설정
    pub config: AppConfig,

    // 통신
    pub chain_rx: mpsc::Receiver<ChainEvent>,

    // 상태
    pub running: bool,
    pub tx_total_count: u64,             // 총 수신 트랜잭션 수
}

const MAX_TRANSACTIONS: usize = 1000;   // 메모리 내 최대 보관 수
```

### 3.5 필터

```rust
pub struct TxFilter {
    pub min_value_eth: Option<f64>,      // 최소 ETH 값
    pub from_address: Option<String>,     // from 주소 필터
    pub to_address: Option<String>,       // to 주소 필터
    pub active: bool,                     // 필터 활성화 여부
}

impl TxFilter {
    pub fn matches(&self, tx: &TxInfo) -> bool {
        if !self.active { return true; }

        if let Some(min) = self.min_value_eth {
            if tx.value_eth < min { return false; }
        }
        if let Some(ref addr) = self.from_address {
            if !tx.from_full.eq_ignore_ascii_case(addr) { return false; }
        }
        if let Some(ref addr) = self.to_address {
            if let Some(ref to) = tx.to_full {
                if !to.eq_ignore_ascii_case(addr) { return false; }
            } else { return false; }
        }
        true
    }
}
```

### 3.6 설정

```rust
pub struct AppConfig {
    pub rpc: RpcConfig,
    pub ui: UiConfig,
}

pub struct RpcConfig {
    pub ws_url: String,               // WebSocket URL
    pub fallback_ws_url: Option<String>, // 백업 URL
    pub max_reconnect_attempts: u32,  // 최대 재연결 시도 (기본: 10)
    pub reconnect_base_delay_ms: u64, // 재연결 기본 딜레이 (기본: 1000)
}

pub struct UiConfig {
    pub max_transactions: usize,      // 화면 보관 최대 수 (기본: 1000)
    pub tick_rate_ms: u64,            // UI 갱신 주기 (기본: 16 = ~60fps)
    pub address_display_len: usize,   // 주소 축약 길이 (기본: 8)
}
```

---

## 4. UI 레이아웃 설계

### 4.1 메인 화면 (기본 모드)

```
┌─ Header ──────────────────────────────────────────────────┐
│  Chain-Eye v0.1    ETH Mainnet    Block: 21,847,234   🟢  │
├─ Transaction List ────────────────────────────────────────┤
│  Time     Hash          From          To         Value    │
│  ──────── ──────────── ──────────── ────────── ────────── │
│▶ 12:45:03 0xab3f..8e2d 0x7c2a..1f4b 0xdef9..3c7a 2.50 E │
│  12:45:02 0x9b1e..5d8f 0x4a2c..9e1d 0xab3f..8e2d 0.00 E │
│  12:45:01 0xdef9..3c7a 0x1234..5678 0x9b1e..5d8f 0.80 E │
│  12:44:59 0x4a2c..9e1d 0xdef9..3c7a 0x7c2a..1f4b 1.20 E │
│  12:44:58 0x7c2a..1f4b 0xab3f..8e2d 0x4a2c..9e1d 5.00 E │
│  ...                                                      │
├─ Status Bar ──────────────────────────────────────────────┤
│  TX: 1,234 received | Filter: OFF | ↑↓ Navigate  Enter ▸ │
├─ Help ────────────────────────────────────────────────────┤
│  [↑↓] Select  [Enter] Detail  [F] Filter  [Q] Quit       │
└───────────────────────────────────────────────────────────┘
```

### 4.2 상세 보기 모드 (Enter 시)

```
┌─ Header ──────────────────────────────────────────────────┐
│  Chain-Eye v0.1    ETH Mainnet    Block: 21,847,234   🟢  │
├─ Transaction List ───────────┬─ Detail ──────────────────┤
│  Time     Hash        Value  │  Transaction Detail        │
│  ──────── ────────── ────── │                             │
│▶ 12:45:03 0xab3..2d  2.50E  │  Hash:  0xab3f...8e2d     │
│  12:45:02 0x9b1..8f  0.00E  │  Block: 21,847,234        │
│  12:45:01 0xdef..7a  0.80E  │  Time:  2026-03-11 12:45  │
│  12:44:59 0x4a2..1d  1.20E  │                             │
│  12:44:58 0x7c2..4b  5.00E  │  From:  0x7c2a..1f4b      │
│                              │  To:    0xdef9..3c7a      │
│                              │  Value: 2.500000 ETH      │
│                              │                             │
│                              │  Gas Price: 25.3 Gwei     │
│                              │  Gas Used:  21,000        │
│                              │  Type: Transfer           │
│                              │  Input: 0 bytes           │
├─ Status Bar ──────────────────────────────────────────────┤
│  TX: 1,234 received | Filter: OFF | [Esc] Back            │
├─ Help ────────────────────────────────────────────────────┤
│  [Esc] Back  [↑↓] Select  [F] Filter  [Q] Quit           │
└───────────────────────────────────────────────────────────┘
```

### 4.3 레이아웃 분할 규칙

```rust
// ui/layout.rs
fn build_layout(area: Rect, show_detail: bool) -> AppLayout {
    // 수직 분할: Header(3) | Main(flex) | StatusBar(1) | Help(1)
    let vertical = Layout::vertical([
        Constraint::Length(3),      // Header
        Constraint::Min(10),        // Main (트랜잭션 목록 or 분할)
        Constraint::Length(1),      // Status bar
        Constraint::Length(1),      // Help bar
    ]);

    // 상세 모드 시 Main 영역을 수평 분할
    if show_detail {
        // Main → TxList(50%) | Detail(50%)
        let horizontal = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);
    }
}
```

---

## 5. WebSocket 연결 설계

### 5.1 연결 수명주기

```
[시작] → Connect → Subscribe(newHeads) → Receive Loop
                                              │
                                    ┌─────────┴──────────┐
                                    │                      │
                              [정상 수신]           [연결 끊김]
                                    │                      │
                              Parse Block           Reconnect
                              Fetch TxList          (backoff)
                              Send ChainEvent            │
                                    │              ┌─────┴─────┐
                                    ▼              │            │
                              [계속 수신]     [성공]       [실패]
                                              Resubscribe  Retry
                                                           (max 10)
```

### 5.2 재연결 전략 (Exponential Backoff)

```rust
// chain/provider.rs
async fn reconnect_with_backoff(config: &RpcConfig) -> Result<WsClient> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        if attempt > config.max_reconnect_attempts {
            return Err(Error::MaxReconnectExceeded);
        }

        // 딜레이: 1s, 2s, 4s, 8s, ... 최대 30s
        let delay = std::cmp::min(
            config.reconnect_base_delay_ms * 2u64.pow(attempt - 1),
            30_000,
        );
        tokio::time::sleep(Duration::from_millis(delay)).await;

        // 메인 URL 시도
        match connect_ws(&config.ws_url).await {
            Ok(client) => return Ok(client),
            Err(_) => {
                // fallback URL이 있으면 시도
                if let Some(ref fallback) = config.fallback_ws_url {
                    if let Ok(client) = connect_ws(fallback).await {
                        return Ok(client);
                    }
                }
            }
        }
        // ConnectionState::Reconnecting 이벤트 전송
    }
}
```

### 5.3 블록 처리 흐름

```rust
// chain/provider.rs
async fn handle_new_block(
    provider: &WsProvider,
    block_number: u64,
    tx: &mpsc::Sender<ChainEvent>,
    filter: &TxFilter,
) -> Result<()> {
    // 1. 블록 정보 조회 (full transactions)
    let block = provider
        .get_block_by_number(block_number, true)
        .await?;

    // 2. BlockInfo 이벤트 전송
    tx.send(ChainEvent::NewBlock(BlockInfo::from(&block))).await?;

    // 3. 트랜잭션 파싱 + 필터 적용
    let transactions: Vec<TxInfo> = block
        .transactions
        .iter()
        .map(TxInfo::from)
        .filter(|t| filter.matches(t))
        .collect();

    // 4. 트랜잭션 이벤트 전송
    if !transactions.is_empty() {
        tx.send(ChainEvent::NewTransactions(transactions)).await?;
    }

    Ok(())
}
```

---

## 6. 키보드 입력 처리

### 6.1 키 바인딩

| 키 | 기본 모드 | 상세 모드 |
|----|----------|----------|
| `↑` / `k` | 이전 트랜잭션 선택 | 이전 트랜잭션 선택 |
| `↓` / `j` | 다음 트랜잭션 선택 | 다음 트랜잭션 선택 |
| `Enter` | 상세 패널 열기 | - |
| `Esc` | - | 상세 패널 닫기 |
| `f` | 필터 토글 (활성/비활성) | 필터 토글 |
| `Home` / `g` | 목록 맨 위로 | 목록 맨 위로 |
| `End` / `G` | 목록 맨 아래로 | 목록 맨 아래로 |
| `q` | 앱 종료 | 앱 종료 |

### 6.2 이벤트 처리 루프

```rust
// app.rs
pub async fn run(&mut self) -> Result<()> {
    let mut terminal = ratatui::init();

    loop {
        // 1. UI 렌더링
        terminal.draw(|frame| ui::render(frame, &self))?;

        // 2. 이벤트 처리 (16ms = ~60fps)
        if crossterm::event::poll(Duration::from_millis(self.config.ui.tick_rate_ms))? {
            if let Event::Key(key) = crossterm::event::read()? {
                self.handle_key(key);
            }
        }

        // 3. 체인 이벤트 수신 (non-blocking)
        while let Ok(event) = self.chain_rx.try_recv() {
            self.handle_chain_event(event);
        }

        // 4. 종료 확인
        if !self.running {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
```

---

## 7. CLI 인터페이스

### 7.1 명령어 구조

```rust
// main.rs
#[derive(Parser)]
#[command(name = "chain-eye")]
#[command(about = "Real-time EVM transaction monitoring TUI")]
pub struct Cli {
    /// WebSocket RPC URL (overrides config file)
    #[arg(long)]
    pub rpc: Option<String>,

    /// Minimum ETH value filter
    #[arg(long, value_name = "ETH")]
    pub min_value: Option<f64>,

    /// Filter by from address
    #[arg(long, value_name = "ADDRESS")]
    pub from: Option<String>,

    /// Filter by to address
    #[arg(long, value_name = "ADDRESS")]
    pub to: Option<String>,

    /// Config file path (default: ~/.chain-eye/config.toml)
    #[arg(long, short)]
    pub config: Option<PathBuf>,
}
```

### 7.2 사용 예시

```bash
# 기본 실행
chain-eye

# 커스텀 RPC
chain-eye --rpc wss://eth-mainnet.g.alchemy.com/v2/YOUR_KEY

# 1 ETH 이상 거래만
chain-eye --min-value 1.0

# 특정 주소 from 필터
chain-eye --from 0x1234567890abcdef1234567890abcdef12345678

# 복합 필터
chain-eye --min-value 10.0 --to 0xUniswapRouterAddress
```

---

## 8. 설정 파일 구조

### 8.1 기본 설정 (`config/default.toml`)

```toml
[rpc]
ws_url = "wss://eth.drpc.org"
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"
max_reconnect_attempts = 10
reconnect_base_delay_ms = 1000

[ui]
max_transactions = 1000
tick_rate_ms = 16
address_display_len = 8
```

### 8.2 사용자 설정 (`~/.chain-eye/config.toml`)

```toml
# 사용자 설정은 기본 설정을 오버라이드
[rpc]
ws_url = "wss://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"

[ui]
max_transactions = 2000
```

### 8.3 설정 로딩 우선순위

```
CLI 인자 > 사용자 설정 (~/.chain-eye/config.toml) > 기본 설정 (내장)
```

---

## 9. 에러 핸들링

### 9.1 에러 유형

```rust
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
```

### 9.2 에러 처리 전략

| 에러 유형 | 처리 방식 |
|----------|----------|
| WebSocket 연결 실패 | 자동 재연결 (backoff) + 상태바 표시 |
| RPC 요청 실패 | 해당 블록 스킵 + 로그 기록 |
| 최대 재연결 초과 | 상태바에 "Disconnected" 표시, 수동 재시작 안내 |
| 설정 파일 오류 | 기본값 사용 + 경고 메시지 표시 |
| 트랜잭션 파싱 실패 | 해당 TX 스킵 (목록에서 제외) |
| TUI 렌더링 오류 | 앱 종료 + 에러 메시지 출력 |

---

## 10. 의존성 목록 (Cargo.toml)

```toml
[package]
name = "chain-eye"
version = "0.1.0"
edition = "2021"
description = "Real-time EVM transaction monitoring TUI"
license = "MIT"

[dependencies]
# TUI
ratatui = "0.29"
crossterm = "0.28"

# 비동기
tokio = { version = "1", features = ["full"] }

# EVM
alloy = { version = "1.5", features = ["provider-ws", "rpc-types"] }
futures-util = "0.3"

# 직렬화
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# CLI
clap = { version = "4", features = ["derive"] }

# 에러 핸들링
thiserror = "2"
anyhow = "1"

# 유틸리티
chrono = "0.4"
dirs = "6"
```

---

## 11. 구현 순서 (Implementation Order)

| 순서 | 파일 | 의존성 | 설명 |
|------|------|--------|------|
| 1 | `Cargo.toml` | 없음 | 프로젝트 초기화 + 의존성 |
| 2 | `src/chain/types.rs` | 없음 | TxInfo, BlockInfo, TxType 구조체 |
| 3 | `src/event.rs` | types.rs | ChainEvent, ConnectionState enum |
| 4 | `src/config.rs` | 없음 | AppConfig, RpcConfig, UiConfig + TOML 로딩 |
| 5 | `src/filter.rs` | types.rs | TxFilter 구조체 + matches() |
| 6 | `src/chain/provider.rs` | types.rs, event.rs, config.rs | WebSocket 연결, 블록 구독, 재연결 |
| 7 | `src/app.rs` | 모든 모듈 | App 상태, 이벤트 루프, 키 핸들링 |
| 8 | `src/ui/header.rs` | app.rs | 헤더 위젯 |
| 9 | `src/ui/tx_list.rs` | app.rs | 트랜잭션 목록 위젯 |
| 10 | `src/ui/tx_detail.rs` | app.rs | 트랜잭션 상세 위젯 |
| 11 | `src/ui/status_bar.rs` | app.rs | 상태바 위젯 |
| 12 | `src/ui/help.rs` | 없음 | 도움말 바 위젯 |
| 13 | `src/ui/layout.rs` | ui/*.rs | 레이아웃 조합 + render() |
| 14 | `src/ui/mod.rs` | ui/*.rs | UI 모듈 re-export |
| 15 | `src/chain/mod.rs` | chain/*.rs | Chain 모듈 re-export |
| 16 | `src/main.rs` | app.rs, config.rs | CLI 파싱, 앱 실행 |
| 17 | `config/default.toml` | 없음 | 기본 설정 파일 |

---

## 12. 주소 축약 규칙

```rust
/// 주소를 `0xabcd..ef01` 형태로 축약
fn abbreviate_address(addr: &str, len: usize) -> String {
    if addr.len() <= len * 2 + 4 {
        return addr.to_string();
    }
    format!("{}..{}", &addr[..len + 2], &addr[addr.len() - len..])
}

// 예시: abbreviate_address("0x1234567890abcdef", 4) → "0x1234..cdef"
```

---

## 13. 성능 고려사항

| 항목 | 전략 |
|------|------|
| **메모리** | VecDeque 최대 1000개 트랜잭션 유지, 초과 시 오래된 것부터 제거 |
| **렌더링** | 16ms tick (60fps), 화면에 보이는 행만 렌더링 (가상 스크롤) |
| **네트워크** | 블록당 1회 RPC 호출 (getBlockByNumber), ~12초 간격 (ETH 블록타임) |
| **파싱** | serde zero-copy 활용, 불필요한 필드 무시 |

---

## 14. 다음 단계

- [ ] 구현 시작 → `/pdca do chain-eye`
- [ ] 구현 순서 표(Section 11)에 따라 순차 개발
- [ ] 각 Phase 완료 시 빌드 검증 (`cargo build`)
