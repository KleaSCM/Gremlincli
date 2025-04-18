mod command_matcher;

use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/SysGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("{}", "👷‍♀️ System Admin Dashboard 👷‍♀️".bright_purple().bold().blink());
    
    command_matcher::run();
}

fn create_new_project() {
    println!("{}", "System information: ℹ️".bright_magenta());
    // TODO: Implement project creation
}

fn show_templates() {
    println!("{}", "Process management: 🔄".bright_cyan());
    // TODO: Implement template listing
}

fn show_settings() {
    println!("{}", "Network tools: 🌐".bright_purple());
    // TODO: Implement settings management
} 