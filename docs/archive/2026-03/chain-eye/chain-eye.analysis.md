# Chain-Eye Gap Analysis Report

> **Analysis Type**: Gap Analysis (Design vs Implementation)
>
> **Project**: chain-eye
> **Version**: 0.1.0
> **Analyst**: bkit-gap-detector
> **Date**: 2026-03-11
> **Design Doc**: [chain-eye.design.md](../02-design/features/chain-eye.design.md)
> **Iteration**: 2 (Post Act-1)

---

## 1. Analysis Overview

### 1.1 Analysis Purpose

Design 문서(Section 1~14)와 실제 구현 코드 간의 일치율을 측정하고, Iteration 1에서 수행한 개선 작업의 효과를 검증한다.

### 1.2 Analysis Scope

- **Design Document**: `docs/02-design/features/chain-eye.design.md`
- **Implementation Path**: `src/`, `Cargo.toml`, `config/default.toml`
- **Analysis Date**: 2026-03-11
- **Build Status**: cargo check 통과 (경고 6개, 에러 0개)

### 1.3 Iteration 1 Changes (Act-1)

| # | 변경 사항 | 파일 |
|---|----------|------|
| 1 | ChainEyeError enum 신규 구현 (6개 variant, thiserror) | `src/error.rs` (신규) |
| 2 | `mod error;` 추가 | `src/main.rs` |
| 3 | Exponential Backoff 재연결 로직 구현 | `src/chain/provider.rs` |
| 4 | Design 문서 의존성 섹션 업데이트 (alloy v1.5, futures-util 추가) | `chain-eye.design.md` Section 10 |

---

## 2. Overall Scores

| Category | Prev Score | New Score | Delta | Status |
|----------|:---------:|:---------:|:-----:|:------:|
| 파일 구조 (Section 2.1) | 100% | 100% | -- | [OK] |
| 데이터 구조 (Section 3) | 93% | 93% | -- | [WARN] |
| UI 레이아웃 (Section 4) | 100% | 100% | -- | [OK] |
| WebSocket 연결 (Section 5) | 60% | 95% | +35% | [OK] |
| 키보드 입력 (Section 6) | 100% | 100% | -- | [OK] |
| CLI 인터페이스 (Section 7) | 100% | 100% | -- | [OK] |
| 설정 파일 (Section 8) | 95% | 95% | -- | [OK] |
| 에러 핸들링 (Section 9) | 10% | 70% | +60% | [WARN] |
| 의존성 (Section 10) | 85% | 100% | +15% | [OK] |
| 성능 고려사항 (Section 13) | 75% | 75% | -- | [WARN] |
| **Overall** | **82%** | **93%** | **+11%** | **[OK]** |

---

## 3. Detailed Gap Analysis

### 3.1 파일 구조 (Design Section 2.1) - 100%

Design 문서에 명시된 17개 파일이 모두 존재한다. 추가로 `src/error.rs`가 신규 생성되었다 (Design Section 9에 암시적으로 포함).

| Design 파일 | 구현 파일 | Status |
|-------------|----------|--------|
| `Cargo.toml` | `Cargo.toml` | [OK] Match |
| `src/main.rs` | `src/main.rs` | [OK] Match |
| `src/app.rs` | `src/app.rs` | [OK] Match |
| `src/event.rs` | `src/event.rs` | [OK] Match |
| `src/config.rs` | `src/config.rs` | [OK] Match |
| `src/filter.rs` | `src/filter.rs` | [OK] Match |
| `src/chain/mod.rs` | `src/chain/mod.rs` | [OK] Match |
| `src/chain/provider.rs` | `src/chain/provider.rs` | [OK] Match |
| `src/chain/types.rs` | `src/chain/types.rs` | [OK] Match |
| `src/ui/mod.rs` | `src/ui/mod.rs` | [OK] Match |
| `src/ui/layout.rs` | `src/ui/layout.rs` | [OK] Match |
| `src/ui/header.rs` | `src/ui/header.rs` | [OK] Match |
| `src/ui/tx_list.rs` | `src/ui/tx_list.rs` | [OK] Match |
| `src/ui/tx_detail.rs` | `src/ui/tx_detail.rs` | [OK] Match |
| `src/ui/status_bar.rs` | `src/ui/status_bar.rs` | [OK] Match |
| `src/ui/help.rs` | `src/ui/help.rs` | [OK] Match |
| `config/default.toml` | `config/default.toml` | [OK] Match |
| (Design 9.1 암시) | `src/error.rs` | [OK] Added |

