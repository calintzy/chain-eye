[English](README.md) | [한국어](README.ko.md) | [中文](README.zh.md)

# Chain-Eye

**EVM 블록체인 트랜잭션을 터미널에서 실시간 모니터링하는 TUI**

```
┌─ Chain-Eye v0.1 ─────────────────────────────────────────────┐
│  ETH Mainnet  Block: 21,847,234  Connected  🟢               │
├──────────────────────────────────────────────────────────────┤
│  Time      Hash          From          To           Value    │
│▶ 12:45:03 0xab3f..8e2d 0x7c2a..1f4b 0xdef9..3c7a 2.50 ETH  │
│  12:45:02 0x9b1e..5d8f 0x4a2c..9e1d 0xab3f..8e2d 0.00 ETH  │
│  12:45:01 0xdef9..3c7a 0x2b1d..4c3e 0x5e6f..7a8b 1.25 ETH  │
│  12:44:59 0x4a2c..9e1d 0x9b1e..5d8f 0x2b1d..4c3e 0.50 ETH  │
├──────────────────────────────────────────────────────────────┤
│  TX: 1,234 received | Filter: OFF                            │
├──────────────────────────────────────────────────────────────┤
│  [↑↓] Select  [Enter] Detail  [F] Filter  [Q] Quit           │
└──────────────────────────────────────────────────────────────┘
```

---

## 왜 Chain-Eye가 필요한가?

현대의 블록체인 개발자, 트레이더, 보안 연구자들은 다음과 같은 문제에 직면합니다:

### 1. 터미널 ↔ 브라우저 컨텍스트 스위칭
코딩이나 데이터 분석 중에 트랜잭션을 확인하려면 매번 브라우저를 열어야 합니다. 개발 흐름이 끊기고 생산성이 떨어집니다.

### 2. 실시간 데이터 부족
Etherscan 같은 웹 익스플로러는 정적이며 새로고침이 필요합니다. 실시간 트랜잭션 스트림을 볼 수 없습니다.

### 3. 필터링과 모니터링의 어려움
특정 지갑, 주소, 최소 거래액을 기준으로 트랜잭션을 실시간으로 필터링할 수 있는 도구가 없습니다.

### 해결책
**Chain-Eye**는 터미널 TUI에서 다음을 제공합니다:
- **실시간 트랜잭션 스트림**: WebSocket을 통한 블록 구독으로 즉시 업데이트
- **강력한 필터링**: 최소 ETH 값, 발신/수신 주소로 트랜잭션 필터링
- **빠른 상세 보기**: Enter 키로 트랜잭션 상세 정보 확인
- **자동 재연결**: Exponential Backoff로 안정적인 연결 유지
- **터미널 중심**: 개발 환경을 벗어나지 않고 체인 데이터 모니터링

---

## 기능

### 핵심 기능
- **실시간 트랜잭션 스트림**: ETH Mainnet의 모든 트랜잭션을 WebSocket으로 실시간 수신
- **트랜잭션 상세 보기**: Enter 키로 거래 상세 정보(Hash, From, To, Value, Gas 등) 표시
- **필터링**: 최소 ETH 값, from 주소, to 주소로 트랜잭션 필터링
- **트랜잭션 분류**: Transfer, ContractCall, ContractCreation으로 거래 유형 구분
- **블록 정보**: 현재 블록 번호와 연결 상태 표시
- **키보드 네비게이션**:
  - Vim 스타일 j/k 지원
  - g로 맨 위로, G로 맨 아래로 이동
- **TOML 설정 파일**: `~/.chain-eye/config.toml`으로 RPC, UI 설정 관리
- **CLI 옵션**: --rpc, --min-value, --from, --to, --config로 런타임 설정 가능
- **자동 재연결**: Exponential Backoff로 연결 끊김 시 자동 복구

---

## 설치 및 실행

### 사전 요구사항
- Rust 1.70 이상

### 빌드
```bash
git clone https://github.com/yourusername/chain-eye.git
cd chain-eye
cargo build --release
```

### 실행
```bash
# 기본 실행 (Ethereum Mainnet, 무료 공개 RPC)
./target/release/chain-eye

# 또는 cargo로 직접 실행
cargo run --release
```

---

## 사용 방법

