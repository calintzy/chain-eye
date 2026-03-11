use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Row, Table, TableState};
use ratatui::Frame;

use crate::chain::chains::ChainId;
use crate::chain::types::TxInfo;
use crate::token::decode::TokenTransfer;

/// 트랜잭션 목록 테이블 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    transactions: &[TxInfo],
    selected_index: usize,
    table_state: &mut TableState,
    chain_id: ChainId,
) {
    let native = chain_id.native_symbol();
    let value_header = format!("Value ({})", native);

    let header = Row::new(vec!["Time", "Hash", "From", "To", &value_header, "Type/Token"])
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

            // 토큰 전송이면 토큰 정보 표시, 아니면 TX 타입
            let type_str = match &tx.token_transfer {
                Some(TokenTransfer::Erc20 { symbol, amount, .. }) => {
                    format!("{} {}", format_token_amount(*amount), symbol)
                }
                Some(TokenTransfer::Erc721 { token_id, .. }) => {
                    format!("NFT #{}", token_id)
                }
                None => format!("{}", tx.tx_type),
            };

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
        ratatui::layout::Constraint::Min(10),    // Type/Token
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

/// 토큰 수량을 읽기 쉽게 포맷
fn format_token_amount(val: f64) -> String {
    if val >= 1_000_000.0 {
        format!("{:.1}M", val / 1_000_000.0)
    } else if val >= 1_000.0 {
        format!("{:.1}K", val / 1_000.0)
    } else if val >= 1.0 {
        format!("{:.2}", val)
    } else if val > 0.0 {
        format!("{:.6}", val)
    } else {
        "0".to_string()
    }
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