---

### 3.2 데이터 구조 (Design Section 3) - 93%

변경 없음. 이전 분석과 동일.

#### 3.2.1 TxInfo (Section 3.1)

14개 필드 전체 완전 일치. TxType enum (Transfer, ContractCall, ContractCreation) 일치. [OK]

#### 3.2.2 BlockInfo (Section 3.2)

6개 필드 완전 일치. [OK]

#### 3.2.3 ChainEvent / ConnectionState (Section 3.3)

| 항목 | Design | 구현 | Status |
|------|--------|------|--------|
| ChainEvent::NewBlock(BlockInfo) | O | O | [OK] |
| ChainEvent::NewTransactions(Vec\<TxInfo\>) | O | O | [OK] |
| ChainEvent::ConnectionStatus(ConnectionState) | O | O | [OK] |
| ConnectionState::Connected | O | O | [OK] |
| ConnectionState::Reconnecting { attempt: u32 } | O | O | [OK] |
| ConnectionState::Disconnected { reason: String } | O | O | [OK] |

#### 3.2.4 App (Section 3.4)

| 필드 | Design | 구현 | Status |
|------|--------|------|--------|
| transactions: VecDeque\<TxInfo\> | O | O | [OK] |
| current_block: Option\<BlockInfo\> | O | O | [OK] |
| connection_state: ConnectionState | O | O | [OK] |
| selected_index: usize | O | O | [OK] |
| show_detail: bool | O | O | [OK] |
| scroll_offset: usize | O | X (table_state: TableState) | [CHANGED] |
| filter: TxFilter | O | O | [OK] |
| config: AppConfig | O | O | [OK] |
| chain_rx: mpsc::Receiver\<ChainEvent\> | O | O | [OK] |
| running: bool | O | O | [OK] |
| tx_total_count: u64 | O | O | [OK] |

**변경사항**: `scroll_offset: usize`가 `table_state: TableState`로 변경됨. ratatui의 `TableState`가 내부적으로 스크롤 오프셋을 관리하므로, 기능적으로 동등하며 더 관용적(idiomatic)인 구현이다.

#### 3.2.5 TxFilter / AppConfig

Design과 완전 일치. [OK]

---

### 3.3 UI 레이아웃 (Design Section 4) - 100%

변경 없음. 이전 분석과 동일.

| 항목 | Design | 구현 | Status |
|------|--------|------|--------|
| Header: Length(3) | O | O | [OK] |
| Main: Min(10) | Min(10) | Min(5) | [CHANGED] (개선) |
| StatusBar: Length(1) | O | O | [OK] |
| HelpBar: Length(1) | O | O | [OK] |
| 상세 모드 50:50 분할 | O | O | [OK] |

---

### 3.4 WebSocket 연결 (Design Section 5) - 95% [+35%]

**Iteration 1의 핵심 개선 영역.** `src/chain/provider.rs`에 Exponential Backoff 재연결 로직이 구현되었다.

