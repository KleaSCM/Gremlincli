use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::dev::rust;
use crate::dev::go;
use crate::dev::typescript;
use crate::dev::cpp;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run() {
    let splash_art = load_ascii("ascii/projectbuilderGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("{}", "👷‍♀️ Launching Project Builder!".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Project Builder Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Rust Projects 🦀".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Go Projects 🐹".bright_cyan());
        println!("{} {}", "3.".bright_green(), "Next.js Projects ⚛️".bright_purple());
        println!("{} {}", "4.".bright_green(), "C++ Projects 🏗️".bright_yellow());
        println!("{} {}", "5.".bright_green(), "Back to Main Menu 🔙".bright_blue());

        print!("\n{}", "Enter your choice (1–5): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => rust::run(),
            "2" => go::run(),
            "3" => typescript::run(),
            "4" => cpp::run(),
            "5" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}
