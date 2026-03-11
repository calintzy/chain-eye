# Chain-Eye Plan Document

> Feature: chain-eye
> Created: 2026-03-11
> Phase: Plan
> Level: Dynamic

## Executive Summary

| 항목 | 내용 |
|------|------|
| **Feature** | Chain-Eye — EVM 트랜잭션 실시간 모니터링 TUI |
| **Start Date** | 2026-03-11 |
| **Target Duration** | MVP 4주 |

### Value Delivered

| 관점 | 설명 |
|------|------|
| **Problem** | 블록체인 개발자/트레이더가 터미널에서 작업 중 체인 데이터를 보려면 매번 브라우저로 전환해야 하며, 멀티체인 실시간 모니터링 CLI 도구가 부재함 |
| **Solution** | Rust + ratatui 기반 TUI로 터미널에서 멀티체인 EVM 트랜잭션을 실시간 WebSocket 스트림으로 모니터링하고 지갑을 추적하는 도구 |
| **Function UX Effect** | 브라우저 전환 없이 터미널 한 화면에서 실시간 트랜잭션 스트림, 지갑 추적, 체인 전환이 가능해져 개발/트레이딩 워크플로우가 끊기지 않음 |
| **Core Value** | 터미널 네이티브 개발자를 위한 컨텍스트 스위칭 제거 및 멀티체인 통합 실시간 모니터링 |

---

## 1. 기능 개요

### 1.1 배경 및 동기

- 블록체인 트랜잭션 모니터링은 현재 Etherscan 등 웹 브라우저에 의존
- 개발자, 트레이더, 보안 연구자는 터미널에서 작업하지만 체인 데이터 확인 시 컨텍스트 스위칭 필요
- 여러 체인(Ethereum, Polygon, Arbitrum 등)을 동시에 모니터링할 수 있는 CLI/TUI 도구가 없음
- 기존 evmscope(MCP 도구, 23개 도구/7개 체인)의 도메인 지식을 TUI로 확장

### 1.2 핵심 고통점

| # | 고통점 | 심각도 |
|---|--------|--------|
| 1 | 터미널 ↔ 브라우저 반복 컨텍스트 스위칭 | 높음 |
| 2 | 체인별 서로 다른 익스플로러를 개별적으로 열어야 함 | 중간 |
| 3 | 웹 익스플로러는 새로고침 필요, 실시간 스트림 부재 | 높음 |
| 4 | 특정 지갑의 실시간 활동 관찰 어려움 | 중간 |

### 1.3 목표

- Ethereum mainnet 실시간 트랜잭션 스트림을 TUI에서 표시
- 트랜잭션 상세 정보 조회 (선택 시)
- 기본 필터링 (value, from, to 주소)
- 블록 정보 실시간 표시
- 키보드 기반 네비게이션

---

## 2. MVP 범위 정의 (v0.1)

### 2.1 포함 (In Scope)

| 기능 | 설명 | 우선순위 |
|------|------|----------|
| 실시간 트랜잭션 스트림 | Ethereum mainnet WebSocket 구독으로 새 블록의 트랜잭션 표시 | P0 |
| TUI 레이아웃 | ratatui 기반 메인 화면 (트랜잭션 목록, 상태바, 단축키 가이드) | P0 |
| 트랜잭션 상세 보기 | 목록에서 선택 시 hash, from, to, value, gas, input data 등 표시 | P0 |
| 블록 정보 표시 | 현재 블록 번호, 타임스탬프 표시 | P1 |
| 기본 필터 | value 최소값, from/to 주소 필터 | P1 |
| 설정 파일 | `~/.chain-eye/config.toml` RPC 엔드포인트 설정 | P1 |

### 2.2 제외 (Out of Scope — 이후 버전)

| 기능 | 예정 버전 |
|------|----------|
| 멀티체인 지원 (Polygon, Arbitrum 등) | v0.2 |
| 지갑 추적 모드 (`--watch`) | v0.2 |
| ENS 이름 해석 | v0.2 |
| ERC-20/721 토큰 전송 디코딩 | v0.2 |
| 고래 감지 (`--whale`) | v0.3 |
| 멤풀 스트림 (`--mempool`) | v0.3 |
| 필터 DSL | v0.3 |
| 알림 시스템 | v0.3 |
| 컨트랙트 이벤트 디코딩 | v1.0 |
| 가격 통합 (CoinGecko) | v1.0 |
| 트랜잭션 그래프 | v1.0 |
| 내보내기 (CSV, JSON) | v1.0 |