| 항목 | Design | 구현 | Prev | New | Status |
|------|--------|------|:----:|:---:|--------|
| WebSocket 연결 | O | O (alloy ProviderBuilder) | [OK] | [OK] | Match |
| newHeads 구독 | O | O (subscribe_blocks) | [OK] | [OK] | Match |
| 블록 조회 (full=true) | O | O (get_block_by_number().full()) | [OK] | [OK] | Match |
| 트랜잭션 파싱 + 필터 | O | O (parse_transactions()) | [OK] | [OK] | Match |
| ChainEvent 전송 | O | O | [OK] | [OK] | Match |
| fallback URL 시도 | O | O (try_connect! 매크로) | [PARTIAL] | [OK] | Match |
| **Exponential Backoff 재연결** | O | O (reconnect_with_backoff! 매크로) | [MISSING] | [OK] | **NEW** |
| **재연결 시 ConnectionState 이벤트** | O | O (Reconnecting { attempt }) | [MISSING] | [OK] | **NEW** |
| **최대 재연결 시도 제한** | O | O (max_reconnect_attempts 사용) | [MISSING] | [OK] | **NEW** |
| **스트림 끊김 자동 재연결** | O | O (외부 loop + reconnect_with_backoff!) | [MISSING] | [OK] | **NEW** |
| reconnect_monitor_task (별도 태스크) | O | X (chain_provider_task 내 통합) | [MISSING] | [CHANGED] | Integrated |

#### Exponential Backoff 구현 상세 검증

| Design 요구사항 | 구현 (`provider.rs:52-85`) | 일치 여부 |
|----------------|--------------------------|----------|
| 딜레이: `base_delay * 2^(attempt-1)` | `base_delay * 2u64.pow(attempt - 1)` (line 66) | 일치 |
| 최대 딜레이: 30초 캡 | `std::cmp::min(..., 30_000)` (line 66) | 일치 |
| 최대 시도 횟수 | `for attempt in 1..=max_attempts` (line 58) | 일치 |
| fallback URL 순차 시도 | `try_connect!` 매크로 내 fallback 로직 (line 32-45) | 일치 |
| Reconnecting 이벤트 전송 | `ConnectionState::Reconnecting { attempt }` (line 60-63) | 일치 |
| 초과 시 Disconnected 이벤트 | `"최대 재연결 시도 초과"` 메시지 (line 77-80) | 일치 |

#### reconnect_monitor_task 판정

Design Section 1.3에서 `reconnect_monitor_task`를 별도 spawn 태스크로 정의했으나, 구현에서는 `chain_provider_task` 내부의 외부 `loop` (line 157-221)에서 스트림 끊김 감지 + 자동 재연결을 통합 처리한다. 기능적으로 동등하며, 별도 태스크 간 동기화가 불필요하여 더 단순한 구조이다. **[CHANGED] - 기능 동등, 아키텍처 차이.**

---

### 3.5 키보드 입력 (Design Section 6) - 100%

변경 없음. 8개 키 바인딩 전체 일치.

| 키 | Design 동작 | 구현 위치 | Status |
|----|------------|----------|--------|
| Up / k | 이전 트랜잭션 선택 | `app.rs:134` | [OK] |
| Down / j | 다음 트랜잭션 선택 | `app.rs:141` | [OK] |
| Enter | 상세 패널 열기 | `app.rs:150` | [OK] |
| Esc | 상세 패널 닫기 | `app.rs:157` | [OK] |
| f | 필터 토글 | `app.rs:162` | [OK] |
| Home / g | 목록 맨 위로 | `app.rs:167` | [OK] |
| End / G | 목록 맨 아래로 | `app.rs:172` | [OK] |
| q | 앱 종료 | `app.rs:131` | [OK] |

---

### 3.6 CLI 인터페이스 (Design Section 7) - 100%

변경 없음. 5개 CLI 인자 전체 일치.

---

### 3.7 설정 파일 (Design Section 8) - 95%

변경 없음. `address_display_len` 기본값 차이(Design 8 vs 구현 4)만 존재. Design의 축약 예시 `0xabcd..ef01`(len=4)과 구현이 일치하므로 기능적으로 적절.

---

### 3.8 에러 핸들링 (Design Section 9) - 70% [+60%]

**Iteration 1의 핵심 개선 영역.** `src/error.rs`에 `ChainEyeError` enum이 구현되었다.

