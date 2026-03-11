use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::chain::types::BlockInfo;
use crate::event::ConnectionState;

/// 헤더 위젯 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    current_block: &Option<BlockInfo>,
    connection_state: &ConnectionState,
) {
    let block_text = match current_block {
        Some(info) => format!("Block: {}", info.number),
        None => "Block: --".to_string(),
    };

    let (conn_text, conn_color) = match connection_state {
        ConnectionState::Connected => ("Connected", Color::Green),
        ConnectionState::Reconnecting { attempt } => {
            // 직접 문자열 반환 대신 static text 사용
            return render_reconnecting(frame, area, &block_text, *attempt);
        }
        ConnectionState::Disconnected { .. } => ("Disconnected", Color::Red),
    };

    let line = Line::from(vec![
        Span::styled(
            " Chain-Eye v0.1 ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("ETH Mainnet", Style::default().fg(Color::White)),
        Span::raw("    "),
        Span::styled(&block_text, Style::default().fg(Color::Yellow)),
        Span::raw("    "),
        Span::styled(conn_text, Style::default().fg(conn_color)),
    ]);

    let widget = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(widget, area);
}

fn render_reconnecting(frame: &mut Frame, area: Rect, block_text: &str, attempt: u32) {
    let conn_text = format!("Reconnecting ({})", attempt);

    let line = Line::from(vec![
        Span::styled(
            " Chain-Eye v0.1 ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("ETH Mainnet", Style::default().fg(Color::White)),
        Span::raw("    "),
        Span::styled(block_text, Style::default().fg(Color::Yellow)),
        Span::raw("    "),
        Span::styled(conn_text, Style::default().fg(Color::Yellow)),
    ]);

    let widget = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(widget, area);
}
