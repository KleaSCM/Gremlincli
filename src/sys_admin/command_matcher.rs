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
use crate::sys_admin::command_types::{CommandName, CommandCategory, COMMAND_MAP};

use std::io::{self, Write};
use std::fs;
use colored::*;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn run_command(cmd: &str, arg: &str) {
    let command = match cmd {
        // Process Management
        "ps" => Some(CommandName::Ps),
        "htop" => Some(CommandName::Htop),
        "kill" => Some(CommandName::KillGraceful),
        "kill9" => Some(CommandName::KillForce),
        "pkill" => Some(CommandName::Pkill),
        "pgrep" => Some(CommandName::Pgrep),

        // Resource Monitoring
        "free" => Some(CommandName::Free),
        "vmstat" => Some(CommandName::Vmstat),
        "iostat" => Some(CommandName::Iostat),
        "mpstat" => Some(CommandName::Mpstat),
        "sar" => Some(CommandName::Sar),

        // Disk and Storage
        "df" => Some(CommandName::Df),
        "du" => Some(CommandName::Du),
        "ncdu" => Some(CommandName::Ncdu),
        "lsblk" => Some(CommandName::Lsblk),
        "mount" => Some(CommandName::Mount),
        "umount" => Some(CommandName::Umount),

        // Log Diving
        "journalctl" => Some(CommandName::JournalctlSystem),
        "journalctl-service" => Some(CommandName::JournalctlService),
        "dmesg" => Some(CommandName::Dmesg),
        "tail-syslog" => Some(CommandName::TailSyslog),
        "auth-log" => Some(CommandName::AuthLog),

        // Networking
        "ip" => Some(CommandName::Ip),
        "ping" => Some(CommandName::Ping),
        "traceroute" => Some(CommandName::Traceroute),
        "netstat" => Some(CommandName::Netstat),
        "nmap" => Some(CommandName::Nmap),
        "curl" => Some(CommandName::Curl),
        "dig" => Some(CommandName::Dig),

        // Service Management
        "systemctl-start" => Some(CommandName::SystemctlStart),
        "systemctl-stop" => Some(CommandName::SystemctlStop),
        "systemctl-restart" => Some(CommandName::SystemctlRestart),
        "systemctl-enable" => Some(CommandName::SystemctlEnable),
        "systemctl-disable" => Some(CommandName::SystemctlDisable),
        "systemctl-status" => Some(CommandName::SystemctlStatus),
        "systemctl-list" => Some(CommandName::SystemctlList),

        // Security
        "chkrootkit" => Some(CommandName::Chkrootkit),
        "rkhunter" => Some(CommandName::Rkhunter),
        "lynis" => Some(CommandName::Lynis),
        "clamscan" => Some(CommandName::Clamscan),
        "fail2ban" => Some(CommandName::Fail2ban),
        "gpg-encrypt" => Some(CommandName::GpgEncrypt),
        "gpg-decrypt" => Some(CommandName::GpgDecrypt),
        "openssl-encrypt" => Some(CommandName::OpensslEncrypt),
        "openssl-decrypt" => Some(CommandName::OpensslDecrypt),

        // Package Management
        "apt-install" => Some(CommandName::AptInstall),
        "apt-remove" => Some(CommandName::AptRemove),
        "apt-update" => Some(CommandName::AptUpdate),
        "pacman-install" => Some(CommandName::PacmanInstall),
        "pacman-remove" => Some(CommandName::PacmanRemove),
        "pacman-update" => Some(CommandName::PacmanUpdate),
        "yay-install" => Some(CommandName::YayInstall),
        "which" => Some(CommandName::Which),
        "whereis" => Some(CommandName::Whereis),
        "find-binary" => Some(CommandName::FindBinary),
        "locate-binary" => Some(CommandName::LocateBinary),
        "dpkg-list" => Some(CommandName::DpkgList),
        "pacman-list" => Some(CommandName::PacmanList),

        _ => None,
    };

    match command {
        Some(cmd) => {
            if let Some(handler) = COMMAND_MAP.get(&cmd) {
                handler.lock().unwrap()(arg);
            } else {
                println!("{} {}", "Error:".bright_red(), "Command handler not found".bright_red());
            }
        }
        None => println!("{} {}", "Unknown command:".bright_red(), cmd),
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

pub fn show_commands_by_category(category: CommandCategory) {
    println!("\n{}", format!("[ {} ]", category_name(category)).bright_purple().bold());
    
    for cmd in CommandName::iter() {
        if cmd.category() == category {
            println!("{} {} {} {}",
                "•".bright_green(),
                cmd_name(cmd).bright_magenta(),
                "–".bright_blue(),
                cmd.description().bright_cyan()
            );
        }
    }
}

fn category_name(category: CommandCategory) -> &'static str {
    match category {
        CommandCategory::ProcessManagement => "Process Management",
        CommandCategory::ResourceMonitoring => "Resource Monitoring",
        CommandCategory::DiskAndStorage => "Disk and Storage",
        CommandCategory::LogDiving => "Log Diving",
        CommandCategory::Networking => "Networking",
        CommandCategory::ServiceManagement => "Service Management",
        CommandCategory::Security => "Security",
        CommandCategory::PackageManagement => "Package Management",
    }
}

fn cmd_name(cmd: CommandName) -> &'static str {
    match cmd {
        // Process Management
        CommandName::Ps => "ps",
        CommandName::Htop => "htop",
        CommandName::KillGraceful => "kill",
        CommandName::KillForce => "kill9",
        CommandName::Pkill => "pkill",
        CommandName::Pgrep => "pgrep",

        // Resource Monitoring
        CommandName::Free => "free",
        CommandName::Vmstat => "vmstat",
        CommandName::Iostat => "iostat",
        CommandName::Mpstat => "mpstat",
        CommandName::Sar => "sar",

        // Disk and Storage
        CommandName::Df => "df",
        CommandName::Du => "du",
        CommandName::Ncdu => "ncdu",
        CommandName::Lsblk => "lsblk",
        CommandName::Mount => "mount",
        CommandName::Umount => "umount",

        // Log Diving
        CommandName::JournalctlSystem => "journalctl",
        CommandName::JournalctlService => "journalctl-service",
        CommandName::Dmesg => "dmesg",
        CommandName::TailSyslog => "tail-syslog",
        CommandName::AuthLog => "auth-log",

        // Networking
        CommandName::Ip => "ip",
        CommandName::Ping => "ping",
        CommandName::Traceroute => "traceroute",
        CommandName::Netstat => "netstat",
        CommandName::Nmap => "nmap",
        CommandName::Curl => "curl",
        CommandName::Dig => "dig",

        // Service Management
        CommandName::SystemctlStart => "systemctl-start",
        CommandName::SystemctlStop => "systemctl-stop",
        CommandName::SystemctlRestart => "systemctl-restart",
        CommandName::SystemctlEnable => "systemctl-enable",
        CommandName::SystemctlDisable => "systemctl-disable",
        CommandName::SystemctlStatus => "systemctl-status",
        CommandName::SystemctlList => "systemctl-list",

        // Security
        CommandName::Chkrootkit => "chkrootkit",
        CommandName::Rkhunter => "rkhunter",
        CommandName::Lynis => "lynis",
        CommandName::Clamscan => "clamscan",
        CommandName::Fail2ban => "fail2ban",
        CommandName::GpgEncrypt => "gpg-encrypt",
        CommandName::GpgDecrypt => "gpg-decrypt",
        CommandName::OpensslEncrypt => "openssl-encrypt",
        CommandName::OpensslDecrypt => "openssl-decrypt",

        // Package Management
        CommandName::AptInstall => "apt-install",
        CommandName::AptRemove => "apt-remove",
        CommandName::AptUpdate => "apt-update",
        CommandName::PacmanInstall => "pacman-install",
        CommandName::PacmanRemove => "pacman-remove",
        CommandName::PacmanUpdate => "pacman-update",
        CommandName::YayInstall => "yay-install",
        CommandName::Which => "which",
        CommandName::Whereis => "whereis",
        CommandName::FindBinary => "find-binary",
        CommandName::LocateBinary => "locate-binary",
        CommandName::DpkgList => "dpkg-list",
        CommandName::PacmanList => "pacman-list",
    }
} 