| 항목 | Design | 구현 | Prev | New | Status |
|------|--------|------|:----:|:---:|--------|
| `ChainEyeError` enum 정의 | O | O (`src/error.rs:3`) | [MISSING] | [OK] | **NEW** |
| `#[derive(thiserror::Error, Debug)]` | O | O (`src/error.rs:2`) | [MISSING] | [OK] | **NEW** |
| `WsConnection(String)` variant | O | O (`src/error.rs:5`) | [MISSING] | [OK] | **NEW** |
| `RpcRequest(String)` variant | O | O (`src/error.rs:8`) | [MISSING] | [OK] | **NEW** |
| `MaxReconnectExceeded(u32)` variant | O | O (`src/error.rs:11`) | [MISSING] | [OK] | **NEW** |
| `ConfigLoad(String)` variant | O | O (`src/error.rs:14`) | [MISSING] | [OK] | **NEW** |
| `TxParse(String)` variant | O | O (`src/error.rs:17`) | [MISSING] | [OK] | **NEW** |
| `UiRender(String)` variant | O | O (`src/error.rs:20`) | [MISSING] | [OK] | **NEW** |
| `mod error;` 선언 | 암시적 | O (`src/main.rs:4`) | [MISSING] | [OK] | **NEW** |
| 에러 타입 실제 사용 (함수 반환값) | O | X (anyhow::Result 계속 사용) | [MISSING] | [MISSING] | Gap |
| 에러 처리 전략 (Section 9.2) 적용 | O | Partial | [MISSING] | [PARTIAL] | Gap |

#### ChainEyeError 에러 메시지 검증

| Variant | Design 메시지 | 구현 메시지 | 일치 |
|---------|-------------|-----------|------|
| WsConnection | "WebSocket 연결 실패: {0}" | "WebSocket 연결 실패: {0}" | 완전 일치 |
| RpcRequest | "RPC 요청 실패: {0}" | "RPC 요청 실패: {0}" | 완전 일치 |
| MaxReconnectExceeded | "최대 재연결 시도 초과 ({0}회)" | "최대 재연결 시도 초과 ({0}회)" | 완전 일치 |
| ConfigLoad | "설정 파일 로딩 실패: {0}" | "설정 파일 로딩 실패: {0}" | 완전 일치 |
| TxParse | "트랜잭션 파싱 실패: {0}" | "트랜잭션 파싱 실패: {0}" | 완전 일치 |
| UiRender | "TUI 렌더링 오류: {0}" | "TUI 렌더링 오류: {0}" | 완전 일치 |

#### 미해결 Gap: ChainEyeError 미사용

`src/error.rs`에 enum이 정의되었으나, 코드 전체에서 `use crate::error::ChainEyeError`가 없다. 모든 함수가 여전히 `anyhow::Result`를 반환하며, `ChainEyeError`를 실제로 생성하거나 매칭하는 코드가 없다. Design Section 9.2의 에러 처리 전략(자동 재연결 시 `WsConnection`, 블록 스킵 시 `RpcRequest` 등)이 `ChainEyeError`를 통해 구조적으로 처리되지 않고, 문자열 기반 에러 메시지로 처리되고 있다.

**70% 산출 근거**: 에러 타입 정의 완료(6/6 variant) = 60%, `mod error` 선언 = 5%, thiserror derive = 5%. 실제 사용 및 에러 전략 적용 미비로 30% 감점.

---

### 3.9 의존성 (Design Section 10) - 100% [+15%]

**Design 문서가 업데이트되어 구현과 완전 일치.**

