//main ui rendering logic
use crate::app::{AppState, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

/// Main render function - called from the main loop
pub fn render(frame: &mut Frame, app: &AppState) {
    // Create main vertical layout (3 sections)
    let main_layout = Layout::vertical([
        Constraint::Length(3),  // Title bar
        Constraint::Min(10),    // Main content
        Constraint::Length(3),  // Command bar
    ])
    .split(frame.area());

    // Render each section
    render_title_bar(frame, app, main_layout[0]);
    render_main_content(frame, app, main_layout[1]);
    render_command_bar(frame, app, main_layout[2]);
}

/// Render the title bar at the top
fn render_title_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    let mode_indicator = match app.input_mode {
        InputMode::Normal => Span::styled(" NORMAL ", Style::default().bg(Color::Blue).fg(Color::White)),
        InputMode::Editing => Span::styled(" EDITING ", Style::default().bg(Color::Green).fg(Color::Black)),
    };

    let status = if let Some(msg) = &app.status_message {
        Span::styled(format!(" {} ", msg), Style::default().fg(Color::Yellow))
    } else {
        Span::raw("")
    };

    let title_line = Line::from(vec![
        Span::styled(" 📋 FaultNote ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("- Error Logger "),
        mode_indicator,
        Span::raw(" "),
        status,
    ]);

    let title_block = Paragraph::new(title_line)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));

    frame.render_widget(title_block, area);
}

/// Render the main content area (page list + input sections)
fn render_main_content(frame: &mut Frame, app: &AppState, area: Rect) {
    // Split horizontally: left sidebar (20%) + right content (80%)
    let content_layout = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(75),
    ])
    .split(area);

    // Render page list on the left
    render_page_list(frame, app, content_layout[0]);

    // Render input sections on the right
    render_input_sections(frame, app, content_layout[1]);
}

/// Render the Notion pages list on the left sidebar
fn render_page_list(frame: &mut Frame, app: &AppState, area: Rect) {
    // Create list items from notion_pages
    let items: Vec<ListItem> = app
        .notion_pages
        .iter()
        .enumerate()
        .map(|(idx, page)| {
            let style = if idx == app.selected_page_index && app.is_page_list_focused() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!(" {} ", page.title)).style(style)
        })
        .collect();

    // Empty state message if no pages
    let list = if items.is_empty() {
        List::new(vec![ListItem::new(" No pages loaded").style(Style::default().fg(Color::DarkGray))])
    } else {
        List::new(items)
    };

    // Determine border color based on focus
    let border_color = if app.is_page_list_focused() {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let list = list
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 📚 Notion Pages ")
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(Style::default().bg(Color::Rgb(45, 85, 155)).fg(Color::White))
        .highlight_symbol("▶ ");

    // Create list state for tracking selection
    let mut state = ListState::default();
    if !app.notion_pages.is_empty() {
        state.select(Some(app.selected_page_index));
    }

    frame.render_stateful_widget(list, area, &mut state);
}

/// Render the four input sections on the right
fn render_input_sections(frame: &mut Frame, app: &AppState, area: Rect) {
    // Split into 4 vertical sections
    let sections = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    // Render each input block
    render_input_block(
        frame,
        "🔴 Error",
        &app.error_input,
        app.active_input_field == 0 && app.is_input_section_focused(),
        app.active_input_field == 0 && app.is_editing(),
        sections[0],
    );

    render_input_block(
        frame,
        "🟡 Problem",
        &app.problem_input,
        app.active_input_field == 1 && app.is_input_section_focused(),
        app.active_input_field == 1 && app.is_editing(),
        sections[1],
    );

    render_input_block(
        frame,
        "🟢 Solution",
        &app.solution_input,
        app.active_input_field == 2 && app.is_input_section_focused(),
        app.active_input_field == 2 && app.is_editing(),
        sections[2],
    );

    render_input_block(
        frame,
        "💻 Code (optional)",
        &app.code_input,
        app.active_input_field == 3 && app.is_input_section_focused(),
        app.active_input_field == 3 && app.is_editing(),
        sections[3],
    );
}

/// Render a single input block
fn render_input_block(
    frame: &mut Frame,
    title: &str,
    content: &str,
    is_focused: bool,
    is_editing: bool,
    area: Rect,
) {
    // Determine styling based on state
    let (border_color, title_style) = if is_editing {
        (Color::Green, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    } else if is_focused {
        (Color::Yellow, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    } else {
        (Color::DarkGray, Style::default().fg(Color::Gray))
    };

    // Show cursor indicator when editing
    let display_content = if is_editing {
        format!("{}▌", content) // Add cursor
    } else {
        content.to_string()
    };

    // Create paragraph with content
    let paragraph = Paragraph::new(display_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(format!(" {} ", title), title_style))
                .border_style(Style::default().fg(border_color)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}

/// Render the command bar at the bottom
fn render_command_bar(frame: &mut Frame, app: &AppState, area: Rect) {
    let commands = if app.is_editing() {
        // Editing mode commands
        vec![
            ("Esc", "Exit Edit"),
            ("Tab", "Next Field"),
            ("Enter", "New Line"),
            ("↑↓", "Switch Field"),
        ]
    } else {
        // Normal mode commands
        vec![
            ("q", "Quit"),
            ("Tab", "Switch Focus"),
            ("↑↓", "Navigate"),
            ("e/i", "Edit"),
            ("Enter", "Submit"),
            ("c", "Clear"),
        ]
    };

    // Build command spans with styling
    let spans: Vec<Span> = commands
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" [{}] ", key), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{} ", desc), Style::default().fg(Color::White)),
                Span::raw(" "),
            ]
        })
        .collect();

    let command_line = Line::from(spans);

    let paragraph = Paragraph::new(command_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Commands ")
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .centered();

    frame.render_widget(paragraph, area);
}
