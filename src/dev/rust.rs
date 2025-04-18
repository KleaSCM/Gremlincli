use std::io::{self, Write};
use std::fs;
use std::process::Command;
use colored::*;
use crate::dev::github_utils::GitHubCli;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn initialize_git(project_path: &str, is_private: bool) -> Result<(), String> {
    // Initialize git repository
    Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to initialize git: {}", e))?;

    // Create initial commit
    Command::new("git")
        .args(&["add", "."])
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to add files: {}", e))?;

    Command::new("git")
        .args(&["commit", "-m", "Gremlin Project init commit"])
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to create initial commit: {}", e))?;

    // Only attempt GitHub repository creation if not private
    if !is_private {
        let github = GitHubCli::new();
        
        if !github.install() {
            println!("{}", "Repository created locally but not pushed to GitHub.".bright_yellow());
            return Ok(());
        }

        if !github.authenticate() {
            println!("{}", "Repository created locally but not pushed to GitHub.".bright_yellow());
            return Ok(());
        }

        if let Err(e) = github.create_repository(project_path, is_private) {
            println!("{} {}", "Failed to create GitHub repository:".bright_red(), e);
            println!("{}", "Repository created locally but not pushed to GitHub.".bright_yellow());
        }
    }

    Ok(())
}

fn display_file_structure(path: &str, prefix: &str) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                
                println!("{}{}{}", prefix, if is_dir { "📁 " } else { "📄 " }, name.bright_cyan());
                
                if is_dir {
                    display_file_structure(&entry.path().to_string_lossy(), &format!("{}  ", prefix));
                }
            }
        }
    }
}

#[allow(dead_code)]
fn create_rust_project(project_name: &str, _template: &str, with_git: bool) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating Rust project '{}'...", project_name).bright_cyan());
    
    // Create project directory
    match Command::new("mkdir")
        .arg("-p")
        .arg(&project_path)
        .status() {
        Ok(_) => {
            // Change to project directory
            if let Err(e) = std::env::set_current_dir(&project_path) {
                println!("{} {}", "Failed to change directory:".bright_red(), e);
                return;
            }
            
            // Initialize cargo project
            match Command::new("cargo")
                .arg("init")
                .status() {
                Ok(_) => {
                    // Modify main.rs content
                    let main_content = r#"fn main() {
    println!("Gremlin Project init commit");
}"#;
                    
                    if let Err(e) = fs::write("src/main.rs", main_content) {
                        println!("{} {}", "Failed to modify main.rs:".bright_red(), e);
                        return;
                    }

                    if with_git {
                        print!("{}", "Is this a private repository? (y/n): ".bright_blue());
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        let is_private = input.trim().to_lowercase() == "y";

                        match initialize_git(&project_path, is_private) {
                            Ok(_) => println!("{}", "✅ Git repository initialized successfully!".bright_green()),
                            Err(e) => println!("{} {}", "Failed to initialize git:".bright_red(), e),
                        }
                    }

                    println!("{}", "✅ Project created successfully!".bright_green());
                    println!("{}", format!("Project location: {}", project_path).bright_cyan());
                    println!("\n{}", "Project structure:".bright_yellow());
                    display_file_structure(&project_path, "");
                },
                Err(e) => println!("{} {}", "Failed to initialize cargo project:".bright_red(), e),
            }
        },
        Err(e) => println!("{} {}", "Failed to create project directory:".bright_red(), e),
    }
}

