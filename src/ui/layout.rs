use ratatui::layout::{Constraint, Layout, Rect};

/// 메인 레이아웃 영역
pub struct AppLayout {
    pub header: Rect,
    pub tx_list: Rect,
    pub tx_detail: Option<Rect>,
    pub status_bar: Rect,
    pub help_bar: Rect,
}

/// 레이아웃 계산
pub fn build_layout(area: Rect, show_detail: bool) -> AppLayout {
    // 수직 분할: Header(3) | Main(flex) | StatusBar(1) | Help(1)
    let vertical = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(5),    // Main
        Constraint::Length(1), // Status bar
        Constraint::Length(1), // Help bar
    ])
    .split(area);

    let (tx_list, tx_detail) = if show_detail {
        // Main 영역을 수평 50:50 분할
        let horizontal = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vertical[1]);
        (horizontal[0], Some(horizontal[1]))
    } else {
        (vertical[1], None)
    };

    AppLayout {
        header: vertical[0],
        tx_list,
        tx_detail,
        status_bar: vertical[2],
        help_bar: vertical[3],
    }
}
