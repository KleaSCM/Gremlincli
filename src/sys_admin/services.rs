use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::sys_admin::command_matcher;
use crate::sys_admin::command_types::CommandCategory;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("/home/klea/Documents/Dev/Gremlincli/ascii/Sysgirl6.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "⚙️ Service Management Dashboard ⚙️".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Service Commands:".bright_cyan());
        command_matcher::show_commands_by_category(CommandCategory::ServiceManagement);
        println!("\n{} {}", "1.".bright_green(), "Back to System Admin Dashboard 🔙".bright_blue());
        println!("{} {}", "2.".bright_green(), "Back to Gremlin Dashboard 🏠".bright_magenta());

        print!("\n{}", "Enter command or option (1-2): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => return,
            "2" => std::process::exit(0),
            cmd => {
                let mut arg = String::new();
                if cmd != "systemctl-status" && cmd != "systemctl-list" {
                    println!("{}", "Enter service name:".bright_blue());
                    io::stdin().read_line(&mut arg).unwrap();
                }
                command_matcher::run_command(cmd, arg.trim());
            }
        }
    }
} 