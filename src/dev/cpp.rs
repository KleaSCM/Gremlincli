use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/CppGirl.txt");
    println!("{}", splash_art.bright_yellow());
    println!("{}", "🏗️ C++ Project Dashboard 🏗️".bright_yellow().bold().blink());
    
    loop {
        println!("\n{}", "C++ Project Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project 🏗️".bright_yellow());
        println!("{} {}", "2.".bright_green(), "Return to Project Builder 🔙".bright_blue());

        print!("\n{}", "Enter your choice (1-2): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => create_new_project(),
            "2" => {
                println!("{}", "Returning to Project Builder...".bright_blue());
                return;
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}

fn create_new_project() {
    println!("\n{}", "C++ Project Templates:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Template 1".bright_yellow());
    println!("{} {}", "2.".bright_green(), "Template 2".bright_yellow());
    println!("{} {}", "3.".bright_green(), "Template 3".bright_yellow());
    println!("{} {}", "4.".bright_green(), "No Template".bright_yellow());
    println!("{} {}", "5.".bright_green(), "Back to C++ Dashboard 🔙".bright_blue());

    print!("\n{}", "Select template (1-5): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => println!("{}", "Creating C++ project with Template 1... 🏗️".bright_yellow()),
        "2" => println!("{}", "Creating C++ project with Template 2... 🏗️".bright_yellow()),
        "3" => println!("{}", "Creating C++ project with Template 3... 🏗️".bright_yellow()),
        "4" => println!("{}", "Creating C++ project with no template... 🏗️".bright_yellow()),
        "5" => return,
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}