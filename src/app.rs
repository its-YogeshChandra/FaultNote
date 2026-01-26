/// Which major section of the UI has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusArea {
    #[default]
    PageList,
    InputSection,
}

/// Current input mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

/// Simplified Notion page info for UI display
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub id: String,
    pub title: String,
}

/// Data to be sent to Notion when submitting a fault log
#[derive(Debug, Clone)]
pub struct FaultLogEntry {
    pub error: String,
    pub problem: String,
    pub solution: String,
    pub code: Option<String>,
}

/// Main application state
#[derive(Debug)]
pub struct AppState {
    pub running: bool,
    pub current_focus: FocusArea,
    pub input_mode: InputMode,
    pub notion_pages: Vec<PageInfo>,
    pub selected_page_index: usize,
    pub active_input_field: usize,
    pub error_input: String,
    pub problem_input: String,
    pub solution_input: String,
    pub code_input: String,
    pub status_message: Option<String>,
    pub is_loading: bool,
}

impl AppState {
    const MAX_INPUTS: usize = 4;

    /// Create a new AppState with default values
    pub fn new() -> Self {
        Self {
            running: true,
            current_focus: FocusArea::PageList,
            input_mode: InputMode::Normal,
            notion_pages: Vec::new(),
            selected_page_index: 0,
            active_input_field: 0,
            error_input: String::new(),
            problem_input: String::new(),
            solution_input: String::new(),
            code_input: String::new(),
            status_message: None,
            is_loading: false,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn toggle_focus(&mut self) {
        match self.current_focus {
            FocusArea::PageList => {
                self.current_focus = FocusArea::InputSection;
            }
            FocusArea::InputSection => {
                self.current_focus = FocusArea::PageList;
                self.input_mode = InputMode::Normal;
            }
        }
    }

    pub fn is_page_list_focused(&self) -> bool {
        matches!(self.current_focus, FocusArea::PageList)
    }

    pub fn is_input_section_focused(&self) -> bool {
        matches!(self.current_focus, FocusArea::InputSection)
    }

    pub fn next_page(&mut self) {
        if self.notion_pages.is_empty() {
            return;
        }
        let total = self.notion_pages.len();
        self.selected_page_index = (self.selected_page_index + 1) % total;
    }

    pub fn previous_page(&mut self) {
        if self.notion_pages.is_empty() {
            return;
        }
        let total = self.notion_pages.len();
        if self.selected_page_index == 0 {
            self.selected_page_index = total - 1;
        } else {
            self.selected_page_index -= 1;
        }
    }

    pub fn get_selected_page(&self) -> Option<&PageInfo> {
        self.notion_pages.get(self.selected_page_index)
    }

    pub fn set_pages(&mut self, pages: Vec<PageInfo>) {
        self.notion_pages = pages;
        self.selected_page_index = 0;
        self.is_loading = false;
    }

    pub fn next_input(&mut self) {
        self.active_input_field = (self.active_input_field + 1) % Self::MAX_INPUTS;
    }

    pub fn previous_input(&mut self) {
        if self.active_input_field == 0 {
            self.active_input_field = Self::MAX_INPUTS - 1;
        } else {
            self.active_input_field -= 1;
        }
    }

    pub fn enter_edit_mode(&mut self) {
        if self.is_input_section_focused() {
            self.input_mode = InputMode::Editing;
        }
    }

    pub fn exit_edit_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn is_editing(&self) -> bool {
        matches!(self.input_mode, InputMode::Editing)
    }

    fn get_active_input_mut(&mut self) -> &mut String {
        match self.active_input_field {
            0 => &mut self.error_input,
            1 => &mut self.problem_input,
            2 => &mut self.solution_input,
            3 => &mut self.code_input,
            _ => &mut self.error_input,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.get_active_input_mut().push(c);
    }

    pub fn delete_char(&mut self) {
        self.get_active_input_mut().pop();
    }

    pub fn add_newline(&mut self) {
        self.get_active_input_mut().push('\n');
    }

    pub fn clear_inputs(&mut self) {
        self.error_input.clear();
        self.problem_input.clear();
        self.solution_input.clear();
        self.code_input.clear();
        self.active_input_field = 0;
    }

    pub fn can_submit(&self) -> bool {
        let has_error = !self.error_input.trim().is_empty();
        let has_problem = !self.problem_input.trim().is_empty();
        let has_solution = !self.solution_input.trim().is_empty();
        let has_page = !self.notion_pages.is_empty();
        has_error && has_problem && has_solution && has_page
    }

    pub fn get_submission_data(&self) -> Option<(String, FaultLogEntry)> {
        if !self.can_submit() {
            return None;
        }

        let page = self.get_selected_page()?;
        let page_id = page.id.clone();

        let entry = FaultLogEntry {
            error: self.error_input.clone(),
            problem: self.problem_input.clone(),
            solution: self.solution_input.clone(),
            code: if self.code_input.trim().is_empty() {
                None
            } else {
                Some(self.code_input.clone())
            },
        };

        Some((page_id, entry))
    }

    pub fn start_loading(&mut self) {
        self.is_loading = true;
        self.status_message = Some("Submitting...".to_string());
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    pub fn set_success(&mut self, message: impl Into<String>) {
        self.status_message = Some(format!("✓ {}", message.into()));
        self.is_loading = false;
    }

    pub fn set_error(&mut self, message: impl Into<String>) {
        self.status_message = Some(format!("✗ {}", message.into()));
        self.is_loading = false;
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn handle_up(&mut self) {
        match self.current_focus {
            FocusArea::PageList => self.previous_page(),
            FocusArea::InputSection => self.previous_input(),
        }
    }

    pub fn handle_down(&mut self) {
        match self.current_focus {
            FocusArea::PageList => self.next_page(),
            FocusArea::InputSection => self.next_input(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
