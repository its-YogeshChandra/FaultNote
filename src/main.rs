// FaultNote - Error Logger TUI Application
use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod app;
mod events;
mod notion;
mod ui;

use app::{AppState, PageInfo};
use notion::client::{NotionClient, create_notion_client, fetch_pages};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize the application
    let mut app = AppState::new();

    // Try to create Notion client and fetch pages
    let notion_client = match create_notion_client() {
        Ok(client) => {
            app.set_status("Connected to Notion API");
            Some(client)
        }
        Err(e) => {
            app.set_error(format!("Notion API error: {}. Using demo pages.", e));
            // Add demo pages as fallback
            app.set_pages(vec![
                PageInfo {
                    id: "demo-1".to_string(),
                    title: "Demo: Project Errors".to_string(),
                },
                PageInfo {
                    id: "demo-2".to_string(),
                    title: "Demo: Bug Tracker".to_string(),
                },
            ]);
            None
        }
    };

    // Fetch actual pages from Notion if client is available
    if let Some(ref client) = notion_client {
        app.set_status("Fetching pages from Notion...");
        match fetch_pages(client).await {
            Ok(pages) => {
                if pages.is_empty() {
                    app.set_status("No pages found. Create a page in Notion first.");
                } else {
                    app.set_success(format!("Loaded {} pages from Notion", pages.len()));
                    app.set_pages(pages);
                }
            }
            Err(e) => {
                app.set_error(format!("Failed to fetch pages: {}", e));
            }
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear the terminal
    terminal.clear()?;

    // Main application loop
    let result = run_app(&mut terminal, &mut app, notion_client.as_ref()).await;

    // Restore terminal on exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Handle any errors from the app
    if let Err(err) = result {
        eprintln!("Application error: {}", err);
        return Err(err);
    }

    println!("Thanks for using FaultNote! 👋");
    Ok(())
}

/// Main application loop
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
    notion_client: Option<&NotionClient>,
) -> io::Result<()> {
    while app.is_running() {
        // Draw the UI
        terminal.draw(|frame| {
            ui::render(frame, app);
        })?;

        // Handle input events (including submission)
        events::handle_events(app, notion_client).await?;
    }

    Ok(())
}
