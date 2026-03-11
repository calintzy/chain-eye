use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::TableState;
use ratatui::Frame;
use tokio::sync::mpsc;

use crate::chain::chains::ChainId;
use crate::chain::types::{BlockInfo, TxInfo};
use crate::config::AppConfig;
use crate::event::{ChainEvent, ConnectionState};
use crate::filter::TxFilter;
use crate::ui;

/// 체인별 상태
pub struct ChainState {
    pub transactions: VecDeque<TxInfo>,
    pub current_block: Option<BlockInfo>,
    pub connection_state: ConnectionState,
    pub tx_total_count: u64,
}

impl ChainState {
    fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
            current_block: None,
            connection_state: ConnectionState::Disconnected {
                reason: "연결 중...".to_string(),
            },
            tx_total_count: 0,
        }
    }
}

/// 지갑 추적 정보
pub struct WatchInfo {
    pub address: String,
    pub ens_name: Option<String>,
    pub balance_eth: Option<f64>,
    pub last_activity: Option<u64>,
    pub tx_count: u64,
}

pub struct App {
    // 멀티체인 상태
    pub chain_states: HashMap<ChainId, ChainState>,
    pub active_chains: Vec<ChainId>,
    pub active_chain_idx: usize,

    // 지갑 추적
    pub watch_info: Option<WatchInfo>,
    pub watch_mode: bool,

    // UI 상태
    pub selected_index: usize,
    pub show_detail: bool,
    pub table_state: TableState,

    // 필터
    pub filter: TxFilter,

    // 설정
    pub config: AppConfig,

    // 통신
    pub chain_rx: mpsc::Receiver<ChainEvent>,

    // 상태
    pub running: bool,
}

impl App {
    pub fn new(
        config: AppConfig,
        filter: TxFilter,
        active_chains: Vec<ChainId>,
        watch_address: Option<String>,
        chain_rx: mpsc::Receiver<ChainEvent>,
    ) -> Self {
        let mut chain_states = HashMap::new();
        for &chain_id in &active_chains {
            chain_states.insert(chain_id, ChainState::new());
        }

        let watch_info = watch_address.map(|addr| WatchInfo {
            address: addr.to_lowercase(),
            ens_name: None,
            balance_eth: None,
            last_activity: None,
            tx_count: 0,
        });

        Self {
            chain_states,
            active_chains,
            active_chain_idx: 0,
            watch_info,
            watch_mode: false,
            selected_index: 0,
            show_detail: false,
            table_state: TableState::default(),
            filter,
            config,
            chain_rx,
            running: true,
        }
    }

    /// 현재 활성 체인 ID
    pub fn active_chain(&self) -> ChainId {
        self.active_chains[self.active_chain_idx]
    }

    /// 현재 활성 체인의 상태
    pub fn active_state(&self) -> &ChainState {
        &self.chain_states[&self.active_chain()]
    }