| 크레이트 | Design 버전 | 구현 버전 | Status |
|---------|------------|----------|--------|
| ratatui | 0.29 | 0.29 | [OK] |
| crossterm | 0.28 | 0.28 | [OK] |
| tokio | 1 (full) | 1 (full) | [OK] |
| alloy | 1.5 (provider-ws, rpc-types) | 1.5 (provider-ws, rpc-types) | [OK] |
| futures-util | 0.3 | 0.3 | [OK] |
| serde | 1 (derive) | 1 (derive) | [OK] |
| serde_json | 1 | 1 | [OK] |
| toml | 0.8 | 0.8 | [OK] |
| clap | 4 (derive) | 4 (derive) | [OK] |
| thiserror | 2 | 2 | [OK] |
| anyhow | 1 | 1 | [OK] |
| chrono | 0.4 | 0.4 | [OK] |
| dirs | 6 | 6 | [OK] |

이전 분석에서 [CHANGED]였던 항목들(alloy 0.9->1.5, tokio-tungstenite 제거, futures-util 추가)이 Design 문서 업데이트로 해소됨.

---

### 3.10 성능 고려사항 (Design Section 13) - 75%

변경 없음. 이전 분석과 동일.

| 항목 | Design 전략 | 구현 상태 | Status |
|------|------------|----------|--------|
| VecDeque 최대 1000개 유지 | O | O (`app.rs:199-201`) | [OK] |
| 16ms tick (60fps) | O | O | [OK] |
| 화면에 보이는 행만 렌더링 (가상 스크롤) | O | 미구현 (전체 tx를 Vec 복사) | [MISSING] |
| 블록당 1회 RPC 호출 | O | O | [OK] |
| serde zero-copy 활용 | O | alloy 자체 파서 사용 | [PARTIAL] |

**[WARN]**: `app.rs:99`에서 `self.transactions.iter().cloned().collect()` 호출 시 전체 트랜잭션을 매 프레임마다 Vec으로 복사한다. 가상 스크롤(화면에 보이는 행만 렌더링) 미구현.

---

## 4. Differences Summary

### 4.1 [FAIL] Missing Features (Design O, Implementation X)

| # | 항목 | Design 위치 | 설명 | Impact | Prev |
|---|------|-------------|------|--------|------|
| 1 | ~~ChainEyeError enum~~ | ~~Section 9.1~~ | ~~6개 variant 미구현~~ | ~~High~~ | **RESOLVED** |
| 2 | ~~Exponential Backoff 재연결~~ | ~~Section 5.2~~ | ~~reconnect_with_backoff() 미구현~~ | ~~High~~ | **RESOLVED** |
| 3 | ChainEyeError 실사용 | Section 9.2 | 에러 타입이 정의만 되고 함수 반환값에 미적용 | Medium | NEW |
| 4 | 가상 스크롤 | Section 13 | 화면에 보이는 행만 렌더링하는 최적화 미구현 | Medium | 유지 |

### 4.2 [INFO] Added Features (Design X, Implementation O)

| # | 항목 | 구현 위치 | 설명 |
|---|------|----------|------|
| 1 | TxType Display trait | `src/chain/types.rs:11-18` | TxType의 문자열 표현 |
| 2 | wei_to_eth() / wei_to_gwei() | `src/chain/types.rs:60-69` | 단위 변환 유틸리티 |
| 3 | TxFilter 헬퍼 (toggle, has_conditions, from_cli) | `src/filter.rs:56-80` | 편의 메서드 |
| 4 | AppConfig::merge() | `src/config.rs:113-126` | 설정 병합 로직 |
| 5 | render_reconnecting() | `src/ui/header.rs:55-80` | Reconnecting 상태 전용 렌더링 |
| 6 | #[command(version)] | `src/main.rs:21` | CLI 버전 표시 |
| 7 | try_connect! 매크로 | `src/chain/provider.rs:15-49` | 연결 시도 로직 캡슐화 |
| 8 | reconnect_with_backoff! 매크로 | `src/chain/provider.rs:52-85` | 재연결 로직 캡슐화 |

### 4.3 [INFO] Changed Features (Design != Implementation)

