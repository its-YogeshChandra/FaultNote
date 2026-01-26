//for the application state
//use std::io;

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

use crate::client::NotionPage;

//main app state
pub struct AppState {
    pub running: bool,
    pub current_focus: FocusArea,
    pub notion_pages: Vec<NotionPage>,
    pub selected_page_index: usize,
    pub input_mode: InputMode,

    //the four input fields
    pub error_input: String,
    pub problem_input: String,
    pub solution_input: String,
    pub code_input: String,

    //active input field
    pub active_input_field: usize,
}

//enum for focus area
pub enum FocusArea {
    PageList,
    InputSection,
}

//enum for inputmode
pub enum InputMode {
    Normal,
    Editing,
}

//impl for app state
impl AppState {
    //function to create new state
    fn new(
        running: bool,
        current_focus: FocusArea,
        notion_pages: Vec<NotionPage>,
        selected_page_index: usize,
        input_mode: InputMode,
        //the four input fields
        error_input: String,
        problem_input: String,
        solution_input: String,
        code_input: String,
        //active input field
        active_input_field: usize,
    ) -> Self {
        Self {
            running,
            current_focus,
            notion_pages,
            selected_page_index,
            input_mode,
            //the four input fields
            error_input,
            problem_input,
            solution_input,
            code_input,
            //active input field
            active_input_field,
        }
    }

    fn change_page(value: String) {
        //check for the value strcut
        if value == "prev".to_string() {
        } else {
        }
    }

    fn change_input(value: String) {
        //check for the value strcut
        if value == "prev".to_string() {
        } else {
        }
    }

    fn toggle_focus() {
        //change the foucs area from the enum
    }

    fn submit_to_notion() {}

    fn clear_inputs() {}
}
