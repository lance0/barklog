use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
};

use crate::app::{AppState, FocusedPanel, InputMode};

const SIDE_PANEL_WIDTH: u16 = 24;

/// Draw the entire UI
pub fn draw(frame: &mut Frame, state: &mut AppState) {
    // Main layout: optional side panel + main content
    let main_chunks = if state.show_side_panel {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(SIDE_PANEL_WIDTH),
                Constraint::Min(20),
            ])
            .split(frame.area())
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20)])
            .split(frame.area())
    };

    // Draw side panel if visible
    if state.show_side_panel {
        draw_side_panel(frame, state, main_chunks[0]);
    }

    // Main content area
    let content_area = if state.show_side_panel { main_chunks[1] } else { main_chunks[0] };

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // Header
            Constraint::Min(3),     // Log view
            Constraint::Length(1),  // Status bar
            Constraint::Length(1),  // Filter bar
        ])
        .split(content_area);

    draw_header(frame, state, content_chunks[0]);
    draw_log_view(frame, state, content_chunks[1]);
    draw_status_bar(frame, state, content_chunks[2]);
    draw_filter_bar(frame, state, content_chunks[3]);

    // Draw help overlay if active
    if state.show_help {
        draw_help_overlay(frame);
    }
}

/// Draw the side panel with sources and saved filters
fn draw_side_panel(frame: &mut Frame, state: &AppState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(state.sources.len() as u16 + 2),  // Sources section
            Constraint::Min(3),  // Filters section
        ])
        .split(area);

    draw_sources_panel(frame, state, chunks[0]);
    draw_filters_panel(frame, state, chunks[1]);
}

