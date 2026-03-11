use ratatui::layout::{Constraint, Layout, Rect};

/// 메인 레이아웃 영역
pub struct AppLayout {
    pub chain_tabs: Option<Rect>,
    pub header: Rect,
    pub tx_list: Rect,
    pub tx_detail: Option<Rect>,
    pub status_bar: Rect,
    pub help_bar: Rect,
}

/// 레이아웃 계산
pub fn build_layout(area: Rect, show_detail: bool, has_tabs: bool) -> AppLayout {
    // 수직 분할: [Tabs(1)] | Header(3) | Main(flex) | StatusBar(1) | Help(1)
    let mut constraints = Vec::new();
    if has_tabs {
        constraints.push(Constraint::Length(1)); // Chain tabs
    }
    constraints.push(Constraint::Length(3));  // Header
    constraints.push(Constraint::Min(5));     // Main
    constraints.push(Constraint::Length(1));   // Status bar
    constraints.push(Constraint::Length(1));   // Help bar

    let vertical = Layout::vertical(constraints).split(area);

    let (chain_tabs, header_idx) = if has_tabs {
        (Some(vertical[0]), 1)
    } else {
        (None, 0)
    };

    let main_idx = header_idx + 1;
    let status_idx = header_idx + 2;
    let help_idx = header_idx + 3;

    let (tx_list, tx_detail) = if show_detail {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(vertical[main_idx]);
        (horizontal[0], Some(horizontal[1]))
    } else {
        (vertical[main_idx], None)
    };

    AppLayout {
        chain_tabs,
        header: vertical[header_idx],
        tx_list,
        tx_detail,
        status_bar: vertical[status_idx],
        help_bar: vertical[help_idx],
    }
}
