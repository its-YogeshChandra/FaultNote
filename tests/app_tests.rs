// Tests for AppState

use faultnote::app::{AppState, FocusArea, FaultLogEntry, InputMode, PageInfo};

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
        PageInfo { id: "1".to_string(), title: "Page 1".to_string() },
        PageInfo { id: "2".to_string(), title: "Page 2".to_string() },
        PageInfo { id: "3".to_string(), title: "Page 3".to_string() },
    ]);

    assert_eq!(app.selected_page_index, 0);

    app.next_page();
    assert_eq!(app.selected_page_index, 1);

    app.next_page();
    assert_eq!(app.selected_page_index, 2);

    app.next_page();
    assert_eq!(app.selected_page_index, 0); // Wrapped

    app.previous_page();
    assert_eq!(app.selected_page_index, 2);
}

#[test]
fn test_empty_page_navigation() {
    let mut app = AppState::new();
    app.next_page();
    app.previous_page();
    assert_eq!(app.selected_page_index, 0);
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
    assert_eq!(app.active_input_field, 0); // Wrapped

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
fn test_newline_input() {
    let mut app = AppState::new();
    app.add_char('A');
    app.add_newline();
    app.add_char('B');
    assert_eq!(app.error_input, "A\nB");
}

#[test]
fn test_can_submit() {
    let mut app = AppState::new();
    assert!(!app.can_submit());

    app.set_pages(vec![PageInfo { id: "1".to_string(), title: "Test".to_string() }]);
    assert!(!app.can_submit());

    app.error_input = "Error".to_string();
    assert!(!app.can_submit());

    app.problem_input = "Problem".to_string();
    assert!(!app.can_submit());

    app.solution_input = "Solution".to_string();
    assert!(app.can_submit());
}

#[test]
fn test_whitespace_only_not_submittable() {
    let mut app = AppState::new();
    app.set_pages(vec![PageInfo { id: "1".to_string(), title: "Test".to_string() }]);
    app.error_input = "   ".to_string();
    app.problem_input = "Problem".to_string();
    app.solution_input = "Solution".to_string();
    assert!(!app.can_submit());
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

#[test]
fn test_edit_mode() {
    let mut app = AppState::new();

    app.enter_edit_mode();
    assert!(!app.is_editing()); // Can't edit when on page list

    app.toggle_focus();
    app.enter_edit_mode();
    assert!(app.is_editing());

    app.exit_edit_mode();
    assert!(!app.is_editing());
}

#[test]
fn test_status_messages() {
    let mut app = AppState::new();
    assert!(app.status_message.is_none());

    app.set_status("Test");
    assert_eq!(app.status_message, Some("Test".to_string()));

    app.set_success("OK");
    assert!(app.status_message.as_ref().unwrap().contains("✓"));

    app.set_error("Fail");
    assert!(app.status_message.as_ref().unwrap().contains("✗"));

    app.clear_status();
    assert!(app.status_message.is_none());
}

#[test]
fn test_submission_data() {
    let mut app = AppState::new();
    assert!(app.get_submission_data().is_none());

    app.set_pages(vec![PageInfo { id: "page-id".to_string(), title: "Test".to_string() }]);
    app.error_input = "Error".to_string();
    app.problem_input = "Problem".to_string();
    app.solution_input = "Solution".to_string();
    app.code_input = "Code".to_string();

    let (page_id, entry) = app.get_submission_data().unwrap();
    assert_eq!(page_id, "page-id");
    assert_eq!(entry.error, "Error");
    assert_eq!(entry.code, Some("Code".to_string()));
}

#[test]
fn test_handle_up_down() {
    let mut app = AppState::new();
    app.set_pages(vec![
        PageInfo { id: "1".to_string(), title: "P1".to_string() },
        PageInfo { id: "2".to_string(), title: "P2".to_string() },
    ]);

    app.handle_down();
    assert_eq!(app.selected_page_index, 1);
    app.handle_up();
    assert_eq!(app.selected_page_index, 0);

    app.toggle_focus();
    app.handle_down();
    assert_eq!(app.active_input_field, 1);
    app.handle_up();
    assert_eq!(app.active_input_field, 0);
}

#[test]
fn test_fault_log_entry() {
    let entry = FaultLogEntry {
        error: "E".to_string(),
        problem: "P".to_string(),
        solution: "S".to_string(),
        code: Some("C".to_string()),
    };
    assert_eq!(entry.error, "E");
    assert_eq!(entry.code, Some("C".to_string()));
}
