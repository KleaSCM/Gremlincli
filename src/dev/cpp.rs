use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/CppGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🏗️ C++ Project Builder 🏗️".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "C++ Options:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Back to Project Builder 🔙".bright_blue());

        print!("\n{}", "Enter choice (1-2): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => println!("{}", "C++ project creation coming soon!".bright_yellow()),
            "2" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 