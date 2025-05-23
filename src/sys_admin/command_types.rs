use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use colored::*;
use crate::sys_admin::command_logic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    ProcessManagement,
    ResourceMonitoring,
    DiskAndStorage,
    LogDiving,
    Networking,
    ServiceManagement,
    Security,
    PackageManagement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandName {
    // Process Management
    Ps,
    Htop,
    KillGraceful,
    KillForce,
    Pkill,
    Pgrep,

    // Resource Monitoring
    Free,
    Vmstat,
    Iostat,
    Mpstat,
    Sar,

    // Disk and Storage
    Df,
    Du,
    Ncdu,
    Lsblk,
    Mount,
    Umount,

    // Log Diving
    JournalctlSystem,
    JournalctlService,
    Dmesg,
    TailSyslog,
    AuthLog,

    // Networking
    Ip,
    Ping,
    Traceroute,
    Netstat,
    Nmap,
    Curl,
    Dig,

    // Service Management
    SystemctlStart,
    SystemctlStop,
    SystemctlRestart,
    SystemctlEnable,
    SystemctlDisable,
    SystemctlStatus,
    SystemctlList,

    // Security
    Chkrootkit,
    Rkhunter,
    Lynis,
    Clamscan,
    Fail2ban,
    GpgEncrypt,
    GpgDecrypt,
    OpensslEncrypt,
    OpensslDecrypt,

    // Package Management
    AptInstall,
    AptRemove,
    AptUpdate,
    PacmanInstall,
    PacmanRemove,
    PacmanUpdate,
    YayInstall,
    Which,
    Whereis,
    FindBinary,
    LocateBinary,
    DpkgList,
    PacmanList,
}

impl CommandName {
    pub fn iter() -> impl Iterator<Item = CommandName> {
        use CommandName::*;
        [
            // Process Management
            Ps, Htop, KillGraceful, KillForce, Pkill, Pgrep,
            // Resource Monitoring
            Free, Vmstat, Iostat, Mpstat, Sar,
            // Disk and Storage
            Df, Du, Ncdu, Lsblk, Mount, Umount,
            // Log Diving
            JournalctlSystem, JournalctlService, Dmesg, TailSyslog, AuthLog,
            // Networking
            Ip, Ping, Traceroute, Netstat, Nmap, Curl, Dig,
            // Service Management
            SystemctlStart, SystemctlStop, SystemctlRestart, SystemctlEnable,
            SystemctlDisable, SystemctlStatus, SystemctlList,
            // Security
            Chkrootkit, Rkhunter, Lynis, Clamscan, Fail2ban,
            GpgEncrypt, GpgDecrypt, OpensslEncrypt, OpensslDecrypt,
            // Package Management
            AptInstall, AptRemove, AptUpdate, PacmanInstall, PacmanRemove,
            PacmanUpdate, YayInstall, Which, Whereis, FindBinary,
            LocateBinary, DpkgList, PacmanList,
        ].iter().copied()
    }

