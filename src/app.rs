/// Which major section of the UI has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusArea {
    #[default]
    PageList, // Left sidebar (Notion pages)
    InputSection, // Right side (Error/Problem/Solution/Code)
}

/// Current input mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal, // Navigation mode - arrow keys move around
    Editing, // Typing mode - keys become text input
}

/// Simplified Notion page info for UI display
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub id: String,    // Notion page UUID
    pub title: String, // Display name
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
    // ─── Application Control ───
    pub running: bool, // false = exit main loop

    // ─── Focus & Mode ───
    pub current_focus: FocusArea, // Which section is focused
    pub input_mode: InputMode,    // Normal or Editing

    // ─── Notion Pages (Left Sidebar) ───
    pub notion_pages: Vec<PageInfo>, // All available pages
    pub selected_page_index: usize,  // Which page is highlighted

    // ─── Input Fields (Right Section) ───
    pub active_input_field: usize, // 0=Error, 1=Problem, 2=Solution, 3=Code
    pub error_input: String,       // Error description text
    pub problem_input: String,     // Problem analysis text
    pub solution_input: String,    // Solution description text
    pub code_input: String,        // Code snippet (optional)

    // ─── Status/Feedback ───
    pub status_message: Option<String>, // "Saved!" or "Error: ..."
    pub is_loading: bool,               // Show loading indicator
}

impl AppState {
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
    /// Signal the main loop to stop and exit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Check if the application is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Switch focus between PageList (left) and InputSection (right)
    pub fn toggle_focus(&mut self) {
        match self.current_focus {
            FocusArea::PageList => {
                self.current_focus = FocusArea::InputSection;
            }
            FocusArea::InputSection => {
                self.current_focus = FocusArea::PageList;
                self.input_mode = InputMode::Normal; // Always exit edit mode when leaving inputs
            }
        }
    }

    /// Check if page list is currently focused
    pub fn is_page_list_focused(&self) -> bool {
        matches!(self.current_focus, FocusArea::PageList)
    }

    /// Check if input section is currently focused
    pub fn is_input_section_focused(&self) -> bool {
        matches!(self.current_focus, FocusArea::InputSection)
    }

    /// Move selection down in the Notion pages list (with wrap-around)
    pub fn next_page(&mut self) {
        if self.notion_pages.is_empty() {
            return;
        }

        let total = self.notion_pages.len();
        self.selected_page_index = (self.selected_page_index + 1) % total;
    }

    /// Move selection up in the Notion pages list (with wrap-around)
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

    /// Get the currently selected Notion page (if any exist)
    pub fn get_selected_page(&self) -> Option<&PageInfo> {
        self.notion_pages.get(self.selected_page_index)
    }

    /// Get the ID of the currently selected page
    pub fn get_selected_page_id(&self) -> Option<&str> {
        self.get_selected_page().map(|p| p.id.as_str())
    }

    /// Set the notion pages (called after API fetch)
    pub fn set_pages(&mut self, pages: Vec<PageInfo>) {
        self.notion_pages = pages;
        self.selected_page_index = 0; // Reset selection
        self.is_loading = false;
    }

    /// Get the total number of pages
    pub fn page_count(&self) -> usize {
        self.notion_pages.len()
    }

    /// Total number of input fields
    const MAX_INPUTS: usize = 4;

    /// Move to next input field (Error → Problem → Solution → Code → Error)
    pub fn next_input(&mut self) {
        self.active_input_field = (self.active_input_field + 1) % Self::MAX_INPUTS;
    }

    /// Move to previous input field (with wrap-around)
    pub fn previous_input(&mut self) {
        if self.active_input_field == 0 {
            self.active_input_field = Self::MAX_INPUTS - 1;
        } else {
            self.active_input_field -= 1;
        }
    }

    /// Check if a specific input field is active
    /// index: 0=Error, 1=Problem, 2=Solution, 3=Code
    pub fn is_input_active(&self, index: usize) -> bool {
        self.active_input_field == index
    }

