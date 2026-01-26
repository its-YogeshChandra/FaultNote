use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::AppState;

/// Handle all input events for the application
/// Returns Ok(()) on success, Err on event reading failure
pub fn handle_events(app: &mut AppState) -> io::Result<()> {
    // Poll for events with a small timeout (100ms)
    // This allows the UI to remain responsive
    if event::poll(Duration::from_millis(100))? {
        // Read the event
        if let Event::Key(key_event) = event::read()? {
            // Only handle key press events (not release)
            if key_event.kind == KeyEventKind::Press {
                handle_key_event(app, key_event);
            }
        }
    }

    Ok(())
}

/// Handle a specific key event based on current app mode
fn handle_key_event(app: &mut AppState, key: KeyEvent) {
    if app.is_editing() {
        //editing mode
        handle_editing_mode(app, key);
    } else {
        // normal mode
        handle_normal_mode(app, key);
    }
}

/// Handle key events in normal (navigation) mode
fn handle_normal_mode(app: &mut AppState, key: KeyEvent) {
    match key.code {
        // ─── Application Control ───
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }

        // ─── Focus Navigation ───
        KeyCode::Tab => {
            app.toggle_focus();
        }

        // ─── Up/Down Navigation ───
        KeyCode::Up | KeyCode::Char('k') => {
            app.handle_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.handle_down();
        }

        // ─── Enter Edit Mode ───
        KeyCode::Char('e') | KeyCode::Char('i') => {
            app.enter_edit_mode();
        }

        // ─── Submit to Notion ───
        KeyCode::Enter => {
            if app.can_submit() {
                // TODO: Trigger async submission
                app.set_status("Ready to submit! (async not connected yet)");
            } else {
                app.set_error("Fill in Error, Problem, and Solution fields first");
            }
        }

        // ─── Clear All Inputs ───
        KeyCode::Char('c') => {
            app.clear_inputs();
            app.set_status("Inputs cleared");
        }

        // ─── Clear Status Message ───
        KeyCode::Esc => {
            app.clear_status();
        }

        _ => {}
    }
}

/// Handle key events in editing mode
fn handle_editing_mode(app: &mut AppState, key: KeyEvent) {
    match key.code {
        // ─── Exit Edit Mode ───
        KeyCode::Esc => {
            app.exit_edit_mode();
        }

        // ─── Text Input ───
        KeyCode::Char(c) => {
            app.add_char(c);
        }

        // ─── Backspace ───
        KeyCode::Backspace => {
            app.delete_char();
        }

        // ─── New Line ───
        KeyCode::Enter => {
            // In editing mode, Enter adds a newline
            // Use Esc to exit, then Enter to submit
            app.add_newline();
        }

        // ─── Navigate to Next Input (while editing) ───
        KeyCode::Tab => {
            app.exit_edit_mode();
            app.next_input();
            app.enter_edit_mode();
        }

        // ─── Navigate Up/Down Between Inputs ───
        KeyCode::Up => {
            app.exit_edit_mode();
            app.previous_input();
        }
        KeyCode::Down => {
            app.exit_edit_mode();
            app.next_input();
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mode_quit() {
        let mut app = AppState::new();
        assert!(app.is_running());

        let key = KeyEvent::new(KeyCode::Char('q'), crossterm::event::KeyModifiers::NONE);
        handle_normal_mode(&mut app, key);

        assert!(!app.is_running());
    }

    #[test]
    fn test_normal_mode_toggle_focus() {
        let mut app = AppState::new();
        assert!(app.is_page_list_focused());

        let key = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        handle_normal_mode(&mut app, key);

        assert!(app.is_input_section_focused());
    }

    #[test]
    fn test_editing_mode_add_char() {
        let mut app = AppState::new();
        app.enter_edit_mode();

        let key = KeyEvent::new(KeyCode::Char('H'), crossterm::event::KeyModifiers::NONE);
        handle_editing_mode(&mut app, key);

        assert_eq!(app.get_active_input(), "H");
    }

    #[test]
    fn test_editing_mode_backspace() {
        let mut app = AppState::new();
        app.error_input = "Hello".to_string();
        app.enter_edit_mode();

        let key = KeyEvent::new(KeyCode::Backspace, crossterm::event::KeyModifiers::NONE);
        handle_editing_mode(&mut app, key);

        assert_eq!(app.get_active_input(), "Hell");
    }

    #[test]
    fn test_editing_mode_escape() {
        let mut app = AppState::new();
        app.toggle_focus(); // Go to input section
        app.enter_edit_mode();
        assert!(app.is_editing());

        let key = KeyEvent::new(KeyCode::Esc, crossterm::event::KeyModifiers::NONE);
        handle_editing_mode(&mut app, key);

        assert!(app.is_normal_mode());
    }
}