| # | 항목 | Design | Implementation | Impact |
|---|------|--------|----------------|--------|
| 1 | scroll_offset | `scroll_offset: usize` | `table_state: TableState` | Low (ratatui 관용적 개선) |
| 2 | Main 영역 최소값 | `Min(10)` | `Min(5)` | Low (소형 터미널 지원) |
| 3 | address_display_len 기본값 | 8 | 4 | Low (Design 예시와 일치) |
| 4 | reconnect_monitor_task | 별도 spawn 태스크 | chain_provider_task 내 통합 | Low (기능 동등, 더 단순) |
| 5 | 재연결 구현 형태 | async fn | macro_rules! | Low (Rust 매크로 패턴, 기능 동일) |

---

## 5. Match Rate Calculation

```
Total Design Items: 89

  Matched:          76 items (85.4%)
  Changed (valid):   6 items ( 6.7%) -- 기능적 동등 또는 개선
  Partially impl:    2 items ( 2.2%)
  Missing:           2 items ( 2.2%) -- ChainEyeError 미사용, 가상 스크롤
  Not applicable:    3 items ( 3.4%) -- serde zero-copy (alloy 사용)

Match Rate (Match + Changed):  82 / 89 = 92.1%
Strict Match Rate (Match only): 76 / 89 = 85.4%

Adjusted Match Rate: 93% (가중 평균)
```

### Category Weights

| Category | Weight | Prev Score | New Score | Weighted |
|----------|:------:|:---------:|:---------:|:--------:|
| 파일 구조 | 15% | 100% | 100% | 15.0% |
| 데이터 구조 | 20% | 93% | 93% | 18.6% |
| UI 레이아웃 | 10% | 100% | 100% | 10.0% |
| WebSocket 연결 | 15% | 60% | 95% | 14.25% |
| 키보드 입력 | 10% | 100% | 100% | 10.0% |
| CLI 인터페이스 | 5% | 100% | 100% | 5.0% |
| 설정 파일 | 5% | 95% | 95% | 4.75% |
| 에러 핸들링 | 10% | 10% | 70% | 7.0% |
| 의존성 | 5% | 85% | 100% | 5.0% |
| 성능 | 5% | 75% | 75% | 3.75% |
| **Total** | **100%** | | | **93.35%** |

**Overall Match Rate: 93% (반올림)** -- 이전 82%에서 +11% 상승.

---

## 6. Architecture Compliance

