use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::sys_admin::networking;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/SysGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("{}", "🛠️ System Admin Dashboard 🛠️".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "System Admin Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Process Management 🧵".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Resource Monitoring 📊".bright_cyan());
        println!("{} {}", "3.".bright_green(), "Disk and Storage 💾".bright_purple());
        println!("{} {}", "4.".bright_green(), "Log Diving 📜".bright_yellow());
        println!("{} {}", "5.".bright_green(), "Networking Tools 🌐".bright_blue());
        println!("{} {}", "6.".bright_green(), "Service Management ⚙️".bright_magenta());
        println!("{} {}", "7.".bright_green(), "Security Tools 🔒".bright_cyan());
        println!("{} {}", "8.".bright_green(), "Package Management 📦".bright_purple());
        println!("{} {}", "0.".bright_green(), "Back to Gremlin Dashboard 🏠".bright_blue());

        print!("\n{}", "Enter your choice (0-8): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "5" => networking::run(),
            "0" => return,
            _ => println!("{}", "⚠️ Category not implemented yet. Try networking (5)!".bright_red()),
        }
    }
} 