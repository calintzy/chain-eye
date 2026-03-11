use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Row, Table, TableState};
use ratatui::Frame;

use crate::chain::types::TxInfo;

/// 트랜잭션 목록 테이블 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    transactions: &[TxInfo],
    selected_index: usize,
    table_state: &mut TableState,
) {
    let header = Row::new(vec!["Time", "Hash", "From", "To", "Value (ETH)", "Type"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(0);

    let rows: Vec<Row> = transactions
        .iter()
        .map(|tx| {
            let time = format_timestamp(tx.timestamp);
            let to_display = tx.to.as_deref().unwrap_or("(create)");
            let value_str = if tx.value_eth >= 0.01 {
                format!("{:.4}", tx.value_eth)
            } else if tx.value_eth > 0.0 {
                format!("{:.8}", tx.value_eth)
            } else {
                "0".to_string()
            };

            let type_str = format!("{}", tx.tx_type);

            Row::new(vec![
                time,
                tx.hash.clone(),
                tx.from.clone(),
                to_display.to_string(),
                value_str,
                type_str,
            ])
        })
        .collect();

    let widths = [
        ratatui::layout::Constraint::Length(8),  // Time
        ratatui::layout::Constraint::Length(13), // Hash
        ratatui::layout::Constraint::Length(13), // From
        ratatui::layout::Constraint::Length(13), // To
        ratatui::layout::Constraint::Length(14), // Value
        ratatui::layout::Constraint::Min(10),    // Type
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(" Transactions ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    table_state.select(Some(selected_index));
    frame.render_stateful_widget(table, area, table_state);
}

/// Unix 타임스탬프를 HH:MM:SS로 변환
fn format_timestamp(ts: u64) -> String {
    use chrono::{TimeZone, Utc};
    let dt = Utc.timestamp_opt(ts as i64, 0).single();
    match dt {
        Some(dt) => {
            let local = dt.with_timezone(&chrono::Local);
            local.format("%H:%M:%S").to_string()
        }
        None => "--:--:--".to_string(),
    }
}
