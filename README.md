# 🔴 FaultNote

A terminal-based error logger that helps you document errors, problems, and solutions directly to your Notion pages.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Ratatui](https://img.shields.io/badge/TUI-Ratatui-blue)

## ✨ Features

- 📚 Browse and select from your Notion pages
- 🔴 Log errors with structured sections (Error, Problem, Solution, Code)
- ⌨️ Keyboard-driven interface
- 🎨 Clean terminal UI built with Ratatui

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- A Notion account with an [integration token](https://www.notion.so/my-integrations)

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/its-YogeshChandra/FaultNote.git
   cd faultnote
   ```

2. **Configure environment variables**
   
   Create a `.env` file in the root directory:
   ```env
   NOTION_API_KEY=your_notion_integration_token
   ```

3. **Run the application**
   ```bash
   cargo run
   ```

   Or build for release:
   ```bash
   cargo build --release
   ./target/release/faultnote
   ```

## ⌨️ Keyboard Controls

| Key | Action |
|-----|--------|
| `Tab` | Switch focus between sections |
| `↑` / `↓` | Navigate pages / input fields |
| `Enter` | Select page / Submit entry |
| `e` | Enter edit mode |
| `Esc` | Exit edit mode |
| `q` | Quit application |

## 📁 Project Structure

```
faultnote/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Application state
│   ├── ui.rs            # UI rendering
│   ├── events.rs        # Event handling
│   ├── tui.rs           # Terminal setup
│   └── notion/          # Notion API client
└── Cargo.toml
```

## 📄 License

Copyright (c) its-YogeshChandra <pandittheroyal@gmail.com>

This project is licensed under the MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)
