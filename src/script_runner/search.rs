use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn list_scripts(dir: &str) -> Vec<String> {
    let path = format!("/home/klea/Documents/Scripts/{}", dir);
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default()
}

pub fn run() {
    let splash_art = load_ascii("ascii/SearchGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🔍 Script Search 🔍".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Script Types:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Python Scripts".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Rust Scripts".bright_magenta());
        println!("{} {}", "3.".bright_green(), "Bash Scripts".bright_magenta());
        println!("{} {}", "4.".bright_green(), "Go Scripts".bright_magenta());
        println!("{} {}", "5.".bright_green(), "Lua Scripts".bright_magenta());
        println!("{} {}", "6.".bright_green(), "PowerShell Scripts".bright_magenta());
        println!("{} {}", "7.".bright_green(), "Back to Script Runner 🔙".bright_blue());

        print!("\n{}", "Enter choice (1-7): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" | "2" | "3" | "4" | "5" | "6" => {
                let dir = match input.trim() {
                    "1" => "Python",
                    "2" => "Rust",
                    "3" => "Bash",
                    "4" => "Go",
                    "5" => "Lua",
                    "6" => "PS1",
                    _ => unreachable!(),
                };

                let scripts = list_scripts(dir);
                if scripts.is_empty() {
                    println!("{}", "No scripts found in this directory.".bright_yellow());
                    continue;
                }

                println!("\n{}", format!("Available {} scripts:", dir).bright_cyan());
                for (i, script) in scripts.iter().enumerate() {
                    println!("{} {}", format!("{}.", i + 1).bright_green(), script.bright_magenta());
                }
            },
            "7" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 