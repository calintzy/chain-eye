use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::WatchInfo;

/// 지갑 추적 패널 렌더링
pub fn render(frame: &mut Frame, area: Rect, watch_info: &WatchInfo) {
    let addr_display = if watch_info.address.len() > 20 {
        format!(
            "{}..{}",
            &watch_info.address[..10],
            &watch_info.address[watch_info.address.len() - 8..]
        )
    } else {
        watch_info.address.clone()
    };

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                "  Address  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&addr_display, Style::default().fg(Color::White)),
        ]),
    ];

    // ENS 이름
    if let Some(ref ens) = watch_info.ens_name {
        lines.push(Line::from(vec![
            Span::styled(
                "  ENS      ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(ens.clone(), Style::default().fg(Color::Green)),
        ]));
    }

    // 잔고
    if let Some(balance) = watch_info.balance_eth {
        lines.push(Line::from(vec![
            Span::styled(
                "  Balance  ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:.4} ETH", balance),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    // TX 카운트
    lines.push(Line::from(vec![
        Span::styled(
            "  TX Count ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}", watch_info.tx_count),
            Style::default().fg(Color::White),
        ),
    ]));

    // 마지막 활동
    if let Some(ts) = watch_info.last_activity {
        use chrono::{TimeZone, Utc};
        let time_str = Utc
            .timestamp_opt(ts as i64, 0)
            .single()
            .map(|dt| {
                let local = dt.with_timezone(&chrono::Local);
                local.format("%H:%M:%S").to_string()
            })
            .unwrap_or_else(|| "--".to_string());

        lines.push(Line::from(vec![
            Span::styled(
                "  Last     ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(time_str, Style::default().fg(Color::DarkGray)),
        ]));
    }

    let widget = Paragraph::new(lines).block(
        Block::default()
            .title(" Watch ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(widget, area);
}
