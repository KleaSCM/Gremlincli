use std::fs;
use colored::*;
use crate::dev::project_setup;
use crate::dev::github_utils::GitHubCli;

/// Load ASCII art from a file
#[allow(dead_code)]
fn load_template_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

/// Create a standard Rust project
#[allow(dead_code)]
pub fn create_rust_project(project_name: &str, is_private: bool) -> Result<(), String> {
    // Create basic project structure
    project_setup::create_project(project_name, "Standard Rust", is_private)?;

    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]"#, project_name);

    fs::write(format!("{}/Cargo.toml", project_name), cargo_toml)
        .map_err(|e| format!("Failed to create Cargo.toml: {}", e))?;

    // Create main.rs
    let main_rs = r#"fn main() {
    println!("Hello, world!");
}"#;

    fs::write(format!("{}/src/main.rs", project_name), main_rs)
        .map_err(|e| format!("Failed to create main.rs: {}", e))?;

    Ok(())
}

/// Create an Iced GUI project
#[allow(dead_code)]
pub fn create_iced_template(project_name: &str, is_private: bool) -> Result<(), String> {
    let base_path = "/home/klea/Documents/Dev/";
    let project_path = format!("{}{}", base_path, project_name);

    // Create basic project structure
    project_setup::create_project(&project_path, "Iced GUI", is_private)?;

    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
iced = "0.12"
iced_aw = "0.7"
tokio = {{ version = "1.0", features = ["full"] }}
futures = "0.3"
serde = {{ version = "1.0", features = ["derive"] }}"#, project_name);

    fs::write(format!("{}/Cargo.toml", project_path), cargo_toml)
        .map_err(|e| format!("Failed to create Cargo.toml: {}", e))?;

    // Create main.rs
    let main_rs = r#"use iced::{Application, Command, Element, Settings, Text};

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Text::new(format!("Counter: {}", self.value)).into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}"#;

    fs::write(format!("{}/src/main.rs", project_path), main_rs)
        .map_err(|e| format!("Failed to create main.rs: {}", e))?;

    // Display project structure
    println!("\n{}", "Project structure:".bright_cyan());
    project_setup::display_file_structure(&project_path, "");

    Ok(())
}

/// Create a Ratatui TUI project
#[allow(dead_code)]
pub fn create_ratatui_template(project_name: &str, is_private: bool) -> Result<(), String> {
    let base_path = "/home/klea/Documents/Dev/";
    let project_path = format!("{}{}", base_path, project_name);

    // Create basic project structure
    project_setup::create_project(&project_path, "Ratatui TUI", is_private)?;

    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.25"
crossterm = "0.27"
anyhow = "1.0"
tokio = {{ version = "1.0", features = ["full"] }}"#, project_name);

    fs::write(format!("{}/Cargo.toml", project_path), cargo_toml)
        .map_err(|e| format!("Failed to create Cargo.toml: {}", e))?;

    // Create main.rs
    let main_rs = r#"use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{stdout, Stdout};

struct App {
    counter: i32,
}

impl App {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }
}

fn main() -> Result<()> {
    let mut app = App::new();
    let mut terminal = setup_terminal()?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = ratatui::widgets::Block::default()
                .title("Counter")
                .borders(ratatui::widgets::Borders::ALL);
            f.render_widget(block, size);

            let text = format!("Counter: {}", app.counter);
            let text = ratatui::widgets::Paragraph::new(text)
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(text, size);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('+') => app.increment(),
                KeyCode::Char('-') => app.decrement(),
                _ => {}
            }
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}"#;

    fs::write(format!("{}/src/main.rs", project_path), main_rs)
        .map_err(|e| format!("Failed to create main.rs: {}", e))?;

    // Display final project structure
    println!("\n{}", "Final project structure:".bright_cyan());
    project_setup::display_file_structure(&project_path, "");

    Ok(())
}

