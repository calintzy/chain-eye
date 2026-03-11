use std::collections::VecDeque;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::TableState;
use ratatui::Frame;
use tokio::sync::mpsc;

use crate::chain::types::{BlockInfo, TxInfo};
use crate::config::AppConfig;
use crate::event::{ChainEvent, ConnectionState};
use crate::filter::TxFilter;
use crate::ui;

pub struct App {
    // 데이터
    pub transactions: VecDeque<TxInfo>,
    pub current_block: Option<BlockInfo>,
    pub connection_state: ConnectionState,

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
    pub tx_total_count: u64,
}

impl App {
    pub fn new(config: AppConfig, filter: TxFilter, chain_rx: mpsc::Receiver<ChainEvent>) -> Self {
        Self {
            transactions: VecDeque::new(),
            current_block: None,
            connection_state: ConnectionState::Disconnected {
                reason: "Starting...".to_string(),
            },
            selected_index: 0,
            show_detail: false,
            table_state: TableState::default(),
            filter,
            config,
            chain_rx,
            running: true,
            tx_total_count: 0,
        }
    }

    /// 메인 이벤트 루프
    pub async fn run(&mut self) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();

        while self.running {
            // 1. UI 렌더링
            terminal.draw(|frame| self.render(frame))?;

            // 2. 키보드 이벤트 (non-blocking poll)
            if event::poll(Duration::from_millis(self.config.ui.tick_rate_ms))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            // 3. 체인 이벤트 수신 (non-blocking)
            while let Ok(chain_event) = self.chain_rx.try_recv() {
                self.handle_chain_event(chain_event);
            }
        }

        ratatui::restore();
        Ok(())
    }

    /// UI 렌더링
    fn render(&mut self, frame: &mut Frame) {
        let layout = ui::layout::build_layout(frame.area(), self.show_detail);

        // Header
        ui::header::render(
            frame,
            layout.header,
            &self.current_block,
            &self.connection_state,
        );

        // Transaction List
        let tx_slice: Vec<TxInfo> = self.transactions.iter().cloned().collect();
        ui::tx_list::render(
            frame,
            layout.tx_list,
            &tx_slice,
            self.selected_index,
            &mut self.table_state,
        );

        // Detail Panel (상세 모드일 때만)
        if let Some(detail_area) = layout.tx_detail {
            let selected_tx = self.transactions.get(self.selected_index);
            ui::tx_detail::render(frame, detail_area, selected_tx);
        }

        // Status Bar
        ui::status_bar::render(
            frame,
            layout.status_bar,
            self.tx_total_count,
            self.transactions.len(),
            &self.filter,
        );

        // Help Bar
        ui::help::render(frame, layout.help_bar, self.show_detail);
    }

    /// 키보드 입력 처리
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            // 종료
            KeyCode::Char('q') => self.running = false,

            // 위로 이동
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }

            // 아래로 이동
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.transactions.is_empty()
                    && self.selected_index < self.transactions.len() - 1
                {
                    self.selected_index += 1;
                }
            }

            // 상세 보기 열기
            KeyCode::Enter => {
                if !self.transactions.is_empty() {
                    self.show_detail = true;
                }
            }

            // 상세 보기 닫기
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
                if !self.transactions.is_empty() {
                    self.selected_index = self.transactions.len() - 1;
                }
            }

            _ => {}
        }
    }

    /// 체인 이벤트 처리
    fn handle_chain_event(&mut self, event: ChainEvent) {
        match event {
            ChainEvent::NewBlock(block_info) => {
                self.current_block = Some(block_info);
            }

            ChainEvent::NewTransactions(txs) => {
                let max = self.config.ui.max_transactions;
                self.tx_total_count += txs.len() as u64;

                // 새 트랜잭션을 앞에 추가 (최신이 위)
                for tx in txs.into_iter().rev() {
                    self.transactions.push_front(tx);
                }

                // 최대 수 초과 시 오래된 것 제거
                while self.transactions.len() > max {
                    self.transactions.pop_back();
                }

                // 선택 인덱스 범위 보정
                if !self.transactions.is_empty()
                    && self.selected_index >= self.transactions.len()
                {
                    self.selected_index = self.transactions.len() - 1;
                }
            }

            ChainEvent::ConnectionStatus(state) => {
                self.connection_state = state;
            }
        }
    }
}