### 기본 실행
```bash
chain-eye
```
무료 공개 RPC `wss://eth.drpc.org`를 사용하여 Ethereum Mainnet의 실시간 트랜잭션을 모니터링합니다.

### CLI 옵션

#### 커스텀 RPC 지정
```bash
chain-eye --rpc wss://your-custom-rpc-url
```

#### 최소 ETH 값 필터
```bash
chain-eye --min-value 1.0
```
1 ETH 이상의 거래만 표시합니다.

#### 특정 주소로 필터링 (발신)
```bash
chain-eye --from 0x1234567890123456789012345678901234567890
```
지정된 주소에서 출발한 거래만 표시합니다.

#### 특정 주소로 필터링 (수신)
```bash
chain-eye --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
지정된 주소로 수신된 거래만 표시합니다.

#### 복합 필터
```bash
chain-eye --min-value 10.0 --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
10 ETH 이상이고 특정 주소로 수신된 거래만 표시합니다.

#### 설정 파일 지정
```bash
chain-eye --config /path/to/config.toml
```

---

## 키보드 단축키

| 키 | 동작 |
|----|------|
| **↑** 또는 **k** | 이전 트랜잭션 선택 |
| **↓** 또는 **j** | 다음 트랜잭션 선택 |
| **Enter** | 선택한 트랜잭션 상세 보기 |
| **Esc** | 상세 보기 닫기 |
| **f** | 필터 활성/비활성 토글 |
| **g** | 목록 맨 위로 이동 |
| **G** | 목록 맨 아래로 이동 |
| **q** | 애플리케이션 종료 |

---

## 설정 파일

`~/.chain-eye/config.toml`에 설정을 저장하면 기본값으로 사용됩니다.

### 설정 예시
```toml
[rpc]
# 주 RPC URL (기본값: wss://eth.drpc.org)
ws_url = "wss://eth.drpc.org"

# Fallback RPC URL (주 RPC 실패 시 사용)
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"

# 최대 재연결 시도 횟수 (기본값: 10)
max_reconnect_attempts = 10

# 재연결 기본 지연 시간 (밀리초, 기본값: 1000)
reconnect_base_delay_ms = 1000

[ui]
# 메모리에 보관할 최대 트랜잭션 수 (기본값: 1000)
max_transactions = 1000

# UI 렌더링 틱 레이트 (밀리초, 기본값: 16 ≈ 60fps)
tick_rate_ms = 16

# 주소 표시 길이 (0x 포함, 기본값: 4 → 0xab3f)
address_display_len = 4
```

### 기본값
- **ws_url**: `wss://eth.drpc.org`
- **fallback_ws_url**: `wss://ethereum-rpc.publicnode.com`
- **max_reconnect_attempts**: 10
- **reconnect_base_delay_ms**: 1000
- **max_transactions**: 1000
- **tick_rate_ms**: 16
- **address_display_len**: 4

---

## 기술 스택

- **언어**: Rust (2021 edition)
- **TUI 프레임워크**: ratatui 0.29
- **터미널 백엔드**: crossterm 0.28
- **EVM RPC 클라이언트**: alloy 1.5 (ethers-rs 후속)
- **비동기 런타임**: tokio 1.x
- **CLI 파서**: clap 4
- **설정 관리**: serde + toml 0.8
- **직렬화**: serde_json 1
- **시간 처리**: chrono 0.4
- **홈 디렉토리**: dirs 6
- **에러 핸들링**: thiserror 2, anyhow 1
- **비동기 유틸**: futures-util 0.3

---

## 프로젝트 구조

```
chain-eye/
├── src/
│   ├── main.rs              # 진입점, CLI 파싱, 초기화
│   ├── app.rs               # 메인 애플리케이션 루프, 이벤트 핸들링
│   ├── config.rs            # TOML 설정 로딩 및 기본값
│   ├── error.rs             # 커스텀 에러 타입
│   ├── event.rs             # 체인 이벤트 정의 (블록, 트랜잭션)
│   ├── filter.rs            # 트랜잭션 필터링 로직
│   ├── chain/
│   │   ├── mod.rs           # 체인 모듈 진입점
│   │   ├── provider.rs      # WebSocket 프로바이더, 블록 구독
│   │   └── types.rs         # TxInfo, BlockInfo, TxType 정의
│   └── ui/
│       ├── mod.rs           # UI 모듈 진입점
│       ├── header.rs        # 헤더 위젯 (블록 번호, 연결 상태)
│       ├── tx_list.rs       # 트랜잭션 목록 테이블
│       ├── tx_detail.rs     # 트랜잭션 상세 보기 팝업
│       ├── status_bar.rs    # 상태 바 (통계, 필터 상태)
│       ├── help.rs          # 도움말 섹션
│       └── layout.rs        # 레이아웃 조합
├── config/
│   └── default.toml         # 기본 설정 파일 예시
├── Cargo.toml               # 프로젝트 메타데이터 및 의존성
├── Cargo.lock               # 의존성 버전 잠금
└── README.md                # 이 파일
```

