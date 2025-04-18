use std::io::{self, Write};
use std::fs;
use std::process::Command;
use colored::*;

use crate::sys_admin::command_matcher;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn check_command(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn install_requirements() -> bool {
    println!("\n{}", "Installing required packages...".bright_yellow());
    println!("{}", "This may take a few minutes...".bright_yellow());
    println!("{}", "Please enter your password when prompted".bright_yellow());
    println!("{}", "----------------------------------------".bright_yellow());
    
    let status = if check_command("apt") {
        println!("{}", "Using apt package manager...".bright_cyan());
        Command::new("sudo")
            .arg("apt")
            .arg("update")
            .status()
            .and_then(|_| {
                Command::new("sudo")
                    .arg("apt")
                    .arg("install")
                    .arg("-y")
                    .args(&["sysstat", "htop", "nmap", "curl", "dnsutils", "fail2ban", "clamav", "gnupg", "openssl"])
                    .status()
            })
    } else if check_command("pacman") {
        println!("{}", "Using pacman package manager...".bright_cyan());
        Command::new("sudo")
            .arg("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .status()
            .and_then(|_| {
                Command::new("sudo")
                    .arg("pacman")
                    .arg("-S")
                    .arg("--noconfirm")
                    .args(&["sysstat", "htop", "nmap", "curl", "bind-tools", "fail2ban", "clamav", "gnupg", "openssl"])
                    .status()
            })
    } else {
        println!("{}", "Unsupported package manager".bright_red());
        return false;
    };
    
    println!("{}", "----------------------------------------".bright_yellow());
    match status {
        Ok(s) if s.success() => {
            println!("{}", "Installation complete!".bright_green());
            true
        },
        _ => {
            println!("{}", "Installation failed. Please try again.".bright_red());
            false
        }
    }
}

pub fn run() {
    let splash_art = load_ascii("ascii/resourceGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📊 Resource Monitoring Dashboard 📊".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "Available Commands:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Memory Usage".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Virtual Memory Stats".bright_magenta());
        println!("{} {}", "3.".bright_green(), "I/O Statistics".bright_magenta());
        println!("{} {}", "4.".bright_green(), "CPU Statistics".bright_magenta());
        println!("{} {}", "5.".bright_green(), "System Activity Report".bright_magenta());
        println!("\n{} {}", "0.".bright_green(), "Back to System Admin Dashboard 🔙".bright_blue());

        print!("\n{}", "Enter choice (0-5): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "0" => return,
            "1" => command_matcher::run_command("free", ""),
            "2" => command_matcher::run_command("vmstat", "1 5"),
            "3" => {
                if !check_command("iostat") && !install_requirements() {
                    continue;
                }
                command_matcher::run_command("iostat", "1 5");
            },
            "4" => {
                if !check_command("mpstat") && !install_requirements() {
                    continue;
                }
                command_matcher::run_command("mpstat", "1 5");
            },
            "5" => {
                if !check_command("sar") && !install_requirements() {
                    continue;
                }
                command_matcher::run_command("sar", "1 5");
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
} 