fn standard_project_options() {
    println!("\n{}", "Standard Rust Project Options:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Create without git".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Create with git and GitHub".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Templates 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-2): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" | "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            if project_name.is_empty() {
                println!("{}", "⚠️ Project name cannot be empty".bright_red());
                return;
            }
            
            create_rust_project(project_name, "standard", input.trim() == "2");
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

fn create_iced_template(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating Iced project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
        format!("{}/src/components", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create Cargo.toml
    let cargo_toml = r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
iced = "0.12.1"
"#;
    
    if let Err(e) = std::fs::write(format!("{}/Cargo.toml", project_path), cargo_toml.replace("{project_name}", project_name)) {
        println!("{} {}", "Failed to create Cargo.toml:".bright_red(), e);
        return;
    }
    
    // Create main.rs
    let main_rs = r#"mod app;
use app::App;

fn main() -> iced::Result {
    App::run(Default::default())
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/main.rs", project_path), main_rs) {
        println!("{} {}", "Failed to create main.rs:".bright_red(), e);
        return;
    }
    
    // Create app.rs
    let app_rs = r#"use iced::{Application, Command, Element, executor};
use crate::messages::Message;

pub struct App {
    // your state here
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (App { }, Command::none())
    }

    fn title(&self) -> String {
        "Rust Gremlin".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonClicked => {
                println!("Gremlin button go brrr");
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::text("Welcome to Rust Gremlin!")
        ]
        .into()
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/app.rs", project_path), app_rs) {
        println!("{} {}", "Failed to create app.rs:".bright_red(), e);
        return;
    }
    
    // Create messages.rs
    let messages_rs = r#"#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked,
    TextChanged(String),
    Tick,
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/messages.rs", project_path), messages_rs) {
        println!("{} {}", "Failed to create messages.rs:".bright_red(), e);
        return;
    }
    
    // Create styles.rs
    let styles_rs = r#"use iced::Color;

pub const PRIMARY: Color = Color::from_rgb(0.2, 0.2, 0.7);
pub const SECONDARY: Color = Color::from_rgb(0.7, 0.2, 0.2);
pub const BACKGROUND: Color = Color::from_rgb(0.9, 0.9, 0.9);"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/styles.rs", project_path), styles_rs) {
        println!("{} {}", "Failed to create styles.rs:".bright_red(), e);
        return;
    }
    
    // Create utils.rs
    let utils_rs = r#"use chrono::{DateTime, Local};

pub fn format_date_time(dt: DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/utils.rs", project_path), utils_rs) {
        println!("{} {}", "Failed to create utils.rs:".bright_red(), e);
        return;
    }
    
    // Create component files
    let sidebar_rs = r#"use iced::Element;
use crate::messages::Message;

pub struct Sidebar {
    // sidebar state
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar { }
    }
    
    pub fn view(&self) -> Element<Message> {
        iced::widget::column![
            iced::widget::text("Sidebar")
        ]
        .into()
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/components/sidebar.rs", project_path), sidebar_rs) {
        println!("{} {}", "Failed to create sidebar.rs:".bright_red(), e);
        return;
    }
    
    let footer_rs = r#"use iced::Element;
use crate::messages::Message;

pub struct Footer {
    // footer state
}

impl Footer {
    pub fn new() -> Self {
        Footer { }
    }
    
    pub fn view(&self) -> Element<Message> {
        iced::widget::row![
            iced::widget::text("Footer")
        ]
        .into()
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/components/footer.rs", project_path), footer_rs) {
        println!("{} {}", "Failed to create footer.rs:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ Iced project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
}

fn create_ratatui_template(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating Ratatui project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create Cargo.toml
    let cargo_toml = r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.26.1"
crossterm = "0.27.0"
anyhow = "1.0.79"
"#;
    
    if let Err(e) = std::fs::write(format!("{}/Cargo.toml", project_path), cargo_toml.replace("{project_name}", project_name)) {
        println!("{} {}", "Failed to create Cargo.toml:".bright_red(), e);
        return;
    }
    
    // Create main.rs
    let main_rs = r#"// Main entry point for the TUI application
use anyhow::Result;
use ratatui::prelude::*;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

mod app;
mod event;
mod terminal;
mod ui;

use app::App;
use terminal::Terminal;

fn main() -> Result<()> {
    // Initialize terminal
    let mut terminal = Terminal::new()?;
    
    // Create app state
    let mut app = App::new();
    
    // Main event loop
    while !app.should_quit {
        // Draw the UI
        terminal.draw(|f| ui::draw(f, &app))?;
        
        // Handle events
        if let Event::Key(key) = event::read()? {
            app.handle_event(key.code);
        }
    }
    
    // Cleanup
    terminal.cleanup()?;
    Ok(())
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/main.rs", project_path), main_rs) {
        println!("{} {}", "Failed to create main.rs:".bright_red(), e);
        return;
    }
    
    // Create app.rs
    let app_rs = r#"// Application state and logic
use crossterm::event::KeyCode;

pub struct App {
    pub should_quit: bool,
    // Add your application state here
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            // Initialize your state here
        }
    }
    
    pub fn handle_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            // Handle other key events here
            _ => {}
        }
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/app.rs", project_path), app_rs) {
        println!("{} {}", "Failed to create app.rs:".bright_red(), e);
        return;
    }
    
    // Create event.rs
    let event_rs = r#"// Event handling module
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum EventResult {
    Continue,
    Quit,
}

