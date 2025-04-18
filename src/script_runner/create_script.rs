use std::io::{self, Write};
use std::fs;
use colored::*;
use crate::script_runner::header;
use crate::script_runner::template;

#[allow(dead_code)]
fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

#[allow(dead_code)]
fn ensure_script_dirs() -> Result<(), String> {
    let base_path = "/home/klea/Documents/Scripts/";
    let dirs = ["Python", "Rust", "Bash", "Go", "Lua", "PS1"];

    for dir in dirs.iter() {
        let path = format!("{}{}", base_path, dir);
        fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create directory {}: {}", dir, e))?;
    }

    Ok(())
}

#[allow(dead_code)]
pub fn run() {
    let splash_art = load_ascii("ascii/CreateScriptGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📝 Create New Script 📝".bright_purple().bold().blink());
    
    // Ensure script directories exist
    if let Err(e) = ensure_script_dirs() {
        println!("{} {}", "Error:".bright_red(), e);
        return;
    }

    loop {
        println!("\n{}", "Script Types:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Python Script".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Rust Script".bright_magenta());
        println!("{} {}", "3.".bright_green(), "Bash Script".bright_magenta());
        println!("{} {}", "4.".bright_green(), "Go Script".bright_magenta());
        println!("{} {}", "5.".bright_green(), "Lua Script".bright_magenta());
        println!("{} {}", "6.".bright_green(), "PowerShell Script".bright_magenta());
        println!("{} {}", "7.".bright_green(), "Back to Script Runner 🔙".bright_blue());

        print!("\n{}", "Enter choice (1-7): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" | "2" | "3" | "4" | "5" | "6" => {
                print!("{}", "Enter script name: ".bright_blue());
                io::stdout().flush().unwrap();
                let mut script_name = String::new();
                io::stdin().read_line(&mut script_name).unwrap();
                let script_name = script_name.trim();

                let (dir, extension, language) = match input.trim() {
                    "1" => ("Python", "py", "Python"),
                    "2" => ("Rust", "rs", "Rust"),
                    "3" => ("Bash", "sh", "Bash"),
                    "4" => ("Go", "go", "Go"),
                    "5" => ("Lua", "lua", "Lua"),
                    "6" => ("PS1", "ps1", "PS1"),
                    _ => unreachable!(),
                };

                let script_path = format!("/home/klea/Documents/Scripts/{}/{}.{}", dir, script_name, extension);
                let header = header::generate_header(&format!("{}.{}", script_name, extension), language);
                let template = template::get_template(language).unwrap_or_default();
                let content = format!("{}\n{}", header, template);
                
                fs::write(&script_path, content)
                    .map_err(|e| format!("Failed to create script: {}", e))
                    .and_then(|_| {
                        if dir == "Bash" {
                            std::process::Command::new("chmod")
                                .arg("+x")
                                .arg(&script_path)
                                .status()
                                .map_err(|e| format!("Failed to make script executable: {}", e))?;
                        }
                        Ok(())
                    })
                    .unwrap_or_else(|e| println!("{} {}", "Error:".bright_red(), e));

                println!("{}", "✅ Script created successfully!".bright_green());
                println!("{} {}", "Script path:".bright_cyan(), script_path.bright_blue());
            },
            "7" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 