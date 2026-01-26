//main ui rendering logic :
use crate::app::AppState;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget, Wrap},
};
use std::io;

//fix the issue
fn render(mut frame: Frame, app: AppState) {
    //create main vertical layout
    let main_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(5),
    ])
    .split(frame.area());

    // render title bar
    let title = Block::bordered().title("FaultNote- Error Logger");
    frame.render_widget(title, main_layout[0]);

    //split the middle section horizontally
    let content_layout =
        Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_layout[1]);

    //render page list
    render_page_list(&mut frame, &app, content_layout[0]);

    //split right section into 4 sub section
    let right_layout = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(content_layout[1]);

    //render the each input section
    render_input_block(
        &mut frame,
        "Error".to_string(),
        &app.error_input,
        app.active_input_field == 0,
        right_layout[0],
    );
    render_input_block(
        &mut frame,
        "Problem".to_string(),
        &app.error_input,
        app.active_input_field == 0,
        right_layout[1],
    );
    render_input_block(
        &mut frame,
        "Error".to_string(),
        &app.error_input,
        app.active_input_field == 0,
        right_layout[2],
    );
    render_input_block(
        &mut frame,
        "Code".to_string(),
        &app.error_input,
        app.active_input_field == 0,
        right_layout[3],
    );

    //render the command bar
    render_command_bar(frame, main_layout[2]);
}

//function to render page list
fn render_page_list(frame: &mut Frame, app: &AppState, area: Rect) {
    //create list items from notion_pages
    let items = app
        .notion_pages
        .iter()
        .map(|page| ListItem::new(page.title.clone()));

    //create list widget with highlight style
    let list = List::new(items)
        .block(Block::bordered().title("Notion Pages"))
        .highlight_style(Style::default().bg(Color::Blue))
        .highlight_symbol("> ");

    //create list for tracking selection
    let mut state = ListState::default().with_selected(Some(app.selected_page_index));

    //render a statefull widget
    &frame.render_stateful_widget(list, area, &mut state);
}

fn render_input_block(
    frame: &mut Frame,
    title: String,
    content: &String,
    is_active: bool,
    area: Rect,
) {
    //determine border style based on focus
    let border_style = if is_active {
        Color::Yellow
    } else {
        Color::Gray
    };

    //create paragraph with content
    let paragraph = Paragraph::new(content.to_string())
        .block(
            Block::bordered()
                .title(title)
                .border_style(Style::default().fg(border_style)),
        )
        .wrap(Wrap { trim: true });

    //render the frame
    frame.render_widget(paragraph, area);
}

fn render_command_bar(mut frame: Frame, area: Rect) {
    //commands for the value
    let commands = "[q] Quit  [Tab] Focus  [↑↓] Navigate  [Enter] Submit  [e] Edit  [Esc] Cancel";

    //paragraph
    let paragraph = Paragraph::new(commands)
        .block(Block::bordered())
        .style(Style::default().fg(Color::Cyan))
        .centered();

    //render the widget
    frame.render_widget(paragraph, area);
}
