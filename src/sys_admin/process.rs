use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::sys_admin::command_matcher;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/processGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🧵 Process Management Dashboard 🧵".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Available Commands:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Show Running Processes".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Interactive Process Viewer".bright_magenta());
        println!("{} {}", "3.".bright_green(), "Kill Process".bright_magenta());
        println!("{} {}", "4.".bright_green(), "Force Kill Process".bright_magenta());
        println!("{} {}", "5.".bright_green(), "Kill by Name".bright_magenta());
        println!("{} {}", "6.".bright_green(), "Find by Name".bright_magenta());
        println!("\n{} {}", "0.".bright_green(), "Back to System Admin Dashboard 🔙".bright_blue());

        print!("\n{}", "Enter choice (0-6): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "0" => return,
            "1" => command_matcher::run_command("ps", ""),
            "2" => command_matcher::run_command("htop", ""),
            "3" | "4" | "5" | "6" => {
                let prompt = match input.trim() {
                    "3" => "Enter PID to kill: ",
                    "4" => "Enter PID to force kill: ",
                    "5" => "Enter process name to kill: ",
                    "6" => "Enter process name to find: ",
                    _ => unreachable!(),
                };
                print!("{}", prompt.bright_blue());
                io::stdout().flush().unwrap();
                let mut arg = String::new();
                io::stdin().read_line(&mut arg).unwrap();
                let cmd = match input.trim() {
                    "3" => "kill",
                    "4" => "kill9",
                    "5" => "pkill",
                    "6" => "pgrep",
                    _ => unreachable!(),
                };
                command_matcher::run_command(cmd, arg.trim());
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 