mod project_builder;
mod script_runner;
pub mod sys_admin;
mod dev;

use std::io::{self, Write};
use std::fs;
use colored::*;

/// Load ASCII art from a file
fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn main_menu() {
    loop {
        let splash_art = load_ascii("ascii/gremlin_main_girl.txt");
        println!("{}", splash_art.bright_magenta());
        println!("\n{}", "✨ Welcome, Gremlin Queen ✨".bright_purple().bold().blink());
        println!("{}", "Choose your mischief:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Project Builder 🏗️".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Script Runner 📜".bright_cyan());
        println!("{} {}", "3.".bright_green(), "System Admin ⚙️".bright_magenta());

        print!("\n{}", "Enter choice (1-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => project_builder::run(),
            "2" => script_runner::run(),
            "3" => sys_admin::run(),
            _ => println!("{}", "⚠️ Invalid choice. Gremlin.exe had a moment.".bright_red()),
        }
    }
}

fn main() {
    main_menu();
}
