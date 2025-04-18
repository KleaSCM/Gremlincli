use crate::sys_admin::command_logic;
use crate::sys_admin::command_names;
use crate::sys_admin::command_categories;
use crate::sys_admin::process;
use crate::sys_admin::resource;
use crate::sys_admin::disk;
use crate::sys_admin::logs;
use crate::sys_admin::networking;
use crate::sys_admin::services;
use crate::sys_admin::security;
use crate::sys_admin::packages;

use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run_command(cmd: &str, arg: &str) {
    match cmd {
        // Process Management
        "ps" => command_logic::ps(),
        "htop" => command_logic::htop(),
        "kill" => command_logic::kill_graceful(arg),
        "kill9" => command_logic::kill_force(arg),
        "pkill" => command_logic::pkill(arg),
        "pgrep" => command_logic::pgrep(arg),

        // Resource Monitoring
        "free" => command_logic::free(),
        "vmstat" => command_logic::vmstat(),
        "iostat" => command_logic::iostat(),
        "mpstat" => command_logic::mpstat(),
        "sar" => command_logic::sar(),

        // Disk and Storage
        "df" => command_logic::df(),
        "du" => command_logic::du(arg),
        "ncdu" => command_logic::ncdu(arg),
        "lsblk" => command_logic::lsblk(),
        "mount" => command_logic::mount(),
        "umount" => command_logic::umount(arg),

        // Log Diving
        "journalctl" => command_logic::journalctl_system(),
        "journalctl-service" => command_logic::journalctl_service(arg),
        "dmesg" => command_logic::dmesg(),
        "tail-syslog" => command_logic::tail_syslog(),
        "auth-log" => command_logic::less_auth_log(),

        // Networking Ninja Tools
        "ip" => command_logic::ip_a(),
        "ping" => command_logic::ping(arg),
        "traceroute" => command_logic::traceroute(arg),
        "netstat" => command_logic::netstat(),
        "nmap" => command_logic::nmap(arg),
        "curl" => command_logic::curl(arg),
        "dig" => command_logic::dig(arg),

        // Service Management
        "systemctl-start" => {
            println!("{} Starting service...", "🚀".yellow());
            let service = get_arg("Enter service name: ");
            command_logic::systemctl_start(&service)
        },
        "systemctl-stop" => {
            println!("{} Stopping service...", "🛑".red());
            let service = get_arg("Enter service name: ");
            command_logic::systemctl_stop(&service)
        },
        "systemctl-restart" => {
            println!("{} Restarting service...", "🔄".cyan());
            let service = get_arg("Enter service name: ");
            command_logic::systemctl_restart(&service)
        },
        "systemctl-enable" => {
            println!("{} Enabling service...", "✅".green());
            let service = get_arg("Enter service name: ");
            command_logic::systemctl_enable(&service)
        },
        "systemctl-disable" => {
            println!("{} Disabling service...", "❌".red());
            let service = get_arg("Enter service name: ");
            command_logic::systemctl_disable(&service)
        },
        "systemctl-status" => {
            println!("{} Checking system status...", "📊".blue());
            command_logic::systemctl_status()
        },
        "systemctl-list" => {
            println!("{} Listing all services...", "📋".yellow());
            command_logic::systemctl_list()
        },

        // Security Scans / Secrets Ops
        "chkrootkit" => command_logic::chkrootkit(),
        "rkhunter" => command_logic::rkhunter(),
        "lynis" => command_logic::lynis(),
        "clamscan" => command_logic::clamscan(arg),
        "fail2ban" => command_logic::fail2ban_status(),
        "gpg-encrypt" => {
            println!("{}", "Enter recipient email/key ID:".bright_blue());
            let mut recipient = String::new();
            io::stdin().read_line(&mut recipient).unwrap();
            command_logic::gpg_encrypt(arg, recipient.trim());
        },
        "gpg-decrypt" => command_logic::gpg_decrypt(arg),
        "openssl-encrypt" => {
            println!("{}", "Enter encryption password:".bright_blue());
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            command_logic::openssl_encrypt(arg, password.trim());
        },
        "openssl-decrypt" => {
            println!("{}", "Enter decryption password:".bright_blue());
            let mut password = String::new();
            io::stdin().read_line(&mut password).unwrap();
            command_logic::openssl_decrypt(arg, password.trim());
        },

        // Package Gremlin
        "apt-install" => command_logic::apt_install(arg),
        "apt-remove" => command_logic::apt_remove(arg),
        "apt-update" => command_logic::apt_update(),
        "pacman-install" => command_logic::pacman_install(arg),
        "pacman-remove" => command_logic::pacman_remove(arg),
        "pacman-update" => command_logic::pacman_update(),
        "yay-install" => command_logic::yay_install(arg),
        "which" => command_logic::which(arg),
        "whereis" => command_logic::whereis(arg),
        "find-binary" => command_logic::find_binary(arg),
        "locate-binary" => command_logic::locate_binary(arg),
        "dpkg-list" => command_logic::dpkg_list(),
        "pacman-list" => command_logic::pacman_list(),

        _ => println!("{} {}", "Unknown command:".bright_red(), cmd),
    }
}

pub fn show_available_commands() {
    let names = command_names::get_command_names();
    let cats = command_categories::get_command_categories();

    println!("\n{}", "Available Commands:".bright_cyan().bold());
    for (cmd, desc) in &names {
        let cat = cats.get(cmd).unwrap_or(&"Uncategorized");
        println!("{} {} {} {} {}",
            "•".bright_green(),
            cmd.bright_magenta(),
            format!("({})", cat).bright_yellow(),
            "–".bright_blue(),
            desc.bright_cyan()
        );
    }
}

pub fn run() {
    let splash_art = load_ascii("ascii/SysGirl.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "🛠️ System Admin Dashboard 🛠️".bright_purple().bold().blink());
    
    loop {
        println!("\n{}", "System Admin Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Process Management 🧵".bright_magenta());
        println!("{} {}", "2.".bright_green(), "Resource Monitoring 📊".bright_cyan());
        println!("{} {}", "3.".bright_green(), "Disk and Storage 💾".bright_purple());
        println!("{} {}", "4.".bright_green(), "Log Diving 📜".bright_yellow());
        println!("{} {}", "5.".bright_green(), "Networking Tools 🌐".bright_blue());
        println!("{} {}", "6.".bright_green(), "Service Management ⚙️".bright_magenta());
        println!("{} {}", "7.".bright_green(), "Security Tools 🔒".bright_cyan());
        println!("{} {}", "8.".bright_green(), "Package Management 📦".bright_purple());
        println!("{} {}", "9.".bright_green(), "Show All Commands 📋".bright_yellow());
        println!("{} {}", "0.".bright_green(), "Back to Main Menu 🔙".bright_blue());

        print!("\n{}", "Enter your choice (0-9): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => process::run(),
            "2" => resource::run(),
            "3" => disk::run(),
            "4" => logs::run(),
            "5" => networking::run(),
            "6" => services::run(),
            "7" => security::run(),
            "8" => packages::run(),
            "9" => show_available_commands(),
            "0" => return,
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}

pub fn show_commands_by_category(category: &str) {
    let names = command_names::get_command_names();
    let cats = command_categories::get_command_categories();

    for (cmd, desc) in &names {
        if cats.get(cmd).unwrap_or(&"") == &category {
            println!("{} {} {} {}",
                "•".bright_green(),
                cmd.bright_magenta(),
                "–".bright_blue(),
                desc.bright_cyan()
            );
        }
    }
}

fn get_arg(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
} 