pub fn poll_event(timeout: Duration) -> anyhow::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/event.rs", project_path), event_rs) {
        println!("{} {}", "Failed to create event.rs:".bright_red(), e);
        return;
    }
    
    // Create terminal.rs
    let terminal_rs = r#"// Terminal initialization and cleanup
use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{stdout, Stdout};

pub struct Terminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        // Enable raw mode and enter alternate screen
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        // Initialize terminal
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        
        Ok(Self { terminal })
    }
    
    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }
    
    pub fn cleanup(&mut self) -> Result<()> {
        // Disable raw mode and leave alternate screen
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/terminal.rs", project_path), terminal_rs) {
        println!("{} {}", "Failed to create terminal.rs:".bright_red(), e);
        return;
    }
    
    // Create ui.rs
    let ui_rs = r#"// UI rendering module
use ratatui::prelude::*;
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    // Create a layout for the main areas
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(frame.size());
    
    // Draw header
    frame.render_widget(
        Paragraph::new("Gremlin TUI")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        main_layout[0],
    );
    
    // Draw main content
    frame.render_widget(
        Paragraph::new("Press 'q' to quit")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        main_layout[1],
    );
    
    // Draw footer
    frame.render_widget(
        Paragraph::new("Footer")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        main_layout[2],
    );
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/ui.rs", project_path), ui_rs) {
        println!("{} {}", "Failed to create ui.rs:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ Ratatui project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
}

fn create_tauri_template(project_name: &str) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", format!("Creating Tauri project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/src", project_path),
        format!("{}/src/components", project_path),
        format!("{}/public", project_path),
        format!("{}/src-tauri/src", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create package.json
    let package_json = r#"{
  "name": "{project_name}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.0.2",
    "vite": "^5.0.0",
    "@tauri-apps/cli": "^1.5.0"
  }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/package.json", project_path), package_json.replace("{project_name}", project_name)) {
        println!("{} {}", "Failed to create package.json:".bright_red(), e);
        return;
    }
    
    // Create vite.config.js
    let vite_config = r#"// Vite configuration for Tauri app
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: ['es2021', 'chrome100', 'safari13'],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})"#;
    
    if let Err(e) = std::fs::write(format!("{}/vite.config.js", project_path), vite_config) {
        println!("{} {}", "Failed to create vite.config.js:".bright_red(), e);
        return;
    }
    
    // Create index.html
    let index_html = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Gremlin Tauri App</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>"#;
    
    if let Err(e) = std::fs::write(format!("{}/index.html", project_path), index_html) {
        println!("{} {}", "Failed to create index.html:".bright_red(), e);
        return;
    }
    
    // Create App.tsx
    let app_tsx = r#"// Main React component
import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="container">
      <h1>Welcome to Gremlin Tauri!</h1>
      <div className="row">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
    </div>
  )
}

