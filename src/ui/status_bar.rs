use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::filter::TxFilter;

/// 상태바 렌더링
pub fn render(
    frame: &mut Frame,
    area: Rect,
    tx_total: u64,
    tx_displayed: usize,
    filter: &TxFilter,
) {
    let filter_text = if filter.active && filter.has_conditions() {
        let mut parts = Vec::new();
        if let Some(min) = filter.min_value_eth {
            parts.push(format!("min:{:.2}ETH", min));
        }
        if filter.from_address.is_some() {
            parts.push("from:set".to_string());
        }
        if filter.to_address.is_some() {
            parts.push("to:set".to_string());
        }
        format!("Filter: ON ({})", parts.join(", "))
    } else {
        "Filter: OFF".to_string()
    };

    let line = Line::from(vec![
        Span::styled(
            format!(" TX: {} received", tx_total),
            Style::default().fg(Color::White),
        ),
        Span::styled(
            format!(" | Showing: {} ", tx_displayed),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled("| ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            filter_text,
            Style::default().fg(if filter.active {
                Color::Green
            } else {
                Color::DarkGray
            }),
        ),
    ]);

    let widget = Paragraph::new(line).style(Style::default().bg(Color::Rgb(30, 30, 30)));

    frame.render_widget(widget, area);
}
