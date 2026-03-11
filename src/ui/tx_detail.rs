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
    use crate::token::decode::TokenTransfer;

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

    let native = tx.chain_id.native_symbol();

    let mut lines = vec![
        Line::from(""),
        detail_line("  Chain", tx.chain_id.display_name()),
        detail_line("  Hash", &tx.hash_full),
        detail_line("  Block", &tx.block_number.to_string()),
        detail_line("  Time", &time_str),
        Line::from(""),
        detail_line("  From", &tx.from_full),
        detail_line("  To", &to_display),
        detail_line("  Value", &format!("{:.6} {}", tx.value_eth, native)),
        Line::from(""),
        detail_line("  Gas Price", &format!("{:.2} Gwei", tx.gas_price_gwei)),
        detail_line("  Gas Limit", &format!("{}", tx.gas_used)),
        detail_line("  Type", &format!("{}", tx.tx_type)),
        detail_line("  Input", &format!("{} bytes", tx.input_size)),
    ];

    // 토큰 전송 정보
    if let Some(ref transfer) = tx.token_transfer {
        lines.push(Line::from(""));
        match transfer {
            TokenTransfer::Erc20 {
                symbol,
                amount,
                from,
                to,
                ..
            } => {
                lines.push(detail_line("  Token", &format!("ERC-20 {}", symbol)));
                lines.push(detail_line("  Amount", &format!("{}", amount)));
                lines.push(detail_line("  TkFrom", from));
                lines.push(detail_line("  TkTo", to));
            }
            TokenTransfer::Erc721 {
                token_id,
                from,
                to,
                ..
            } => {
                lines.push(detail_line("  Token", "ERC-721 NFT"));
                lines.push(detail_line("  TokenID", &format!("#{}", token_id)));
                lines.push(detail_line("  TkFrom", from));
                lines.push(detail_line("  TkTo", to));
            }
        }
    }

    lines
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