/// Create a Tauri desktop app project
#[allow(dead_code)]
pub fn create_tauri_template(project_name: &str, is_private: bool) -> Result<(), String> {
    let base_path = "/home/klea/Documents/Dev/";
    let project_path = format!("{}{}", base_path, project_name);

    // Create basic project structure
    fs::create_dir_all(format!("{}/src", project_path))
        .map_err(|e| format!("Failed to create src directory: {}", e))?;

    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = "1.5"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0""#, project_name);

    fs::write(format!("{}/Cargo.toml", project_path), cargo_toml)
        .map_err(|e| format!("Failed to create Cargo.toml: {}", e))?;

    // Create package.json
    let package_json = r#"{
  "name": "tauri-app",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "tauri dev",
    "build": "tauri build"
  },
  "dependencies": {
    "@tauri-apps/api": "^1.5.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.0"
  }
}"#;

    fs::write(format!("{}/package.json", project_path), package_json)
        .map_err(|e| format!("Failed to create package.json: {}", e))?;

    // Create src-tauri directory and its contents
    let src_tauri_path = format!("{}/src-tauri", project_path);
    fs::create_dir_all(&src_tauri_path)
        .map_err(|e| format!("Failed to create src-tauri directory: {}", e))?;

    // Create src-tauri/tauri.conf.json
    let tauri_conf = r#"{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:3000",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Tauri App",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.tauri.dev",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Tauri App",
        "width": 800,
        "height": 600
      }
    ]
  }
}"#;

    fs::write(format!("{}/tauri.conf.json", src_tauri_path), tauri_conf)
        .map_err(|e| format!("Failed to create tauri.conf.json: {}", e))?;

    // Create src-tauri/src directory
    let src_tauri_src_path = format!("{}/src", src_tauri_path);
    fs::create_dir_all(&src_tauri_src_path)
        .map_err(|e| format!("Failed to create src-tauri/src directory: {}", e))?;

    // Create main.rs in src-tauri/src
    let main_rs = r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}"#;

    fs::write(format!("{}/main.rs", src_tauri_src_path), main_rs)
        .map_err(|e| format!("Failed to create main.rs: {}", e))?;

    // Create .gitignore
    let gitignore = r#"# Generated by Cargo
/target/
**/*.rs.bk
Cargo.lock
.idea/
.vscode/
*.swp
*.swo
.DS_Store
Thumbs.db
node_modules/
dist/
"#;

    fs::write(format!("{}/.gitignore", project_path), gitignore)
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;

    // Create README.md
    let readme = format!(r#"# {}

A Tauri desktop application.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Node.js and npm
- Tauri CLI: `cargo install tauri-cli`

### Installation

1. Install dependencies:
```bash
npm install
```

2. Run the development server:
```bash
npm run dev
```

3. Build the application:
```bash
npm run build
```

## Project Structure

```
{}
├── src/              # Frontend source code
├── src-tauri/        # Tauri backend
│   ├── src/         # Rust source code
│   └── tauri.conf.json
├── Cargo.toml       # Rust dependencies
└── package.json     # Node.js dependencies
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
"#, project_name, project_name);

    fs::write(format!("{}/README.md", project_path), readme)
        .map_err(|e| format!("Failed to create README.md: {}", e))?;

    // Initialize git if needed
    if is_private {
        let github = GitHubCli::new();
        github.create_repository(&project_path, true)
            .map_err(|e| format!("Failed to create GitHub repository: {}", e))?;
    }

    // Display project structure
    println!("\n{}", "Project structure:".bright_cyan());
    project_setup::display_file_structure(&project_path, "");

    // Run npm install
    println!("\n{}", "Installing npm dependencies...".bright_cyan());
    std::process::Command::new("npm")
        .arg("install")
        .current_dir(&project_path)
        .status()
        .map_err(|e| format!("Failed to run npm install: {}", e))?;
    println!("{}", "✅ npm dependencies installed successfully!".bright_green());

    Ok(())
}
