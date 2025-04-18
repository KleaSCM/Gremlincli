use std::io::{self, Write};
use std::fs;
use colored::*;

use crate::sys_admin::command_matcher;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn is_valid_ip(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|&part| part.parse::<u8>().is_ok())
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn run() {
    let splash_art = load_ascii("ascii/SysGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🌐 Networking Tools Dashboard 🌐".bright_purple().bold().blink());
    
    let commands = [
        ("ip", "Show network interfaces"),
        ("ping", "Test network connectivity"),
        ("traceroute", "Trace network path"),
        ("netstat", "Network statistics"),
        ("nmap", "Network scanner"),
        ("curl", "Transfer data from URLs"),
        ("dig", "DNS lookup utility"),
    ];

    loop {
        println!("\n{}", "Available Commands:".bright_cyan());
        for (i, (cmd, desc)) in commands.iter().enumerate() {
            println!("{} {} {} {} {}",
                format!("{}.", i + 1).bright_green(),
                cmd.bright_magenta(),
                "–".bright_blue(),
                desc.bright_cyan(),
                if *cmd == "ip" || *cmd == "netstat" { "(No input needed)" } else { "" }
            );
        }
        println!("\n{} {}", "0.".bright_green(), "Back to System Admin Dashboard 🔙".bright_blue());

        let choice = get_input("\nEnter command number (0-7): ");
        
        match choice.parse::<usize>() {
            Ok(0) => return,
            Ok(n) if n <= commands.len() => {
                let (cmd, _) = commands[n - 1];
                match cmd {
                    "ip" | "netstat" => {
                        command_matcher::run_command(cmd, "");
                    },
                    "ping" | "traceroute" | "nmap" | "dig" => {
                        let target = get_input("Enter target (IP or URL): ");
                        if is_valid_ip(&target) || is_valid_url(&target) {
                            command_matcher::run_command(cmd, &target);
                        } else {
                            println!("{}", "Invalid target format. Please enter a valid IP or URL.".bright_red());
                        }
                    },
                    "curl" => {
                        let url = get_input("Enter URL: ");
                        if is_valid_url(&url) {
                            command_matcher::run_command(cmd, &url);
                        } else {
                            println!("{}", "Invalid URL format. Please enter a valid URL starting with http:// or https://".bright_red());
                        }
                    },
                    _ => println!("{}", "Invalid command".bright_red()),
                }
            },
            _ => println!("{}", "Invalid choice. Please enter a number between 0 and 7.".bright_red()),
        }
    }
} 