use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/RustGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("{}", "🦀 Rust Project Dashboard 🦀".bright_magenta().bold().blink());
    
    loop {
        println!("\n{}", "Rust Project Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project 🏗️".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Return to Project Builder 🔙".bright_blue());
        println!("{} {}", "3.".bright_green(), "Return to Gremlin Dashboard 🏠".bright_purple());

        print!("\n{}", "Enter your choice (1-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => create_new_project(),
            "2" => {
                println!("{}", "Returning to Project Builder...".bright_blue());
                return;
            },
            "3" => {
                println!("{}", "Returning to Gremlin Dashboard...".bright_purple());
                return;
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}

fn create_new_project() {
    println!("\n{}", "Rust Project Templates:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Template 1".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Template 2".bright_magenta());
    println!("{} {}", "3.".bright_green(), "Template 3".bright_magenta());
    println!("{} {}", "4.".bright_green(), "No Template".bright_magenta());
    println!("{} {}", "5.".bright_green(), "Back to Rust Dashboard 🔙".bright_blue());

    print!("\n{}", "Select template (1-5): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => println!("{}", "Creating Rust project with Template 1... 🦀".bright_magenta()),
        "2" => println!("{}", "Creating Rust project with Template 2... 🦀".bright_magenta()),
        "3" => println!("{}", "Creating Rust project with Template 3... 🦀".bright_magenta()),
        "4" => println!("{}", "Creating Rust project with no template... 🦀".bright_magenta()),
        "5" => return,
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
} 