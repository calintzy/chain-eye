use std::collections::HashMap;

use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Tabs;
use ratatui::Frame;

use crate::app::ChainState;
use crate::chain::chains::ChainId;
use crate::event::ConnectionState;

/// 체인 탭 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    active_chains: &[ChainId],
    active_idx: usize,
    chain_states: &HashMap<ChainId, ChainState>,
) {
    let titles: Vec<Line> = active_chains
        .iter()
        .enumerate()
        .map(|(i, chain_id)| {
            let name = chain_id.display_name();
            let color = chain_id.color();

            // 연결 상태 아이콘
            let status_icon = chain_states
                .get(chain_id)
                .map(|s| match &s.connection_state {
                    ConnectionState::Connected => "●",
                    ConnectionState::Reconnecting { .. } => "◌",
                    ConnectionState::Disconnected { .. } => "○",
                })
                .unwrap_or("○");

            let label = format!(" {} {} {} ", i + 1, name, status_icon);

            if i == active_idx {
                Line::from(Span::styled(
                    label,
                    Style::default()
                        .fg(color)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                ))
            } else {
                Line::from(Span::styled(
                    label,
                    Style::default().fg(color),
                ))
            }
        })
        .collect();

    let tabs = Tabs::new(titles)
        .select(active_idx)
        .style(Style::default().fg(Color::DarkGray))
        .divider(Span::raw("│"));

    frame.render_widget(tabs, area);
}