/// Draw the sources list
fn draw_sources_panel(frame: &mut Frame, state: &AppState, area: Rect) {
    let focused = state.focused_panel == FocusedPanel::Sources;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Sources ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = state.sources
        .iter()
        .enumerate()
        .map(|(i, source)| {
            let prefix = if i == state.current_source_idx { "▶ " } else { "  " };
            let style = if i == state.current_source_idx {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(format!("{}{}", prefix, source.name())).style(style)
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

/// Draw the saved filters list
fn draw_filters_panel(frame: &mut Frame, state: &AppState, area: Rect) {
    let focused = state.focused_panel == FocusedPanel::Filters;
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Saved Filters ")
        .borders(Borders::ALL)
        .border_style(border_style);

    if state.saved_filters.is_empty() {
        let msg = Paragraph::new("  (none)")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(msg, area);
    } else {
        let items: Vec<ListItem> = state.saved_filters
            .iter()
            .enumerate()
            .map(|(i, filter)| {
                let prefix = if i == state.selected_filter_idx { "▶ " } else { "  " };
                let indicator = if filter.is_regex { " [.*]" } else { "" };
                let style = if i == state.selected_filter_idx {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}{}", prefix, filter.name, indicator)).style(style)
            })
            .collect();

        let list = List::new(items).block(block);
        frame.render_widget(list, area);
    }
}

/// Draw the header showing the current source
fn draw_header(frame: &mut Frame, state: &AppState, area: Rect) {
    let source_name = state.current_source().name();
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" bark ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw("| "),
        Span::styled(source_name, Style::default().fg(Color::Cyan)),
    ]))
    .style(Style::default().bg(Color::DarkGray));

    frame.render_widget(header, area);
}

/// Draw the main log view
fn draw_log_view(frame: &mut Frame, state: &mut AppState, area: Rect) {
    let focused = state.focused_panel == FocusedPanel::LogView;
    let border_style = if focused && state.show_side_panel {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .borders(if state.show_side_panel { Borders::LEFT } else { Borders::NONE })
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let height = inner.height as usize;
    if height == 0 {
        return;
    }

    // Get visible lines
    let level_colors = state.level_colors_enabled;
    let visible = state.visible_lines(height);

    // Build the paragraph content
    let mut lines_content: Vec<Line<'_>> = Vec::with_capacity(height);

    for (_idx, line) in visible {
        let rendered = line.get_rendered(level_colors);
        // Clone the lines from the rendered text
        for text_line in rendered.lines.iter() {
            lines_content.push(text_line.clone());
            if lines_content.len() >= height {
                break;
            }
        }
        if lines_content.len() >= height {
            break;
        }
    }

    // Pad with empty lines if needed
    while lines_content.len() < height {
        lines_content.push(Line::default());
    }

    let mut paragraph = Paragraph::new(lines_content);
    if state.line_wrap {
        paragraph = paragraph.wrap(Wrap { trim: false });
    }
    frame.render_widget(paragraph, inner);

    // Draw scrollbar if there are more lines than visible
    let (total, filtered) = state.line_counts();
    if filtered > height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        let mut scrollbar_state = ScrollbarState::new(filtered)
            .position(state.scroll);

        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }

    // Show "no lines" message if empty
    if total == 0 {
        let msg = Paragraph::new("Waiting for log lines...")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(msg, inner);
    } else if filtered == 0 && state.active_filter.is_some() {
        let msg = Paragraph::new("No lines match the current filter")
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(msg, inner);
    }
}

/// Draw the status bar
fn draw_status_bar(frame: &mut Frame, state: &AppState, area: Rect) {
    let (total, filtered) = state.line_counts();

    let mode_str = match state.mode {
        InputMode::Normal => "NORMAL",
        InputMode::FilterEditing => "FILTER",
        InputMode::SourceSelect => "SOURCE",
    };

    let follow_indicator = if state.stick_to_bottom { "[F]" } else { "" };
    let regex_indicator = if state.filter_is_regex { "[.*]" } else { "" };
    let wrap_indicator = if state.line_wrap { "[W]" } else { "" };
    let color_indicator = if state.level_colors_enabled { "[C]" } else { "" };

    // Combine indicators
    let indicators: Vec<&str> = [follow_indicator, regex_indicator, wrap_indicator, color_indicator]
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect();
    let indicators_str = if indicators.is_empty() {
        String::new()
    } else {
        format!(" {}", indicators.join(" "))
    };

    let filter_str = state.active_filter
        .as_ref()
        .map(|f| format!(" | filter: {}", f.pattern))
        .unwrap_or_default();

    let help_text = match state.mode {
        InputMode::FilterEditing => " Enter:apply  Esc:cancel  Ctrl+r:regex ",
        _ => " ?:help  w:wrap  c:colors ",
    };

    let status = Line::from(vec![
        Span::styled(
            format!(" {} ", mode_str),
            Style::default().bg(Color::Blue).fg(Color::White),
        ),
        Span::raw(format!(" {}/{} lines{}{} ", filtered, total, indicators_str, filter_str)),
        Span::styled(help_text, Style::default().fg(Color::DarkGray)),
    ]);

    let paragraph = Paragraph::new(status)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(paragraph, area);
}

/// Draw the filter input bar
fn draw_filter_bar(frame: &mut Frame, state: &mut AppState, area: Rect) {
    match state.mode {
        InputMode::FilterEditing => {
            // Create a layout with "/" prefix and textarea
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(1),  // "/" prefix
                    Constraint::Min(1),     // textarea
                ])
                .split(area);

            let prefix = Paragraph::new("/")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(prefix, chunks[0]);

            frame.render_widget(&state.filter_textarea, chunks[1]);
        }
        _ => {
            if let Some(msg) = &state.status_message {
                let content = Line::from(Span::styled(msg.as_str(), Style::default().fg(Color::Yellow)));
                let paragraph = Paragraph::new(content);
                frame.render_widget(paragraph, area);
            }
        }
    }
}

/// Draw the help overlay
fn draw_help_overlay(frame: &mut Frame) {
    let area = frame.area();

    // Center the help box
    let width = 50.min(area.width.saturating_sub(4));
    let height = 22.min(area.height.saturating_sub(4));
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    let help_area = Rect::new(x, y, width, height);

    // Clear background
    frame.render_widget(Clear, help_area);

    let help_text = vec![
        Line::from(Span::styled("Keyboard Shortcuts", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j/k, ↑/↓     Scroll up/down"),
        Line::from("  g/G          Go to top/bottom"),
        Line::from("  PgUp/PgDn    Page up/down"),
        Line::from("  Mouse wheel  Scroll (when enabled)"),
        Line::from(""),
        Line::from("Filtering:"),
        Line::from("  /            Start filter input"),
        Line::from("  r            Toggle regex mode"),
        Line::from("  s            Save current filter"),
        Line::from("  Esc          Clear filter"),
        Line::from(""),
        Line::from("Display:"),
        Line::from("  w            Toggle line wrapping"),
        Line::from("  c            Toggle level colors"),
        Line::from("  b            Toggle side panel"),
        Line::from("  Tab          Cycle panel focus"),
        Line::from("  ?            Toggle this help"),
        Line::from("  q            Quit"),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(help_text).block(block);
    frame.render_widget(paragraph, help_area);
}
