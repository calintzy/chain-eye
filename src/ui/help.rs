use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// 도움말 바 렌더링 (기본 모드)
pub fn render(frame: &mut Frame, area: Rect, show_detail: bool, has_tabs: bool) {
    let mut keys = if show_detail {
        vec![
            key_span("[Esc]", "Back"),
            key_span("[↑↓/jk]", "Select"),
            key_span("[f]", "Filter"),
            key_span("[q]", "Quit"),
        ]
    } else {
        vec![
            key_span("[↑↓/jk]", "Select"),
            key_span("[Enter]", "Detail"),
            key_span("[f]", "Filter"),
            key_span("[g/G]", "Top/Bottom"),
            key_span("[q]", "Quit"),
        ]
    };

    if has_tabs {
        keys.insert(0, key_span("[Tab/1-7]", "Chain"));
    }

    let mut spans: Vec<Span> = vec![Span::raw(" ")];
    for (i, (key, desc)) in keys.into_iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default()));
        }
        spans.push(key);
        spans.push(Span::raw(" "));
        spans.push(desc);
    }

    let line = Line::from(spans);
    let widget = Paragraph::new(line).style(Style::default().bg(Color::Rgb(20, 20, 40)));

    frame.render_widget(widget, area);
}

fn key_span(key: &str, desc: &str) -> (Span<'static>, Span<'static>) {
    (
        Span::styled(
            key.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(desc.to_string(), Style::default().fg(Color::White)),
    )
}