---

## 3. 타겟 사용자

| 사용자 유형 | MVP 관련성 | 사용 시나리오 |
|------------|-----------|-------------|
| 스마트 컨트랙트 개발자 | 높음 | 배포한 컨트랙트 트랜잭션 실시간 모니터링 |
| DeFi 트레이더 | 중간 | 대량 거래 실시간 관찰 (필터 활용) |
| 보안 연구자 | 중간 | 의심스러운 트랜잭션 패턴 실시간 감시 |
| 블록체인 학습자 | 높음 | 실시간 트랜잭션 관찰로 블록체인 이해 |

---

## 4. 기술 스택

| 구성 요소 | 기술 | 선택 이유 |
|----------|------|----------|
| 언어 | **Rust** | 성능, 크로스 플랫폼 바이너리, 메모리 안전성 |
| TUI 프레임워크 | **ratatui** (+ crossterm) | Rust TUI 표준, 활발한 생태계, 풍부한 위젯 |
| 비동기 런타임 | **tokio** | 비동기 WebSocket + HTTP 처리 |
| EVM RPC 클라이언트 | **alloy** | ethers-rs 후속, 최신 EVM RPC 라이브러리 |
| WebSocket | **tokio-tungstenite** | tokio 기반 비동기 WebSocket |
| 직렬화 | **serde + serde_json** | JSON-RPC 요청/응답 처리 |
| 설정 관리 | **toml** (serde) | `config.toml` 파일 파싱 |
| CLI 인자 | **clap** | 커맨드라인 인자 파싱 |

### 4.1 프로젝트 구조 (예상)

```
chain-eye/
├── Cargo.toml
├── src/
│   ├── main.rs              # 엔트리포인트, CLI 파싱
│   ├── app.rs               # 애플리케이션 상태 관리
│   ├── ui/
│   │   ├── mod.rs            # UI 모듈
│   │   ├── layout.rs         # 메인 레이아웃
│   │   ├── tx_list.rs        # 트랜잭션 목록 위젯
│   │   ├── tx_detail.rs      # 트랜잭션 상세 위젯
│   │   └── status_bar.rs     # 상태바 위젯
│   ├── chain/
│   │   ├── mod.rs            # 체인 모듈
│   │   ├── provider.rs       # RPC/WebSocket 프로바이더
│   │   └── types.rs          # 트랜잭션, 블록 타입 정의
│   ├── config.rs             # 설정 파일 로딩
│   └── filter.rs             # 트랜잭션 필터링
├── config/
│   └── default.toml          # 기본 설정
└── README.md
```

---

## 5. RPC 엔드포인트 전략

### 5.1 기본 제공 (무료)

| 체인 | WebSocket 엔드포인트 | 한계 |
|------|---------------------|------|
| Ethereum | Alchemy 무료 tier (wss) | 월 3억 CU, 초당 30 요청 |
| (fallback) | Ankr 공개 WebSocket | Rate limit 있음, 불안정할 수 있음 |

### 5.2 사용자 커스텀

```toml
# ~/.chain-eye/config.toml
[rpc.ethereum]
ws = "wss://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"
```

### 5.3 연결 안정성 전략

- WebSocket 끊김 시 자동 재연결 (exponential backoff)
- 무료 RPC 실패 시 fallback RPC로 자동 전환
- 연결 상태를 상태바에 표시 (Connected / Reconnecting / Disconnected)

---

## 6. 구현 계획

### Phase 1: 프로젝트 초기화 및 TUI 기반 (1주)

| # | 작업 | 산출물 |
|---|------|--------|
| 1.1 | Cargo 프로젝트 생성, 의존성 설정 | `Cargo.toml` |
| 1.2 | ratatui + crossterm 기반 TUI 프레임워크 구축 | `src/ui/layout.rs` |
| 1.3 | 키보드 이벤트 핸들링 (방향키, Tab, Q 종료) | `src/app.rs` |
| 1.4 | 기본 레이아웃 렌더링 (헤더, 메인 영역, 상태바) | `src/ui/` |

