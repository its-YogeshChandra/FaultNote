// ═══════════════════════════════════════════════════════════════════════════
// FaultNote - Error Logger TUI Application
// ═══════════════════════════════════════════════════════════════════════════

use std::io;

mod app;
mod events;
mod notion;
mod ui;

use app::AppState;

fn main() -> io::Result<()> {
    // ─────────────────────────────────────────────────────────────────────
    // 1. Create app instance with defaults
    // ─────────────────────────────────────────────────────────────────────
    let mut app = AppState::new();

    // ─────────────────────────────────────────────────────────────────────
    // 2. TODO: Initialize terminal
    //    - enable_raw_mode()
    //    - Enter alternate screen
    //    - Create Terminal with CrosstermBackend
    // ─────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────
    // 3. TODO: Fetch Notion pages
    //    - Call notion::client::fetch_pages()
    //    - app.set_pages(pages)
    // ─────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────
    // 4. TODO: Main loop
    //    while app.is_running() {
    //        - terminal.draw(|frame| ui::render(frame, &app))
    //        - events::handle_events(&mut app)
    //    }
    // ─────────────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────────────
    // 5. TODO: Cleanup on exit
    //    - disable_raw_mode()
    //    - Leave alternate screen
    // ─────────────────────────────────────────────────────────────────────

    println!("FaultNote initialized successfully!");
    println!("App state: {:?}", app);

    Ok(())
}
