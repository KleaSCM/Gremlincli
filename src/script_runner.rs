use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/ScriptGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("{}", "👷‍♀️ Launching Script Runner!".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Script Runner Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Script 📝".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Run Script 🚀".bright_cyan());
        println!("{} {}", "3.".bright_green(), "Build Script 🔨".bright_purple());
        println!("{} {}", "4.".bright_green(), "Back to Main Menu 🔙".bright_blue());

        print!("\n{}", "Enter your choice (1–4): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => create_new_project(),
            "2" => show_templates(),
            "3" => show_settings(),
            "4" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}

fn create_new_project() {
    println!("{}", "Creating new Script... 📝".bright_magenta());
    // TODO: Implement project creation
}

fn show_templates() {
    println!("{}", "Available templates: 📋".bright_cyan());
    // TODO: Implement template listing
}

fn show_settings() {
    println!("{}", "Project settings: ⚙️".bright_purple());
    // TODO: Implement settings management
} 