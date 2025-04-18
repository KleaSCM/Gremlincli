use std::io::{self, Write};
use std::fs;
use colored::*;

mod run_script;
mod create_script;
mod search;
mod header;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/ScriptRunnerGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📜 Script Runner Dashboard 📜".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Script Management:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create Script 📝".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Search Scripts 🔍".bright_cyan());
        println!("{} {}", "3.".bright_green(), "Run Script 🚀".bright_magenta());
        println!("{} {}", "0.".bright_green(), "Back to Gremlin Dashboard 🏠".bright_blue());

        print!("\n{}", "Enter your choice (0-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => create_script::run(),
            "2" => search::run(),
            "3" => run_script::run(),
            "0" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 