export default App"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/App.tsx", project_path), app_tsx) {
        println!("{} {}", "Failed to create App.tsx:".bright_red(), e);
        return;
    }
    
    // Create main.tsx
    let main_tsx = r#"// Entry point for the React application
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#;
    
    if let Err(e) = std::fs::write(format!("{}/src/main.tsx", project_path), main_tsx) {
        println!("{} {}", "Failed to create main.tsx:".bright_red(), e);
        return;
    }
    
    // Create tauri.conf.json
    let tauri_conf = r#"{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist",
    "withGlobalTauri": true
  },
  "package": {
    "productName": "{project_name}",
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
      "identifier": "com.gremlin.{project_name}",
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
        "title": "Gremlin Tauri",
        "width": 800,
        "height": 600
      }
    ]
  }
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src-tauri/tauri.conf.json", project_path), tauri_conf.replace("{project_name}", project_name)) {
        println!("{} {}", "Failed to create tauri.conf.json:".bright_red(), e);
        return;
    }
    
    // Create main.rs
    let main_rs = r#"// Tauri backend entry point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Prevents additional console window on Windows in release, DO NOT REMOVE!!

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/src-tauri/src/main.rs", project_path), main_rs) {
        println!("{} {}", "Failed to create main.rs:".bright_red(), e);
        return;
    }
    
    // Create Cargo.toml for Tauri
    let cargo_toml = r#"[package]
name = "{project_name}"
version = "0.1.0"
description = "A Tauri App"
authors = ["Gremlin"]
edition = "2021"
rust-version = "1.60"

[build-dependencies]
tauri-build = { version = "1.5.0", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
tauri = { version = "1.5.0", features = [] }

[features]
custom-protocol = [ "tauri/custom-protocol" ]"#;
    
    if let Err(e) = std::fs::write(format!("{}/src-tauri/Cargo.toml", project_path), cargo_toml.replace("{project_name}", project_name)) {
        println!("{} {}", "Failed to create Cargo.toml:".bright_red(), e);
        return;
    }
    
    println!("{}", "✅ Tauri project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
    println!("\n{}", "Project structure:".bright_yellow());
    display_file_structure(&project_path, "");
    println!("\n{}", "To get started, run:".bright_yellow());
    println!("{}", format!("cd {}", project_name).bright_cyan());
    println!("{}", "npm install".bright_cyan());
    println!("{}", "npm run tauri dev".bright_cyan());
}

fn template_dashboard() {
    println!("\n{}", "Rust Project Templates:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Standard Rust Project".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Iced GUI Project".bright_magenta());
    println!("{} {}", "3.".bright_green(), "Ratatui TUI Project".bright_magenta());
    println!("{} {}", "4.".bright_green(), "Tauri Desktop App".bright_magenta());
    println!("{} {}", "5.".bright_green(), "Template 4".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Rust Dashboard 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-5): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" => standard_project_options(),
        "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            if project_name.is_empty() {
                println!("{}", "⚠️ Project name cannot be empty".bright_red());
                return;
            }
            
            create_iced_template(project_name);
        },
        "3" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            if project_name.is_empty() {
                println!("{}", "⚠️ Project name cannot be empty".bright_red());
                return;
            }
            
            create_ratatui_template(project_name);
        },
        "4" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            if project_name.is_empty() {
                println!("{}", "⚠️ Project name cannot be empty".bright_red());
                return;
            }
            
            create_tauri_template(project_name);
        },
        "5" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            if project_name.is_empty() {
                println!("{}", "⚠️ Project name cannot be empty".bright_red());
                return;
            }
            
            create_rust_project(project_name, input.trim(), false);
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

pub fn run() {
    let splash_art = load_ascii("ascii/RustGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🦀 Rust Project Builder 🦀".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Rust Options:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Back to Project Builder 🔙".bright_blue());
        println!("{} {}", "3.".bright_green(), "Back to Gremlin Dashboard 🏠".bright_magenta());

        print!("\n{}", "Enter choice (1-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => template_dashboard(),
            "2" => return,
            "3" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 