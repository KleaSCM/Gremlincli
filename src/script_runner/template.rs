use std::io::{self, Write};
use std::fs;
use colored::*;

pub fn run() {
    let splash_art = load_ascii("ascii/TemplateGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📋 Script Templates 📋".bright_purple().bold().blink());
    
    // TODO: Implement template selection and application
}

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
} 