use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::AppState;
use crate::notion::client::{NotionClient, create_entry, FaultLogEntry};

/// Handle all input events for the application
/// Returns Ok(()) on success, Err on event reading failure
pub async fn handle_events(app: &mut AppState, notion_client: Option<&NotionClient>) -> io::Result<()> {
    // Poll for events with a small timeout (100ms)
    // This allows the UI to remain responsive
    if event::poll(Duration::from_millis(100))? {
        // Read the event
        if let Event::Key(key_event) = event::read()? {
            // Only handle key press events (not release)
            if key_event.kind == KeyEventKind::Press {
                handle_key_event(app, key_event, notion_client).await;
            }
        }
    }

    Ok(())
}

/// Handle a specific key event based on current app mode
async fn handle_key_event(app: &mut AppState, key: KeyEvent, notion_client: Option<&NotionClient>) {
    if app.is_editing() {
        // Editing mode - no async needed
        handle_editing_mode(app, key);
    } else {
        // Normal mode - may need async for submission
        handle_normal_mode(app, key, notion_client).await;
    }
}

/// Handle key events in normal (navigation) mode
async fn handle_normal_mode(app: &mut AppState, key: KeyEvent, notion_client: Option<&NotionClient>) {
    match key.code {
        // Application Control
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit();
        }

        // Focus Navigation
        KeyCode::Tab => {
            app.toggle_focus();
        }

        // Up/Down Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.handle_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.handle_down();
        }

        // Enter Edit Mode
        KeyCode::Char('e') | KeyCode::Char('i') => {
            app.enter_edit_mode();
        }

        // Submit to Notion
        KeyCode::Enter => {
            submit_to_notion(app, notion_client).await;
        }

        // Clear All Inputs
        KeyCode::Char('c') => {
            app.clear_inputs();
            app.set_status("Inputs cleared");
        }

        // Clear Status Message
        KeyCode::Esc => {
            app.clear_status();
        }

        _ => {}
    }
}

/// Submit the fault log entry to Notion
async fn submit_to_notion(app: &mut AppState, notion_client: Option<&NotionClient>) {
    // Check if we can submit
    if !app.can_submit() {
        app.set_error("Fill in Error, Problem, and Solution fields first");
        return;
    }

    // Check if we have a Notion client
    let client = match notion_client {
        Some(c) => c,
        None => {
            app.set_error("Notion API not connected. Check your API_KEY in .env");
            return;
        }
    };

    // Get the submission data
    let (page_id, entry_data) = match app.get_submission_data() {
        Some(data) => data,
        None => {
            app.set_error("Failed to prepare submission data");
            return;
        }
    };

    // Check if this is a demo page
    if page_id.starts_with("demo-") {
        app.set_error("Cannot submit to demo pages. Connect to Notion API first.");
        return;
    }

    // Create the FaultLogEntry for the API
    let entry = FaultLogEntry {
        error: entry_data.error,
        problem: entry_data.problem,
        solution: entry_data.solution,
        code: entry_data.code,
    };

    // Show loading status
    app.start_loading();

    // Make the API call
    match create_entry(client, &page_id, &entry).await {
        Ok(()) => {
            app.set_success("Error logged to Notion successfully! ✓");
            app.clear_inputs();
        }
        Err(e) => {
            app.set_error(format!("Failed to submit: {}", e));
        }
    }
}

/// Handle key events in editing mode
fn handle_editing_mode(app: &mut AppState, key: KeyEvent) {
    match key.code {
        // Exit Edit Mode
        KeyCode::Esc => {
            app.exit_edit_mode();
        }

        // Text Input
        KeyCode::Char(c) => {
            app.add_char(c);
        }

        // Backspace
        KeyCode::Backspace => {
            app.delete_char();
        }

        // New Line
        KeyCode::Enter => {
            // In editing mode, Enter adds a newline
            // Use Esc to exit, then Enter to submit
            app.add_newline();
        }

        // Navigate to Next Input (while editing)
        KeyCode::Tab => {
            app.exit_edit_mode();
            app.next_input();
            app.enter_edit_mode();
        }

        // Navigate Up/Down Between Inputs
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