---

## 작동 원리

### 1. 초기화
1. CLI 옵션 파싱
2. `~/.chain-eye/config.toml` 로드 (있는 경우)
3. CLI 옵션으로 설정 오버라이드
4. TxFilter 생성

### 2. WebSocket 연결
1. 주 RPC URL(`ws_url`)로 연결 시도
2. 실패 시 fallback RPC 시도
3. 성공하면 WebSocket 연결 유지
4. 실패 시 Exponential Backoff로 재연결

### 3. 실시간 스트림
1. WebSocket으로 블록 헤더 구독 (`subscribe("newHeads")`)
2. 각 블록마다 `eth_getBlockByNumber` 호출로 트랜잭션 목록 조회
3. 각 트랜잭션에서:
   - `from`, `to`, `value`, `data` 파싱
   - `TxType` 분류 (Transfer, ContractCall, ContractCreation)
   - `value`를 ETH로 변환
4. `ChainEvent::NewTransaction` 발행

### 4. UI 렌더링
1. 매 프레임 (기본 16ms) 화면 재렌더링
2. 트랜잭션 목록 표시
3. 선택된 항목 하이라이트
4. 필터 상태 표시

### 5. 이벤트 처리
- **키 이벤트**: 상하 이동, Enter, Esc, f, g, G, q 등 처리
- **체인 이벤트**: 새 트랜잭션, 블록 정보 업데이트

---

## 트랜잭션 타입 분류

| 타입 | 조건 | 예시 |
|------|------|------|
| **Transfer** | `to`가 있고 `input` 데이터 없음 | 단순 ETH 전송 |
| **ContractCall** | `to`가 있고 `input` 데이터 있음 | Uniswap 스왑, 토큰 전송 |
| **ContractCreation** | `to`가 없음 (null) | 새 컨트랙트 배포 |

---

## 재연결 메커니즘

WebSocket 연결이 끊어질 경우 Exponential Backoff로 자동 재연결합니다.

```
시도 1: 1초 대기
시도 2: 2초 대기
시도 3: 4초 대기
시도 4: 8초 대기
...
최대 시도: max_reconnect_attempts 횟수
```

재연결 성공 시 자동으로 다시 블록 구독을 시작합니다.

---

## 문제 해결

### RPC 연결 실패
- **증상**: `Disconnected: Connection failed` 메시지
- **해결**:
  - `--rpc` 옵션으로 다른 RPC 서버 지정
  - `~/.chain-eye/config.toml`의 `fallback_ws_url` 확인
  - 네트워크 연결 확인

### 트랜잭션이 표시되지 않음
- **증상**: 목록이 비어있음
- **해결**:
  - 필터가 너무 강하지 않은지 확인 (f 키로 토글)
  - RPC 연결 상태 확인 (헤더의 연결 상태 아이콘 🟢/🔴)
  - 블록이 수신 중인지 확인

### 높은 CPU 사용량
- **증상**: 팬이 시끄럽고 CPU 사용률 높음
- **해결**:
  - `tick_rate_ms` 증가 (기본 16 → 50)
  - `max_transactions` 감소 (기본 1000 → 500)

---

## 라이선스

MIT License

---

## 기여

기여는 환영합니다. Pull Request를 보내주세요.

---

## 로드맵

- **v0.1** (현재): Ethereum Mainnet 실시간 스트림
- **v0.2**: 멀티체인 지원 (Polygon, Arbitrum, Base 등)
- **v0.3**: 고급 필터링, 지갑 추적 모드
- **v1.0**: 컨트랙트 이벤트 디코딩, 실시간 가격 통합

---

## 문의

이슈나 질문은 GitHub Issues에서 보내주세요.
