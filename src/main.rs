mod project_builder;
mod script_runner;
mod sys_admin;

use std::io::{self, Write};
use std::fs;

/// Load ASCII art from a file
fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn splash() {
    let splash_art = load_ascii("ascii/gremlin_main_girl.txt");
    println!("{}", splash_art);
}

fn main_menu() {
    println!("\n✨ Welcome, Gremlin Queen ✨");
    println!("Choose your mischief:");
    println!("1. Project Builder");
    println!("2. Script Runner");
    println!("3. System Admin Gremlin");

    print!("\nEnter choice (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => project_builder::run(),
        "2" => script_runner::run(),
        "3" => sys_admin::run(),
        _ => println!("⚠️ Invalid choice. Gremlin.exe had a moment."),
    }
}

fn main() {
    splash();
    main_menu();
}