| 모듈 | 의존 대상 | Design 의도 | 실제 의존 | Status |
|------|----------|-------------|----------|--------|
| chain/types | 없음 (std::fmt만) | 독립 타입 정의 | std::fmt | [OK] |
| event | chain::types | 이벤트 타입 | chain::types | [OK] |
| error | 없음 (thiserror만) | 에러 타입 정의 | thiserror | [OK] |
| config | 없음 (외부 크레이트만) | 설정 로딩 | anyhow, serde, std, dirs, toml | [OK] |
| filter | chain::types | 필터 로직 | chain::types | [OK] |
| chain/provider | types, event, config, filter | 네트워크 계층 | alloy, futures_util, tokio, types, config, event, filter | [OK] |
| app | 전체 | 상태 + 루프 | crossterm, ratatui, tokio, types, config, event, filter, ui | [OK] |
| ui/* | app (간접), types, event, filter | UI 렌더링 | ratatui, chain::types, event, filter | [OK] |
| main | app, config, filter, chain::provider, error | 엔트리포인트 | clap, tokio, app, chain, config, filter (error는 mod 선언만) | [OK] |

**Architecture Score: 100%** - 모든 모듈 의존 방향이 Design 의도와 일치.

---

## 7. Convention Compliance

### 7.1 Rust Naming Convention

| Category | Convention | Compliance | Violations |
|----------|-----------|:----------:|------------|
| Structs | PascalCase | 100% | - |
| Enums | PascalCase | 100% | - |
| Functions | snake_case | 100% | - |
| Constants | UPPER_SNAKE_CASE | N/A | 상수가 config 기반 설계 |
| Modules | snake_case | 100% | - |
| Files | snake_case.rs | 100% | - |
| Folders | snake_case | 100% | - |

### 7.2 Import Order

Rust 표준 관행(std -> 외부 크레이트 -> crate 내부) 준수: 100%

### 7.3 Convention Score: 100%

---

## 8. Iteration Comparison

```
                       Iteration 0 (Initial)     Iteration 1 (Act-1)
                       ─────────────────────     ─────────────────────
Overall Match Rate:          82%           ───▶         93%   (+11%)

WebSocket 연결:               60%           ───▶         95%   (+35%)
에러 핸들링:                   10%           ───▶         70%   (+60%)
의존성:                        85%           ───▶        100%   (+15%)

Missing Items:                 4            ───▶          2    (-2)
Critical Gaps:                 3            ───▶          0    (-3)
```

### Resolved Items (Act-1)

| # | 항목 | 이전 | 현재 | 해결 방법 |
|---|------|------|------|----------|
| 1 | ChainEyeError enum | 0% (MISSING) | 100% (MATCH) | `src/error.rs` 신규 생성, 6개 variant 구현 |
| 2 | Exponential Backoff | 0% (MISSING) | 100% (MATCH) | `reconnect_with_backoff!` 매크로 구현 |
| 3 | 재연결 자동화 | 0% (MISSING) | 95% (CHANGED) | 외부 loop + 재연결 매크로 통합 |
| 4 | 의존성 불일치 | 85% (WARN) | 100% (OK) | Design 문서 Section 10 업데이트 |

---

## 9. Recommended Actions

### 9.1 Short-term Actions (Match Rate 향상)

| Priority | Item | Location | Description | Expected Impact |
|----------|------|----------|-------------|-----------------|
| 1 | ChainEyeError 실사용 | `src/chain/provider.rs`, `src/app.rs`, `src/config.rs` | `anyhow::Result` 대신 `Result<T, ChainEyeError>` 사용. 에러 처리 전략(Section 9.2) 적용 | 에러 핸들링 70% -> 95% |
| 2 | 가상 스크롤 구현 | `src/app.rs:99`, `src/ui/tx_list.rs` | 화면에 보이는 행만 슬라이스하여 Table에 전달, `iter().cloned().collect()` 제거 | 성능 75% -> 95% |

### 9.2 Design Document Update Suggested

| Item | Description |
|------|-------------|
| `src/error.rs` 파일 | Section 2.1 프로젝트 구조에 `src/error.rs` 추가 |
| scroll_offset | `table_state: TableState`로 변경 반영 |
| address_display_len | 기본값 8 -> 4로 변경 반영 |
| reconnect_monitor_task | chain_provider_task 내 통합 설계로 변경 반영 |
| Added helper functions | TxFilter::toggle(), from_cli() 등 반영 |

---

## 10. Next Steps

### Match Rate >= 90% 달성 -- Report 생성 가능

현재 Match Rate **93%**로 90% 임계값을 초과하였으므로, 완료 보고서 생성이 가능하다.

- [x] ~~[Act] ChainEyeError enum 구현~~ (Resolved)
- [x] ~~[Act] Exponential Backoff 재연결 전략 구현~~ (Resolved)
- [x] ~~[Act] 스트림 끊김 자동 재연결 구현~~ (Resolved)
- [x] ~~[Design Update] 의존성 목록 업데이트~~ (Resolved)
- [ ] [Optional] ChainEyeError 실사용 적용 (에러 핸들링 70% -> 95%)
- [ ] [Optional] 가상 스크롤 최적화 (성능 75% -> 95%)
- [ ] [Design Update] 변경된 구현 사항 반영 (error.rs, scroll_offset 등)
- [ ] [Report] 완료 보고서 생성 -> `/pdca report chain-eye`

---

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-03-11 | Initial gap analysis (82% match rate) | bkit-gap-detector |
| 2.0 | 2026-03-11 | Iteration 1 re-analysis (93% match rate, +11%) | bkit-gap-detector |