    /// 메인 이벤트 루프
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();

        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(Duration::from_millis(self.config.ui.tick_rate_ms))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            while let Ok(chain_event) = self.chain_rx.try_recv() {
                self.handle_chain_event(chain_event);
            }
        }

        ratatui::restore();
        Ok(())
    }

    /// UI 렌더링
    fn render(&mut self, frame: &mut Frame) {
        let has_tabs = self.active_chains.len() > 1;
        let layout = ui::layout::build_layout(frame.area(), self.show_detail, has_tabs);

        // 체인 탭 (멀티체인일 때만)
        if let Some(tab_area) = layout.chain_tabs {
            ui::tabs::render(
                frame,
                tab_area,
                &self.active_chains,
                self.active_chain_idx,
                &self.chain_states,
            );
        }

        let chain_id = self.active_chain();
        let state = &self.chain_states[&chain_id];

        // Header
        ui::header::render(
            frame,
            layout.header,
            &state.current_block,
            &state.connection_state,
            chain_id,
        );

        // Transaction List
        let tx_slice: Vec<TxInfo> = state.transactions.iter().cloned().collect();
        let selected_tx = state.transactions.get(self.selected_index).cloned();
        ui::tx_list::render(
            frame,
            layout.tx_list,
            &tx_slice,
            self.selected_index,
            &mut self.table_state,
            chain_id,
        );

        // Detail Panel
        if let Some(detail_area) = layout.tx_detail {
            ui::tx_detail::render(frame, detail_area, selected_tx.as_ref());
        }

        // Status Bar
        let tx_total = self.chain_states[&chain_id].tx_total_count;
        let tx_len = self.chain_states[&chain_id].transactions.len();
        ui::status_bar::render(
            frame,
            layout.status_bar,
            tx_total,
            tx_len,
            &self.filter,
            chain_id,
        );

        // Help Bar
        ui::help::render(frame, layout.help_bar, self.show_detail, has_tabs);
    }

    /// 키보드 입력 처리
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.running = false,

            // 체인 탭 전환
            KeyCode::Tab => {
                if self.active_chains.len() > 1 {
                    self.active_chain_idx =
                        (self.active_chain_idx + 1) % self.active_chains.len();
                    self.selected_index = 0;
                }
            }
            KeyCode::BackTab => {
                if self.active_chains.len() > 1 {
                    if self.active_chain_idx == 0 {
                        self.active_chain_idx = self.active_chains.len() - 1;
                    } else {
                        self.active_chain_idx -= 1;
                    }
                    self.selected_index = 0;
                }
            }

            // 체인 직접 선택 (1-7)
            KeyCode::Char(c @ '1'..='7') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < self.active_chains.len() {
                    self.active_chain_idx = idx;
                    self.selected_index = 0;
                }
            }

            // Watch 모드 토글
            KeyCode::Char('w') => {
                if self.watch_info.is_some() {
                    self.watch_mode = !self.watch_mode;
                }
            }

            // 위로 이동
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }

            // 아래로 이동
            KeyCode::Down | KeyCode::Char('j') => {
                let txs = &self.active_state().transactions;
                if !txs.is_empty() && self.selected_index < txs.len() - 1 {
                    self.selected_index += 1;
                }
            }

            // 상세 보기
            KeyCode::Enter => {
                if !self.active_state().transactions.is_empty() {
                    self.show_detail = true;
                }
            }

            KeyCode::Esc => {
                self.show_detail = false;
            }

            // 필터 토글
            KeyCode::Char('f') => {
                self.filter.toggle();
            }

            // 맨 위로
            KeyCode::Home | KeyCode::Char('g') => {
                self.selected_index = 0;
            }

            // 맨 아래로
            KeyCode::End | KeyCode::Char('G') => {
                let txs = &self.active_state().transactions;
                if !txs.is_empty() {
                    self.selected_index = txs.len() - 1;
                }
            }

            _ => {}
        }
    }

    /// 체인 이벤트 처리
    fn handle_chain_event(&mut self, event: ChainEvent) {
        match event {
            ChainEvent::NewBlock(chain_id, block_info) => {
                if let Some(state) = self.chain_states.get_mut(&chain_id) {
                    state.current_block = Some(block_info);
                }
            }

            ChainEvent::NewTransactions(chain_id, txs) => {
                let max = self.config.ui.max_transactions;
                if let Some(state) = self.chain_states.get_mut(&chain_id) {
                    state.tx_total_count += txs.len() as u64;

                    for tx in txs.into_iter().rev() {
                        // Watch 모드: 추적 주소와 관련된 TX 카운트
                        if let Some(ref mut watch) = self.watch_info {
                            let addr = &watch.address;
                            if tx.from_full.to_lowercase() == *addr
                                || tx
                                    .to_full
                                    .as_ref()
                                    .map(|t| t.to_lowercase() == *addr)
                                    .unwrap_or(false)
                            {
                                watch.tx_count += 1;
                                watch.last_activity = Some(tx.timestamp);
                            }
                        }
                        state.transactions.push_front(tx);
                    }

                    while state.transactions.len() > max {
                        state.transactions.pop_back();
                    }

                    // 현재 활성 체인이면 선택 인덱스 보정
                    let active = self.active_chains[self.active_chain_idx];
                    if chain_id == active
                        && !state.transactions.is_empty()
                        && self.selected_index >= state.transactions.len()
                    {
                        self.selected_index = state.transactions.len() - 1;
                    }
                }
            }

            ChainEvent::ConnectionStatus(chain_id, conn_state) => {
                if let Some(state) = self.chain_states.get_mut(&chain_id) {
                    state.connection_state = conn_state;
                }
            }
        }
    }
}