### Phase 2: WebSocket 연결 및 데이터 스트림 (1주)

| # | 작업 | 산출물 |
|---|------|--------|
| 2.1 | Ethereum WebSocket 프로바이더 연결 (alloy) | `src/chain/provider.rs` |
| 2.2 | `newHeads` 구독으로 새 블록 감지 | `src/chain/provider.rs` |
| 2.3 | 블록 내 트랜잭션 목록 조회 및 파싱 | `src/chain/types.rs` |
| 2.4 | 연결 끊김 시 자동 재연결 로직 | `src/chain/provider.rs` |

### Phase 3: UI 통합 및 인터랙션 (1주)

| # | 작업 | 산출물 |
|---|------|--------|
| 3.1 | 실시간 트랜잭션 목록 렌더링 (가상 스크롤링) | `src/ui/tx_list.rs` |
| 3.2 | 트랜잭션 선택 시 상세 정보 패널 | `src/ui/tx_detail.rs` |
| 3.3 | 블록 번호/타임스탬프 상태바 표시 | `src/ui/status_bar.rs` |
| 3.4 | 연결 상태 표시 (Connected/Reconnecting) | `src/ui/status_bar.rs` |

### Phase 4: 필터링 및 설정 (1주)

| # | 작업 | 산출물 |
|---|------|--------|
| 4.1 | `config.toml` 로딩 및 파싱 | `src/config.rs` |
| 4.2 | CLI 인자 파싱 (--chain, --filter) | `src/main.rs` |
| 4.3 | 기본 필터 구현 (value 최소값, from/to 주소) | `src/filter.rs` |
| 4.4 | README 작성, 데모 GIF 제작 | `README.md` |

---

## 7. 리스크 및 대응

| 리스크 | 확률 | 영향도 | 대응 전략 |
|--------|------|--------|----------|
| 무료 RPC WebSocket rate limit 초과 | 높음 | 높음 | fallback RPC 로테이션 + 사용자 키 지원 + polling 모드 대안 |
| alloy 라이브러리 breaking change | 중간 | 중간 | Cargo.lock으로 버전 고정, 특정 버전 pinning |
| 대량 트랜잭션 시 TUI 렌더링 지연 | 중간 | 중간 | 가상 스크롤링 + 렌더링 프레임 제한 (60fps 상한) |
| 터미널 호환성 문제 (Windows 등) | 낮음 | 중간 | crossterm 사용으로 크로스 플랫폼 지원, CI에서 주요 터미널 테스트 |
| WebSocket 연결 불안정 | 중간 | 높음 | 자동 재연결 (exponential backoff), 상태바에 연결 상태 표시 |

---

## 8. 경쟁 분석 요약

| 경쟁 도구 | Chain-Eye 차별점 |
|----------|-----------------|
| EVM-trackooor (방치) | 활발한 유지보수 + TUI + 멀티체인 |
| Etherscan (웹) | 터미널 네이티브, 브라우저 불필요, 실시간 WebSocket |
| eth-cli (조회) | 실시간 스트림 (단순 조회가 아닌 모니터링) |
| evmscope (MCP) | 사람용 인터랙티브 UI (AI 에이전트용이 아닌) |

---

## 9. 성공 지표 (현실적 목표)

| 지표 | 3개월 목표 | 6개월 목표 |
|------|-----------|-----------|
| GitHub Stars | 300+ | 800+ |
| 주간 설치 (cargo install) | 200+ | 500+ |
| 지원 체인 | 1 (ETH) | 7 |
| 커뮤니티 피드백/이슈 | 20+ | 50+ |

---

## 10. 다음 단계

- [ ] Design 문서 작성 → `/pdca design chain-eye`
- [ ] 프로젝트 구조 상세 설계 (모듈 간 인터페이스)
- [ ] ratatui 위젯 레이아웃 상세 목업
- [ ] alloy WebSocket 구독 PoC (Proof of Concept)
