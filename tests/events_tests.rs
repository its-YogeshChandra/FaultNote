// Tests for event handling

use faultnote::app::AppState;

#[test]
fn test_app_state_for_events() {
    let app = AppState::new();
    assert!(app.is_running());
}

#[test]
fn test_app_quit() {
    let mut app = AppState::new();
    assert!(app.is_running());
    app.quit();
    assert!(!app.is_running());
}