    pub fn category(&self) -> CommandCategory {
        match self {
            // Process Management
            CommandName::Ps
            | CommandName::Htop
            | CommandName::KillGraceful
            | CommandName::KillForce
            | CommandName::Pkill
            | CommandName::Pgrep => CommandCategory::ProcessManagement,

            // Resource Monitoring
            CommandName::Free
            | CommandName::Vmstat
            | CommandName::Iostat
            | CommandName::Mpstat
            | CommandName::Sar => CommandCategory::ResourceMonitoring,

            // Disk and Storage
            CommandName::Df
            | CommandName::Du
            | CommandName::Ncdu
            | CommandName::Lsblk
            | CommandName::Mount
            | CommandName::Umount => CommandCategory::DiskAndStorage,

            // Log Diving
            CommandName::JournalctlSystem
            | CommandName::JournalctlService
            | CommandName::Dmesg
            | CommandName::TailSyslog
            | CommandName::AuthLog => CommandCategory::LogDiving,

            // Networking
            CommandName::Ip
            | CommandName::Ping
            | CommandName::Traceroute
            | CommandName::Netstat
            | CommandName::Nmap
            | CommandName::Curl
            | CommandName::Dig => CommandCategory::Networking,

            // Service Management
            CommandName::SystemctlStart
            | CommandName::SystemctlStop
            | CommandName::SystemctlRestart
            | CommandName::SystemctlEnable
            | CommandName::SystemctlDisable
            | CommandName::SystemctlStatus
            | CommandName::SystemctlList => CommandCategory::ServiceManagement,

            // Security
            CommandName::Chkrootkit
            | CommandName::Rkhunter
            | CommandName::Lynis
            | CommandName::Clamscan
            | CommandName::Fail2ban
            | CommandName::GpgEncrypt
            | CommandName::GpgDecrypt
            | CommandName::OpensslEncrypt
            | CommandName::OpensslDecrypt => CommandCategory::Security,

            // Package Management
            CommandName::AptInstall
            | CommandName::AptRemove
            | CommandName::AptUpdate
            | CommandName::PacmanInstall
            | CommandName::PacmanRemove
            | CommandName::PacmanUpdate
            | CommandName::YayInstall
            | CommandName::Which
            | CommandName::Whereis
            | CommandName::FindBinary
            | CommandName::LocateBinary
            | CommandName::DpkgList
            | CommandName::PacmanList => CommandCategory::PackageManagement,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            // Process Management
            CommandName::Ps => "Show running processes",
            CommandName::Htop => "Interactive process viewer",
            CommandName::KillGraceful => "Gracefully terminate a process",
            CommandName::KillForce => "Forcefully terminate a process",
            CommandName::Pkill => "Kill processes by name",
            CommandName::Pgrep => "Find processes by name",

            // Resource Monitoring
            CommandName::Free => "Human-readable memory stats",
            CommandName::Vmstat => "Memory + process + CPU stats",
            CommandName::Iostat => "CPU and disk I/O stats",
            CommandName::Mpstat => "Per-core CPU stats",
            CommandName::Sar => "Historical system stats",

            // Disk and Storage
            CommandName::Df => "Disk space usage (human readable)",
            CommandName::Du => "Folder sizes",
            CommandName::Ncdu => "Interactive disk usage explorer",
            CommandName::Lsblk => "Block devices",
            CommandName::Mount => "Show mounted filesystems",
            CommandName::Umount => "Unmount a device",

            // Log Diving
            CommandName::JournalctlSystem => "System journal logs",
            CommandName::JournalctlService => "Service-specific logs",
            CommandName::Dmesg => "Kernel messages",
            CommandName::TailSyslog => "System log tail",
            CommandName::AuthLog => "Authentication logs",

            // Networking
            CommandName::Ip => "Show network interfaces",
            CommandName::Ping => "Test network connectivity",
            CommandName::Traceroute => "Trace network path",
            CommandName::Netstat => "Network statistics",
            CommandName::Nmap => "Network scanner",
            CommandName::Curl => "Transfer data from URLs",
            CommandName::Dig => "DNS lookup utility",

            // Service Management
            CommandName::SystemctlStart => "Start a service",
            CommandName::SystemctlStop => "Stop a service",
            CommandName::SystemctlRestart => "Restart a service",
            CommandName::SystemctlEnable => "Enable a service",
            CommandName::SystemctlDisable => "Disable a service",
            CommandName::SystemctlStatus => "Check service status",
            CommandName::SystemctlList => "List all services",

            // Security
            CommandName::Chkrootkit => "Check for rootkits",
            CommandName::Rkhunter => "Rootkit hunter",
            CommandName::Lynis => "Security auditing",
            CommandName::Clamscan => "Virus scanner",
            CommandName::Fail2ban => "Ban malicious IPs",
            CommandName::GpgEncrypt => "Encrypt with GPG",
            CommandName::GpgDecrypt => "Decrypt with GPG",
            CommandName::OpensslEncrypt => "Encrypt with OpenSSL",
            CommandName::OpensslDecrypt => "Decrypt with OpenSSL",

            // Package Management
            CommandName::AptInstall => "Install Debian package",
            CommandName::AptRemove => "Remove Debian package",
            CommandName::AptUpdate => "Update Debian packages",
            CommandName::PacmanInstall => "Install Arch package",
            CommandName::PacmanRemove => "Remove Arch package",
            CommandName::PacmanUpdate => "Update Arch packages",
            CommandName::YayInstall => "Install AUR package",
            CommandName::Which => "Find binary location",
            CommandName::Whereis => "Locate binary and docs",
            CommandName::FindBinary => "Find binary in PATH",
            CommandName::LocateBinary => "Find binary in database",
            CommandName::DpkgList => "List Debian packages",
            CommandName::PacmanList => "List Arch packages",
        }
    }
}

