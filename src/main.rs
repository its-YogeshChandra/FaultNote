//data struct that is being taken from user
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

mod app;
mod notion;
mod ui;

use crate::notion::client;
use app::AppState;

fn main() {
    //initiallize terminal

    //create app instance
    let app_instance = AppState::new(
        running,
        current_focus,
        notion_pages,
        selected_page_index,
        input_mode,
        error_input,
        problem_input,
        solution_input,
        code_input,
        active_input_field,
    );

    //fetch notion page

    //mainloop

    //clean up on exit ;
}
