use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::chain::types::TxInfo;

/// 트랜잭션 상세 패널 렌더링
pub fn render(frame: &mut Frame, area: Rect, tx: Option<&TxInfo>) {
    let content = match tx {
        Some(tx) => build_detail_lines(tx),
        None => vec![Line::from(Span::styled(
            "  Select a transaction to view details",
            Style::default().fg(Color::DarkGray),
        ))],
    };

    let widget = Paragraph::new(content).block(
        Block::default()
            .title(" Detail ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(widget, area);
}

fn build_detail_lines(tx: &TxInfo) -> Vec<Line<'static>> {
    let to_display = tx
        .to_full
        .clone()
        .unwrap_or_else(|| "(Contract Creation)".to_string());

    let time_str = {
        use chrono::{TimeZone, Utc};
        Utc.timestamp_opt(tx.timestamp as i64, 0)
            .single()
            .map(|dt| {
                let local = dt.with_timezone(&chrono::Local);
                local.format("%Y-%m-%d %H:%M:%S").to_string()
            })
            .unwrap_or_else(|| "--".to_string())
    };

    vec![
        Line::from(""),
        detail_line("  Hash", &tx.hash_full),
        detail_line("  Block", &tx.block_number.to_string()),
        detail_line("  Time", &time_str),
        Line::from(""),
        detail_line("  From", &tx.from_full),
        detail_line("  To", &to_display),
        detail_line("  Value", &format!("{:.6} ETH", tx.value_eth)),
        Line::from(""),
        detail_line("  Gas Price", &format!("{:.2} Gwei", tx.gas_price_gwei)),
        detail_line("  Gas Limit", &format!("{}", tx.gas_used)),
        detail_line("  Type", &format!("{}", tx.tx_type)),
        detail_line("  Input", &format!("{} bytes", tx.input_size)),
    ]
}

fn detail_line(label: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:<12}", label),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(value.to_string(), Style::default().fg(Color::White)),
    ])
}