type CommandFn = Arc<Mutex<Box<dyn Fn(&str) -> () + Send + Sync>>>;

lazy_static! {
    pub static ref COMMAND_MAP: HashMap<CommandName, CommandFn> = init_command_map();
}

fn init_command_map() -> HashMap<CommandName, CommandFn> {
    let mut m = HashMap::new();
    
    // Process Management
    m.insert(CommandName::Ps, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::ps()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Htop, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::htop()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::KillGraceful, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // Try to find process by name if not a PID
        if arg.chars().all(|c| c.is_digit(10)) {
            command_logic::kill_graceful(arg);
        } else {
            // Search for process by name
            let output = std::process::Command::new("pgrep")
                .arg("-l")
                .arg(arg)
                .output();
            
            match output {
                Ok(out) => {
                    let processes = String::from_utf8_lossy(&out.stdout);
                    if processes.is_empty() {
                        println!("{}", format!("No process found matching '{}'", arg).bright_red());
                    } else {
                        println!("{}", "Found processes:".bright_cyan());
                        println!("{}", processes.bright_green());
                        println!("{}", "Killing all matching processes...".bright_yellow());
                        command_logic::kill_graceful(arg);
                    }
                },
                Err(e) => println!("{} {}", "Search failed:".bright_red(), e),
            }
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::KillForce, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // Try to find process by name if not a PID
        if arg.chars().all(|c| c.is_digit(10)) {
            command_logic::kill_force(arg);
        } else {
            // Search for process by name
            let output = std::process::Command::new("pgrep")
                .arg("-l")
                .arg(arg)
                .output();
            
            match output {
                Ok(out) => {
                    let processes = String::from_utf8_lossy(&out.stdout);
                    if processes.is_empty() {
                        println!("{}", format!("No process found matching '{}'", arg).bright_red());
                    } else {
                        println!("{}", "Found processes:".bright_cyan());
                        println!("{}", processes.bright_green());
                        println!("{}", "Forcefully killing all matching processes...".bright_yellow());
                        command_logic::kill_force(arg);
                    }
                },
                Err(e) => println!("{} {}", "Search failed:".bright_red(), e),
            }
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Pkill, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // Search for process by name first
        let output = std::process::Command::new("pgrep")
            .arg("-l")
            .arg(arg)
            .output();
        
        match output {
            Ok(out) => {
                let processes = String::from_utf8_lossy(&out.stdout);
                if processes.is_empty() {
                    println!("{}", format!("No process found matching '{}'", arg).bright_red());
                } else {
                    println!("{}", "Found processes:".bright_cyan());
                    println!("{}", processes.bright_green());
                    println!("{}", "Killing all matching processes...".bright_yellow());
                    command_logic::pkill(arg);
                }
            },
            Err(e) => println!("{} {}", "Search failed:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Pgrep, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // Search for process by name
        let output = std::process::Command::new("pgrep")
            .arg("-l")
            .arg(arg)
            .output();
        
        match output {
            Ok(out) => {
                let processes = String::from_utf8_lossy(&out.stdout);
                if processes.is_empty() {
                    println!("{}", format!("No process found matching '{}'", arg).bright_red());
                } else {
                    println!("{}", "Found processes:".bright_cyan());
                    println!("{}", processes.bright_green());
                }
            },
            Err(e) => println!("{} {}", "Search failed:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Resource Monitoring
    m.insert(CommandName::Free, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::free()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Vmstat, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        let interval = parts.get(0).unwrap_or(&"1");
        let count = parts.get(1).unwrap_or(&"5");
        command_logic::vmstat(interval, count);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Iostat, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        let interval = parts.get(0).unwrap_or(&"1");
        let count = parts.get(1).unwrap_or(&"5");
        command_logic::iostat(interval, count);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Mpstat, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        let interval = parts.get(0).unwrap_or(&"1");
        let count = parts.get(1).unwrap_or(&"5");
        command_logic::mpstat(interval, count);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Sar, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        let interval = parts.get(0).unwrap_or(&"1");
        let count = parts.get(1).unwrap_or(&"5");
        command_logic::sar(interval, count);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Disk and Storage
    m.insert(CommandName::Df, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::df()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Du, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // If no path provided, use current directory
        let path = if arg.is_empty() { "." } else { arg };
        command_logic::du(path);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Ncdu, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // If no path provided, use current directory
        let path = if arg.is_empty() { "." } else { arg };
        command_logic::ncdu(path);
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Lsblk, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::lsblk()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Mount, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::mount()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Umount, Arc::new(Mutex::new(Box::new(|arg: &str| {
        // Try to find mount point if device name is provided
        if !arg.starts_with("/") {
            let output = std::process::Command::new("findmnt")
                .arg("-n")
                .arg("-o")
                .arg("TARGET")
                .arg(arg)
                .output();
            
            match output {
                Ok(out) => {
                    let mountpoint = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !mountpoint.is_empty() {
                        println!("{}", format!("Found mount point: {}", mountpoint).bright_cyan());
                        command_logic::umount(&mountpoint);
                    } else {
                        command_logic::umount(arg);
                    }
                },
                Err(_) => command_logic::umount(arg),
            }
        } else {
            command_logic::umount(arg);
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Log Diving
    m.insert(CommandName::JournalctlSystem, Arc::new(Mutex::new(Box::new(|_: &str| {
        println!("{}", "Showing system journal logs...".bright_cyan());
        let output = std::process::Command::new("journalctl")
            .arg("-xe")  // Show errors and warnings
            .arg("--no-pager")  // Don't use pager
            .arg("--since")  // Show last hour
            .arg("1 hour ago")
            .output();
        
        match output {
            Ok(out) => {
                let logs = String::from_utf8_lossy(&out.stdout);
                if logs.is_empty() {
                    println!("{}", "No recent system logs found.".bright_red());
                } else {
                    // Color code log levels
                    for line in logs.lines() {
                        let color = if line.contains("error") || line.contains("Error") { "red" }
                                  else if line.contains("warning") || line.contains("Warning") { "yellow" }
                                  else if line.contains("info") || line.contains("Info") { "green" }
                                  else { "white" };
                        println!("{}", line.color(color));
                    }
                }
            },
            Err(e) => println!("{} {}", "Failed to read system logs:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::JournalctlService, Arc::new(Mutex::new(Box::new(|_: &str| {
        println!("{}", "Showing recent service logs...".bright_cyan());
        let output = std::process::Command::new("systemctl")
            .arg("list-units")
            .arg("--type=service")
            .arg("--state=running")
            .output();
        
        match output {
            Ok(out) => {
                let services = String::from_utf8_lossy(&out.stdout);
                if services.is_empty() {
                    println!("{}", "No running services found.".bright_red());
                } else {
                    println!("{}", "Running Services:".bright_cyan());
                    for line in services.lines().skip(1) {  // Skip header
                        if let Some(service) = line.split_whitespace().next() {
                            println!("{}", format!("• {}", service).bright_green());
                            // Show last 5 log entries for each service
                            let logs = std::process::Command::new("journalctl")
                                .arg("-u")
                                .arg(service)
                                .arg("-n")
                                .arg("5")
                                .arg("--no-pager")
                                .output();
                            
                            match logs {
                                Ok(log_out) => {
                                    let service_logs = String::from_utf8_lossy(&log_out.stdout);
                                    for log_line in service_logs.lines() {
                                        println!("  {}", log_line.bright_blue());
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                    }
                }
            },
            Err(e) => println!("{} {}", "Failed to list services:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Dmesg, Arc::new(Mutex::new(Box::new(|_: &str| {
        println!("{}", "Showing recent kernel messages...".bright_cyan());
        let output = std::process::Command::new("dmesg")
            .arg("-T")  // Human readable timestamps
            .arg("--level=err,warn")  // Show errors and warnings
            .output();
        
        match output {
            Ok(out) => {
                let messages = String::from_utf8_lossy(&out.stdout);
                if messages.is_empty() {
                    println!("{}", "No recent kernel messages found.".bright_red());
                } else {
                    for line in messages.lines() {
                        let color = if line.contains("error") || line.contains("Error") { "red" }
                                  else if line.contains("warning") || line.contains("Warning") { "yellow" }
                                  else { "white" };
                        println!("{}", line.color(color));
                    }
                }
            },
            Err(e) => println!("{} {}", "Failed to read kernel messages:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::TailSyslog, Arc::new(Mutex::new(Box::new(|_: &str| {
        println!("{}", "Showing recent system log entries...".bright_cyan());
        let output = std::process::Command::new("tail")
            .arg("-n")
            .arg("50")  // Show last 50 lines
            .arg("/var/log/syslog")
            .output();
        
        match output {
            Ok(out) => {
                let logs = String::from_utf8_lossy(&out.stdout);
                if logs.is_empty() {
                    println!("{}", "No system log entries found.".bright_red());
                } else {
                    for line in logs.lines() {
                        let color = if line.contains("error") || line.contains("Error") { "red" }
                                  else if line.contains("warning") || line.contains("Warning") { "yellow" }
                                  else if line.contains("info") || line.contains("Info") { "green" }
                                  else { "white" };
                        println!("{}", line.color(color));
                    }
                }
            },
            Err(e) => println!("{} {}", "Failed to read system log:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::AuthLog, Arc::new(Mutex::new(Box::new(|_: &str| {
        println!("{}", "Showing recent authentication logs...".bright_cyan());
        let output = std::process::Command::new("tail")
            .arg("-n")
            .arg("50")  // Show last 50 lines
            .arg("/var/log/auth.log")
            .output();
        
        match output {
            Ok(out) => {
                let logs = String::from_utf8_lossy(&out.stdout);
                if logs.is_empty() {
                    println!("{}", "No authentication logs found.".bright_red());
                } else {
                    for line in logs.lines() {
                        let color = if line.contains("failed") || line.contains("Failed") { "red" }
                                  else if line.contains("accepted") || line.contains("Accepted") { "green" }
                                  else { "white" };
                        println!("{}", line.color(color));
                    }
                }
            },
            Err(e) => println!("{} {}", "Failed to read authentication log:".bright_red(), e),
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Networking
    m.insert(CommandName::Ip, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::ip_a()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Ping, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::ping(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Traceroute, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::traceroute(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Netstat, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::netstat()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Nmap, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::nmap(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Curl, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::curl(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Dig, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::dig(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Service Management
    m.insert(CommandName::SystemctlStart, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::systemctl_start(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlStop, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::systemctl_stop(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlRestart, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::systemctl_restart(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlEnable, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::systemctl_enable(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlDisable, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::systemctl_disable(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlStatus, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::systemctl_status()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::SystemctlList, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::systemctl_list()) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Security
    m.insert(CommandName::Chkrootkit, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::chkrootkit()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Rkhunter, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::rkhunter()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Lynis, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::lynis()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Clamscan, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::clamscan(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Fail2ban, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::fail2ban()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::GpgEncrypt, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.splitn(2, ' ').collect();
        if parts.len() == 2 {
            command_logic::gpg_encrypt(parts[0], parts[1]);
        } else {
            println!("{}", "Error: Please provide both input and output files".red());
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::GpgDecrypt, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.splitn(2, ' ').collect();
        if parts.len() == 2 {
            command_logic::gpg_decrypt(parts[0]);
        } else {
            println!("{}", "Error: Please provide input file".red());
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::OpensslEncrypt, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.splitn(2, ' ').collect();
        if parts.len() == 2 {
            command_logic::openssl_encrypt(parts[0], parts[1]);
        } else {
            println!("{}", "Error: Please provide both input and output files".red());
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::OpensslDecrypt, Arc::new(Mutex::new(Box::new(|arg: &str| {
        let parts: Vec<&str> = arg.splitn(2, ' ').collect();
        if parts.len() == 2 {
            command_logic::openssl_decrypt(parts[0], parts[1]);
        } else {
            println!("{}", "Error: Please provide both input and output files".red());
        }
    }) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    // Package Management
    m.insert(CommandName::AptInstall, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::apt_install(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::AptRemove, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::apt_remove(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::AptUpdate, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::apt_update()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::PacmanInstall, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::pacman_install(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::PacmanRemove, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::pacman_remove(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::PacmanUpdate, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::pacman_update()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::YayInstall, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::yay_install(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Which, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::which(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::Whereis, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::whereis(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::FindBinary, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::find_binary(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::LocateBinary, Arc::new(Mutex::new(Box::new(|arg: &str| command_logic::locate_binary(arg)) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::DpkgList, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::dpkg_list()) as Box<dyn Fn(&str) -> () + Send + Sync>)));
    m.insert(CommandName::PacmanList, Arc::new(Mutex::new(Box::new(|_: &str| command_logic::pacman_list()) as Box<dyn Fn(&str) -> () + Send + Sync>)));

    m
} 