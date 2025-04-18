use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::sys_admin::command_matcher;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/logsGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📜 Log Diving Dashboard 📜".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Available Commands:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "System Journal Logs".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Service Logs".bright_magenta());
        println!("{} {}", "3.".bright_green(), "Kernel Messages".bright_magenta());
        println!("{} {}", "4.".bright_green(), "System Log".bright_magenta());
        println!("{} {}", "5.".bright_green(), "Auth Log".bright_magenta());
        println!("\n{} {}", "0.".bright_green(), "Back to System Admin Dashboard 🔙".bright_blue());

        print!("\n{}", "Enter choice (0-5): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "0" => return,
            "1" => command_matcher::run_command("journalctl", ""),
            "2" => command_matcher::run_command("journalctl-service", ""),
            "3" => command_matcher::run_command("dmesg", ""),
            "4" => command_matcher::run_command("tail-syslog", ""),
            "5" => command_matcher::run_command("auth-log", ""),
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 