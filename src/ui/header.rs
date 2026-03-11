use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::chain::chains::ChainId;
use crate::chain::types::BlockInfo;
use crate::event::ConnectionState;

/// 헤더 위젯 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    current_block: &Option<BlockInfo>,
    connection_state: &ConnectionState,
    chain_id: ChainId,
) {
    let block_text = match current_block {
        Some(info) => format!("Block: {}", info.number),
        None => "Block: --".to_string(),
    };

    let gas_text = current_block
        .as_ref()
        .and_then(|info| info.base_fee_gwei.map(|fee| format!("Gas: {:.1} Gwei", fee)))
        .unwrap_or_default();

    let chain_name = chain_id.display_name();
    let chain_color = chain_id.color();

    let (conn_text, conn_color) = match connection_state {
        ConnectionState::Connected => ("Connected".to_string(), Color::Green),
        ConnectionState::Reconnecting { attempt } => {
            (format!("Reconnecting ({})", attempt), Color::Yellow)
        }
        ConnectionState::Disconnected { .. } => ("Disconnected".to_string(), Color::Red),
    };

    let mut spans = vec![
        Span::styled(
            " Chain-Eye v0.2 ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(chain_name, Style::default().fg(chain_color).add_modifier(Modifier::BOLD)),
        Span::raw("    "),
        Span::styled(&block_text, Style::default().fg(Color::Yellow)),
    ];

    if !gas_text.is_empty() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(gas_text, Style::default().fg(Color::Magenta)));
    }

    spans.push(Span::raw("    "));
    spans.push(Span::styled(conn_text, Style::default().fg(conn_color)));

    let line = Line::from(spans);

    let widget = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(widget, area);
}