    /// Get the name of the currently active input field
    pub fn get_active_input_name(&self) -> &'static str {
        match self.active_input_field {
            0 => "Error",
            1 => "Problem",
            2 => "Solution",
            3 => "Code",
            _ => "Unknown",
        }
    }

    /// Switch to editing mode for current input field
    pub fn enter_edit_mode(&mut self) {
        // Only allow editing when focused on InputSection
        if self.is_input_section_focused() {
            self.input_mode = InputMode::Editing;
        }
    }

    /// Return to normal navigation mode
    pub fn exit_edit_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    /// Check if currently in editing mode
    pub fn is_editing(&self) -> bool {
        matches!(self.input_mode, InputMode::Editing)
    }

    /// Check if currently in normal mode
    pub fn is_normal_mode(&self) -> bool {
        matches!(self.input_mode, InputMode::Normal)
    }

    /// Get immutable reference to the currently active input field
    pub fn get_active_input(&self) -> &String {
        match self.active_input_field {
            0 => &self.error_input,
            1 => &self.problem_input,
            2 => &self.solution_input,
            3 => &self.code_input,
            _ => &self.error_input, // Fallback (should never happen)
        }
    }

    /// Get mutable reference to the currently active input field
    pub fn get_active_input_mut(&mut self) -> &mut String {
        match self.active_input_field {
            0 => &mut self.error_input,
            1 => &mut self.problem_input,
            2 => &mut self.solution_input,
            3 => &mut self.code_input,
            _ => &mut self.error_input, // Fallback (should never happen)
        }
    }

    /// Add a character to the currently active input field
    pub fn add_char(&mut self, c: char) {
        self.get_active_input_mut().push(c);
    }

    /// Remove the last character from active input field (Backspace)
    pub fn delete_char(&mut self) {
        self.get_active_input_mut().pop();
    }

    /// Add a newline to the active input field
    pub fn add_newline(&mut self) {
        self.get_active_input_mut().push('\n');
    }

    /// Clear all input fields and reset to first field
    pub fn clear_inputs(&mut self) {
        self.error_input.clear();
        self.problem_input.clear();
        self.solution_input.clear();
        self.code_input.clear();
        self.active_input_field = 0;
    }

    /// Get the content of a specific input field by index
    pub fn get_input_by_index(&self, index: usize) -> &String {
        match index {
            0 => &self.error_input,
            1 => &self.problem_input,
            2 => &self.solution_input,
            3 => &self.code_input,
            _ => &self.error_input,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //                          SUBMISSION
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if we have enough data to submit to Notion
    /// Required: error, problem, solution (not empty) + a page selected
    /// Code is optional
    pub fn can_submit(&self) -> bool {
        let has_error = !self.error_input.trim().is_empty();
        let has_problem = !self.problem_input.trim().is_empty();
        let has_solution = !self.solution_input.trim().is_empty();
        let has_page = !self.notion_pages.is_empty();

        has_error && has_problem && has_solution && has_page
    }

    /// Create the data structure needed for Notion API submission
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

    /// Mark submission as in progress
    pub fn start_loading(&mut self) {
        self.is_loading = true;
        self.status_message = Some("Submitting...".to_string());
    }

    /// Mark submission as complete
    pub fn finish_loading(&mut self) {
        self.is_loading = false;
    }

    // ═══════════════════════════════════════════════════════════════════════
    //                        STATUS MESSAGES
    // ═══════════════════════════════════════════════════════════════════════

    /// Set a status message to display to user
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    /// Set a success message
    pub fn set_success(&mut self, message: impl Into<String>) {
        self.status_message = Some(format!("✓ {}", message.into()));
        self.is_loading = false;
    }

    /// Set an error message
    pub fn set_error(&mut self, message: impl Into<String>) {
        self.status_message = Some(format!("✗ {}", message.into()));
        self.is_loading = false;
    }

    /// Clear the status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Get the current status message
    pub fn get_status(&self) -> Option<&String> {
        self.status_message.as_ref()
    }

    /// Check if there's a status message to display
    pub fn has_status(&self) -> bool {
        self.status_message.is_some()
    }

    /// Handle navigation based on current focus
    pub fn handle_up(&mut self) {
        match self.current_focus {
            FocusArea::PageList => self.previous_page(),
            FocusArea::InputSection => self.previous_input(),
        }
    }

    /// Handle navigation based on current focus
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

//test for the navigation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app_state() {
        let app = AppState::new();
        assert!(app.running);
        assert!(matches!(app.current_focus, FocusArea::PageList));
        assert!(matches!(app.input_mode, InputMode::Normal));
        assert!(app.notion_pages.is_empty());
        assert_eq!(app.selected_page_index, 0);
        assert_eq!(app.active_input_field, 0);
    }

    #[test]
    fn test_toggle_focus() {
        let mut app = AppState::new();
        assert!(app.is_page_list_focused());

        app.toggle_focus();
        assert!(app.is_input_section_focused());

        app.toggle_focus();
        assert!(app.is_page_list_focused());
    }

    #[test]
    fn test_page_navigation() {
        let mut app = AppState::new();
        app.set_pages(vec![
            PageInfo {
                id: "1".to_string(),
                title: "Page 1".to_string(),
            },
            PageInfo {
                id: "2".to_string(),
                title: "Page 2".to_string(),
            },
            PageInfo {
                id: "3".to_string(),
                title: "Page 3".to_string(),
            },
        ]);

        assert_eq!(app.selected_page_index, 0);

        app.next_page();
        assert_eq!(app.selected_page_index, 1);

        app.next_page();
        assert_eq!(app.selected_page_index, 2);

        // Wrap around
        app.next_page();
        assert_eq!(app.selected_page_index, 0);

        // Go backwards
        app.previous_page();
        assert_eq!(app.selected_page_index, 2);
    }

    #[test]
    fn test_input_navigation() {
        let mut app = AppState::new();

        assert_eq!(app.active_input_field, 0);

        app.next_input();
        assert_eq!(app.active_input_field, 1);

        app.next_input();
        app.next_input();
        app.next_input();
        assert_eq!(app.active_input_field, 0); // Wrapped around

        app.previous_input();
        assert_eq!(app.active_input_field, 3);
    }

    #[test]
    fn test_text_input() {
        let mut app = AppState::new();

        app.add_char('H');
        app.add_char('e');
        app.add_char('l');
        app.add_char('l');
        app.add_char('o');

        assert_eq!(app.error_input, "Hello");

        app.delete_char();
        assert_eq!(app.error_input, "Hell");

        app.next_input();
        app.add_char('W');
        app.add_char('o');
        app.add_char('r');
        app.add_char('l');
        app.add_char('d');

        assert_eq!(app.problem_input, "World");
    }

    #[test]
    fn test_can_submit() {
        let mut app = AppState::new();

        // Can't submit with no pages
        assert!(!app.can_submit());

        // Add a page
        app.set_pages(vec![PageInfo {
            id: "1".to_string(),
            title: "Test".to_string(),
        }]);

        // Still can't submit - no content
        assert!(!app.can_submit());

        // Add required fields
        app.error_input = "Test error".to_string();
        assert!(!app.can_submit());

        app.problem_input = "Test problem".to_string();
        assert!(!app.can_submit());

        app.solution_input = "Test solution".to_string();
        assert!(app.can_submit()); // Now we can submit!

        // Code is optional
        app.code_input = "fn main() {}".to_string();
        assert!(app.can_submit());
    }

    #[test]
    fn test_clear_inputs() {
        let mut app = AppState::new();

        app.error_input = "Error".to_string();
        app.problem_input = "Problem".to_string();
        app.solution_input = "Solution".to_string();
        app.code_input = "Code".to_string();
        app.active_input_field = 2;

        app.clear_inputs();

        assert!(app.error_input.is_empty());
        assert!(app.problem_input.is_empty());
        assert!(app.solution_input.is_empty());
        assert!(app.code_input.is_empty());
        assert_eq!(app.active_input_field, 0);
    }
}
