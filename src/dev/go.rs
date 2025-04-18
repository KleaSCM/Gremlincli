use std::io::{self, Write};
use std::fs;
use colored::*;
use crate::dev::project_setup;
use crate::dev::template;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/GoGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🐹 Go Project Builder 🐹".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Go Options:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Back to Project Builder 🔙".bright_blue());
        println!("{} {}", "3.".bright_green(), "Back to Gremlin Dashboard 🏠".bright_magenta());

        print!("\n{}", "Enter choice (1-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                print!("{}", "Enter project name: ".bright_blue());
                io::stdout().flush().unwrap();
                let mut project_name = String::new();
                io::stdin().read_line(&mut project_name).unwrap();
                let project_name = project_name.trim();

                println!("\n{}", "Project Options:".bright_cyan());
                println!("{} {}", "1.".bright_green(), "Create with GitHub (Public)".bright_magenta());
                println!("{} {}", "2.".bright_green(), "Create with GitHub (Private)".bright_magenta());
                println!("{} {}", "3.".bright_green(), "Create without GitHub".bright_magenta());
                
                print!("\n{}", "Enter choice (1-3): ".bright_blue());
                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();

                let (with_github, is_private) = match choice.trim() {
                    "1" => (true, false),
                    "2" => (true, true),
                    "3" => (false, false),
                    _ => {
                        println!("{}", "⚠️ Invalid choice.".bright_red());
                        continue;
                    }
                };

                println!("\n{}", "Template Options:".bright_cyan());
                println!("{} {}", "1.".bright_green(), "Standard Go Project".bright_magenta());
                println!("{} {}", "2.".bright_green(), "Gin Web Framework".bright_magenta());
                println!("{} {}", "3.".bright_green(), "Echo Web Framework".bright_magenta());
                println!("{} {}", "4.".bright_green(), "Cobra CLI Project".bright_magenta());
                
                print!("\n{}", "Enter template choice (1-4): ".bright_blue());
                io::stdout().flush().unwrap();
                let mut template_choice = String::new();
                io::stdin().read_line(&mut template_choice).unwrap();

                let result = match template_choice.trim() {
                    "1" => project_setup::create_go_project(project_name, with_github, is_private),
                    "2" => template::create_gin_template(project_name, is_private),
                    "3" => template::create_echo_template(project_name, is_private),
                    "4" => template::create_cobra_template(project_name, is_private),
                    _ => {
                        println!("{}", "⚠️ Invalid template choice.".bright_red());
                        continue;
                    }
                };

                if let Err(e) = result {
                    println!("{} {}", "Error:".bright_red(), e);
                    continue;
                }
                
                println!("{}", "✅ Project created successfully!".bright_green());
                println!("{} {}", "Project path:".bright_cyan(), format!("/home/klea/Documents/Dev/{}", project_name).bright_blue());
            },
            "2" => return,
            "3" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 