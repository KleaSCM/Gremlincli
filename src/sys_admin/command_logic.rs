use std::process::Command;
use colored::*;

pub fn ip_a() {
    println!("{}", "Checking network interfaces and IPs...".bright_cyan());
    let output = Command::new("ip")
        .arg("a")
        .output();

    match output {
        Ok(out) => {
            let interfaces = String::from_utf8_lossy(&out.stdout);
            if interfaces.is_empty() {
                println!("{}", "No network interfaces found.".bright_red());
            } else {
                println!("{}", "Network Interfaces:".bright_cyan());
                println!("{}", interfaces.bright_green());
            }
        },
        Err(e) => println!("{} {}", "IP command failed:".bright_red(), e),
    }
}

pub fn ping(target: &str) {
    // Try to determine if it's an IP or hostname
    let is_ip = target.split('.').count() == 4 && 
                target.split('.').all(|x| x.parse::<u8>().is_ok());
    
    println!("{}", format!("Pinging {} ({})...", 
        target, 
        if is_ip { "IP address" } else { "hostname" }
    ).bright_cyan());
    
    let output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg(target)
        .output();

    match output {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            if result.contains("100% packet loss") {
                println!("{}", "❌ Target is unreachable".bright_red());
            } else if result.contains("0% packet loss") {
                println!("{}", "✅ Target is reachable".bright_green());
            } else {
                println!("{}", result.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Ping failed:".bright_red(), e),
    }
}

pub fn traceroute(target: &str) {
    println!("{}", format!("Tracing route to {}...", target).bright_cyan());
    let output = Command::new("traceroute")
        .arg(target)
        .output();

    match output {
        Ok(out) => {
            let trace = String::from_utf8_lossy(&out.stdout);
            if trace.is_empty() {
                println!("{}", "No route found to target.".bright_red());
            } else {
                println!("{}", "Route:".bright_cyan());
                println!("{}", trace.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Traceroute failed:".bright_red(), e),
    }
}

pub fn netstat() {
    println!("{}", "Checking network connections...".bright_cyan());
    let output = Command::new("netstat")
        .arg("-tuln")
        .output();

    match output {
        Ok(out) => {
            let connections = String::from_utf8_lossy(&out.stdout);
            if connections.is_empty() {
                println!("{}", "No active network connections found.".bright_red());
            } else {
                println!("{}", "Active Connections:".bright_cyan());
                println!("{}", connections.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Netstat failed:".bright_red(), e),
    }
}

pub fn nmap(target: &str) {
    // Try to determine if it's an IP or hostname
    let is_ip = target.split('.').count() == 4 && 
                target.split('.').all(|x| x.parse::<u8>().is_ok());
    
    println!("{}", format!("Scanning {} ({}) for open ports...", 
        target, 
        if is_ip { "IP address" } else { "hostname" }
    ).bright_cyan());
    
    let output = Command::new("nmap")
        .arg("-sV")
        .arg(target)
        .output();

    match output {
        Ok(out) => {
            let scan = String::from_utf8_lossy(&out.stdout);
            if scan.is_empty() {
                println!("{}", "No open ports found.".bright_red());
            } else {
                println!("{}", "Scan Results:".bright_cyan());
                println!("{}", scan.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Nmap failed:".bright_red(), e),
    }
}

pub fn curl(url: &str) {
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        println!("{}", "Invalid URL format. Please include http:// or https://".bright_red());
        return;
    }

    println!("{}", format!("Testing connection to {}...", url).bright_cyan());
    let output = Command::new("curl")
        .arg("-I")
        .arg(url)
        .output();

    match output {
        Ok(out) => {
            let headers = String::from_utf8_lossy(&out.stdout);
            if headers.is_empty() {
                println!("{}", "No response received from server.".bright_red());
            } else {
                println!("{}", "Server Response:".bright_cyan());
                println!("{}", headers.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Curl failed:".bright_red(), e),
    }
}

pub fn dig(domain: &str) {
    println!("{}", format!("Checking DNS records for {}...", domain).bright_cyan());
    let output = Command::new("dig")
        .arg("+short")
        .arg(domain)
        .output();

    match output {
        Ok(out) => {
            let records = String::from_utf8_lossy(&out.stdout);
            if records.is_empty() {
                println!("{}", "No DNS records found.".bright_red());
            } else {
                println!("{}", "DNS Records:".bright_cyan());
                println!("{}", records.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Dig failed:".bright_red(), e),
    }
}

pub fn kill(target: &str) {
    println!("{}", format!("Attempting to kill {}...", target).bright_cyan());
    let output = if target.chars().all(char::is_numeric) {
        Command::new("kill").arg(target).output()
    } else {
        Command::new("pkill").arg(target).output()
    };

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Kill failed:".bright_red(), e),
    }
}

pub fn ps() {
    println!("{}", "Showing running processes...".bright_cyan());
    let output = Command::new("ps")
        .arg("aux")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "PS failed:".bright_red(), e),
    }
}

pub fn htop() {
    println!("{}", "Launching htop...".bright_cyan());
    let status = Command::new("htop")
        .status();

    match status {
        Ok(_) => println!("{}", "Htop closed.".bright_green()),
        Err(e) => println!("{} {}", "Htop failed:".bright_red(), e),
    }
}

pub fn kill_graceful(pid_or_name: &str) {
    // Try to parse as PID first
    if let Ok(pid) = pid_or_name.parse::<i32>() {
        println!("{}", format!("Attempting to kill PID {} gracefully...", pid).bright_cyan());
        let output = Command::new("kill")
            .arg(pid.to_string())
            .output();

        match output {
            Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
            Err(e) => println!("{} {}", "Kill failed:".bright_red(), e),
        }
    } else {
        // If not a PID, try to find the process by name
        println!("{}", format!("Searching for processes named {}...", pid_or_name).bright_cyan());
        let pgrep_output = Command::new("pgrep")
            .arg("-l")
            .arg(pid_or_name)
            .output();

        match pgrep_output {
            Ok(out) => {
                let processes = String::from_utf8_lossy(&out.stdout);
                if processes.is_empty() {
                    println!("{}", format!("No processes found matching '{}'", pid_or_name).bright_red());
                    return;
                }
                
                println!("{}", "Found processes:".bright_cyan());
                println!("{}", processes.bright_green());
                
                // Ask if user wants to kill all matching processes
                println!("{}", "Kill all matching processes? (y/n): ".bright_yellow());
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                
                if input.trim().to_lowercase() == "y" {
                    let pkill_output = Command::new("pkill")
                        .arg(pid_or_name)
                        .output();
                    
                    match pkill_output {
                        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
                        Err(e) => println!("{} {}", "Pkill failed:".bright_red(), e),
                    }
                }
            },
            Err(e) => println!("{} {}", "Pgrep failed:".bright_red(), e),
        }
    }
}

pub fn kill_force(pid_or_name: &str) {
    // Try to parse as PID first
    if let Ok(pid) = pid_or_name.parse::<i32>() {
        println!("{}", format!("Brutally terminating PID {}...", pid).bright_cyan());
        let output = Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output();

        match output {
            Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
            Err(e) => println!("{} {}", "Kill failed:".bright_red(), e),
        }
    } else {
        // If not a PID, try to find the process by name
        println!("{}", format!("Searching for processes named {}...", pid_or_name).bright_cyan());
        let pgrep_output = Command::new("pgrep")
            .arg("-l")
            .arg(pid_or_name)
            .output();

        match pgrep_output {
            Ok(out) => {
                let processes = String::from_utf8_lossy(&out.stdout);
                if processes.is_empty() {
                    println!("{}", format!("No processes found matching '{}'", pid_or_name).bright_red());
                    return;
                }
                
                println!("{}", "Found processes:".bright_cyan());
                println!("{}", processes.bright_green());
                
                // Ask if user wants to kill all matching processes
                println!("{}", "Kill all matching processes? (y/n): ".bright_yellow());
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                
                if input.trim().to_lowercase() == "y" {
                    let pkill_output = Command::new("pkill")
                        .arg("-9")
                        .arg(pid_or_name)
                        .output();
                    
                    match pkill_output {
                        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
                        Err(e) => println!("{} {}", "Pkill failed:".bright_red(), e),
                    }
                }
            },
            Err(e) => println!("{} {}", "Pgrep failed:".bright_red(), e),
        }
    }
}

pub fn pkill(name: &str) {
    println!("{}", format!("Searching for processes named {}...", name).bright_cyan());
    let pgrep_output = Command::new("pgrep")
        .arg("-l")
        .arg(name)
        .output();

    match pgrep_output {
        Ok(out) => {
            let processes = String::from_utf8_lossy(&out.stdout);
            if processes.is_empty() {
                println!("{}", format!("No processes found matching '{}'", name).bright_red());
                return;
            }
            
            println!("{}", "Found processes:".bright_cyan());
            println!("{}", processes.bright_green());
            
            // Ask if user wants to kill all matching processes
            println!("{}", "Kill all matching processes? (y/n): ".bright_yellow());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            
            if input.trim().to_lowercase() == "y" {
                let pkill_output = Command::new("pkill")
                    .arg(name)
                    .output();
                
                match pkill_output {
                    Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
                    Err(e) => println!("{} {}", "Pkill failed:".bright_red(), e),
                }
            }
        },
        Err(e) => println!("{} {}", "Pgrep failed:".bright_red(), e),
    }
}

pub fn pgrep(name: &str) {
    println!("{}", format!("Searching for processes named {}...", name).bright_cyan());
    let output = Command::new("pgrep")
        .arg("-l")
        .arg(name)
        .output();

    match output {
        Ok(out) => {
            let processes = String::from_utf8_lossy(&out.stdout);
            if processes.is_empty() {
                println!("{}", format!("No processes found matching '{}'", name).bright_red());
            } else {
                println!("{}", "Found processes:".bright_cyan());
                println!("{}", processes.bright_green());
            }
        },
        Err(e) => println!("{} {}", "Pgrep failed:".bright_red(), e),
    }
}

pub fn free() {
    println!("{}", "Checking memory usage...".bright_cyan());
    let output = Command::new("free")
        .arg("-h")
        .output();

    match output {
        Ok(out) => {
            let memory = String::from_utf8_lossy(&out.stdout);
            if memory.is_empty() {
                println!("{}", "No memory information available.".bright_red());
            } else {
                println!("{}", "Memory Usage:".bright_cyan());
                // Parse and format the output
                let lines: Vec<&str> = memory.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    let mem_info = lines[1];
                    let swap_info = if lines.len() >= 3 { lines[2] } else { "" };
                    
                    println!("{}", format!("{}\n{}", headers, mem_info).bright_green());
                    if !swap_info.is_empty() {
                        println!("{}", swap_info.bright_green());
                    }
                } else {
                    println!("{}", memory.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Free failed:".bright_red(), e),
    }
}

pub fn vmstat(interval: &str, count: &str) {
    // Set defaults if not provided
    let interval = if interval.is_empty() { "1" } else { interval };
    let count = if count.is_empty() { "5" } else { count };

    println!("{}", format!("Monitoring system stats (interval: {}s, count: {})...", interval, count).bright_cyan());
    let output = Command::new("vmstat")
        .arg(interval)
        .arg(count)
        .output();

    match output {
        Ok(out) => {
            let stats = String::from_utf8_lossy(&out.stdout);
            if stats.is_empty() {
                println!("{}", "No system statistics available.".bright_red());
            } else {
                println!("{}", "System Statistics:".bright_cyan());
                let lines: Vec<&str> = stats.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    println!("{}", headers.bright_green());
                    
                    // Format each data line with color indicators
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 17 {
                            // CPU usage indicators
                            let cpu_idle = fields[15].parse::<f32>().unwrap_or(0.0);
                            let cpu_usage = 100.0 - cpu_idle;
                            let cpu_color = if cpu_usage > 80.0 { "red" } 
                                         else if cpu_usage > 50.0 { "yellow" } 
                                         else { "green" };
                            
                            let mem_free = fields[3].parse::<i32>().unwrap_or(0);
                            
                            println!("{}", format!("{} (CPU: {:.1}%, Free Mem: {}K)", 
                                line, cpu_usage, mem_free).color(cpu_color));
                        } else {
                            println!("{}", line.bright_green());
                        }
                    }
                } else {
                    println!("{}", stats.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Vmstat failed:".bright_red(), e),
    }
}

pub fn iostat(interval: &str, count: &str) {
    println!("{}", format!("Monitoring I/O stats (interval: {}s, count: {})...", interval, count).bright_cyan());
    println!("{}", "I/O Statistics:".bright_cyan());
    
    let output = Command::new("iostat")
        .arg("-x")
        .arg(interval)
        .arg(count)
        .output()
        .expect("Failed to execute iostat");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.is_empty() {
        println!("{}", "No I/O statistics available".bright_red());
        return;
    }
    
    // Print the header
    println!("{}", output_str.lines().take(3).collect::<Vec<&str>>().join("\n").bright_white());
    
    // Parse and format the device statistics
    for line in output_str.lines().skip(3) {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 7 {
            continue;
        }
        
        let device = parts[0];
        let util = parts[parts.len() - 1].parse::<f64>().unwrap_or(0.0);
        
        // Color code based on utilization
        let color = if util > 80.0 {
            "red"
        } else if util > 50.0 {
            "yellow"
        } else {
            "green"
        };
        
        println!("{}", format!("{}: {:.1}%", device, util).color(color));
    }
}

pub fn mpstat(interval: &str, count: &str) {
    // Set defaults if not provided
    let interval = if interval.is_empty() { "1" } else { interval };
    let count = if count.is_empty() { "5" } else { count };

    println!("{}", format!("Monitoring CPU stats (interval: {}s, count: {})...", interval, count).bright_cyan());
    let output = Command::new("mpstat")
        .arg("-P")
        .arg("ALL")
        .arg(interval)
        .arg(count)
        .output();

    match output {
        Ok(out) => {
            let stats = String::from_utf8_lossy(&out.stdout);
            if stats.is_empty() {
                println!("{}", "No CPU statistics available.".bright_red());
            } else {
                println!("{}", "CPU Statistics:".bright_cyan());
                let lines: Vec<&str> = stats.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    println!("{}", headers.bright_green());
                    
                    // Format each data line with color indicators
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 4 {
                            // CPU usage indicators
                            let idle = fields[3].parse::<f32>().unwrap_or(0.0);
                            let usage = 100.0 - idle;
                            let cpu_color = if usage > 80.0 { "red" } 
                                         else if usage > 50.0 { "yellow" } 
                                         else { "green" };
                            
                            println!("{}", format!("{} (Usage: {:.1}%)", 
                                line, usage).color(cpu_color));
                        } else {
                            println!("{}", line.bright_green());
                        }
                    }
                } else {
                    println!("{}", stats.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Mpstat failed:".bright_red(), e),
    }
}

pub fn sar(interval: &str, count: &str) {
    // Set defaults if not provided
    let interval = if interval.is_empty() { "1" } else { interval };
    let count = if count.is_empty() { "5" } else { count };

    println!("{}", format!("Monitoring system activity (interval: {}s, count: {})...", interval, count).bright_cyan());
    let output = Command::new("sar")
        .arg("-u")
        .arg("-r")
        .arg("-b")
        .arg(interval)
        .arg(count)
        .output();

    match output {
        Ok(out) => {
            let stats = String::from_utf8_lossy(&out.stdout);
            if stats.is_empty() {
                println!("{}", "No system activity data available.".bright_red());
            } else {
                println!("{}", "System Activity Report:".bright_cyan());
                let lines: Vec<&str> = stats.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    println!("{}", headers.bright_green());
                    
                    // Format each data line with color indicators
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 4 {
                            // CPU usage indicators
                            let idle = fields[3].parse::<f32>().unwrap_or(0.0);
                            let usage = 100.0 - idle;
                            let cpu_color = if usage > 80.0 { "red" } 
                                         else if usage > 50.0 { "yellow" } 
                                         else { "green" };
                            
                            println!("{}", format!("{} (CPU: {:.1}%)", 
                                line, usage).color(cpu_color));
                        } else {
                            println!("{}", line.bright_green());
                        }
                    }
                } else {
                    println!("{}", stats.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Sar failed:".bright_red(), e),
    }
}

pub fn df() {
    println!("{}", "Checking disk space usage...".bright_cyan());
    let output = Command::new("df")
        .arg("-h")
        .output();

    match output {
        Ok(out) => {
            let disk = String::from_utf8_lossy(&out.stdout);
            if disk.is_empty() {
                println!("{}", "No disk information available.".bright_red());
            } else {
                println!("{}", "Disk Usage:".bright_cyan());
                let lines: Vec<&str> = disk.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    println!("{}", headers.bright_green());
                    
                    // Format each data line with color indicators
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 5 {
                            // Disk usage indicators
                            let usage = fields[4].trim_end_matches('%').parse::<f32>().unwrap_or(0.0);
                            let disk_color = if usage > 90.0 { "red" } 
                                          else if usage > 70.0 { "yellow" } 
                                          else { "green" };
                            
                            println!("{}", format!("{} (Usage: {:.1}%)", 
                                line, usage).color(disk_color));
                        } else {
                            println!("{}", line.bright_green());
                        }
                    }
                } else {
                    println!("{}", disk.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Df failed:".bright_red(), e),
    }
}

pub fn du(path: &str) {
    if path.is_empty() {
        println!("{}", "Please provide a path to analyze".bright_red());
        return;
    }

    println!("{}", format!("Analyzing disk usage in {}...", path).bright_cyan());
    let output = Command::new("du")
        .arg("-h")
        .arg("--max-depth=1")
        .arg(path)
        .output();

    match output {
        Ok(out) => {
            let usage = String::from_utf8_lossy(&out.stdout);
            if usage.is_empty() {
                println!("{}", "No disk usage information available.".bright_red());
            } else {
                println!("{}", "Directory Sizes:".bright_cyan());
                let lines: Vec<&str> = usage.lines().collect();
                for line in lines {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        let size = fields[0];
                        let path = fields[1];
                        println!("{} {}", size.bright_green(), path.bright_blue());
                    }
                }
            }
        },
        Err(e) => println!("{} {}", "Du failed:".bright_red(), e),
    }
}

pub fn ncdu(path: &str) {
    if path.is_empty() {
        println!("{}", "Please provide a path to analyze".bright_red());
        return;
    }

    println!("{}", format!("Launching interactive disk usage explorer in {}...", path).bright_cyan());
    let status = Command::new("ncdu")
        .arg(path)
        .status();

    match status {
        Ok(_) => println!("{}", "Ncdu closed.".bright_green()),
        Err(e) => println!("{} {}", "Ncdu failed:".bright_red(), e),
    }
}

pub fn lsblk() {
    println!("{}", "Listing block devices...".bright_cyan());
    let output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE")
        .output();

    match output {
        Ok(out) => {
            let devices = String::from_utf8_lossy(&out.stdout);
            if devices.is_empty() {
                println!("{}", "No block devices found.".bright_red());
            } else {
                println!("{}", "Block Devices:".bright_cyan());
                let lines: Vec<&str> = devices.lines().collect();
                if lines.len() >= 2 {
                    let headers = lines[0];
                    println!("{}", headers.bright_green());
                    
                    // Format each device line
                    for line in lines.iter().skip(1) {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() >= 4 {
                            let mountpoint = fields[3];
                            let color = if mountpoint == "/" { "yellow" } 
                                     else if !mountpoint.is_empty() { "green" } 
                                     else { "blue" };
                            println!("{}", line.color(color));
                        } else {
                            println!("{}", line.bright_green());
                        }
                    }
                } else {
                    println!("{}", devices.bright_green());
                }
            }
        },
        Err(e) => println!("{} {}", "Lsblk failed:".bright_red(), e),
    }
}

pub fn mount() {
    println!("{}", "Showing mounted filesystems...".bright_cyan());
    let output = Command::new("mount")
        .output();

    match output {
        Ok(out) => {
            let mounts = String::from_utf8_lossy(&out.stdout);
            if mounts.is_empty() {
                println!("{}", "No mounted filesystems found.".bright_red());
            } else {
                println!("{}", "Mounted Filesystems:".bright_cyan());
                let lines: Vec<&str> = mounts.lines().collect();
                for line in lines {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 3 {
                        let mountpoint = fields[2];
                        let color = if mountpoint == "/" { "yellow" } 
                                 else if mountpoint.starts_with("/") { "green" } 
                                 else { "blue" };
                        println!("{}", line.color(color));
                    } else {
                        println!("{}", line.bright_green());
                    }
                }
            }
        },
        Err(e) => println!("{} {}", "Mount failed:".bright_red(), e),
    }
}

pub fn umount(device: &str) {
    if device.is_empty() {
        println!("{}", "Please provide a device or mount point to unmount".bright_red());
        return;
    }

    println!("{}", format!("Unmounting {}...", device).bright_cyan());
    let output = Command::new("umount")
        .arg(device)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("{}", format!("Successfully unmounted {}", device).bright_green());
            } else {
                println!("{}", format!("Failed to unmount {}: {}", 
                    device, String::from_utf8_lossy(&out.stderr)).bright_red());
            }
        },
        Err(e) => println!("{} {}", "Umount failed:".bright_red(), e),
    }
}

pub fn journalctl_system() {
    println!("{}", "Checking system logs...".bright_cyan());
    let output = Command::new("journalctl")
        .arg("-xe")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Journalctl failed:".bright_red(), e),
    }
}

pub fn journalctl_service(service: &str) {
    println!("{}", format!("Checking logs for service {}...", service).bright_cyan());
    let output = Command::new("journalctl")
        .arg("-u")
        .arg(service)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Journalctl failed:".bright_red(), e),
    }
}

pub fn dmesg() {
    println!("{}", "Checking kernel ring buffer...".bright_cyan());
    let output = Command::new("dmesg")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Dmesg failed:".bright_red(), e),
    }
}

pub fn tail_syslog() {
    println!("{}", "Watching system logs in real-time...".bright_cyan());
    let status = Command::new("tail")
        .arg("-f")
        .arg("/var/log/syslog")
        .status();

    match status {
        Ok(_) => println!("{}", "Log watching ended.".bright_green()),
        Err(e) => println!("{} {}", "Tail failed:".bright_red(), e),
    }
}

pub fn less_auth_log() {
    println!("{}", "Checking authentication logs...".bright_cyan());
    let status = Command::new("less")
        .arg("/var/log/auth.log")
        .status();

    match status {
        Ok(_) => println!("{}", "Auth log viewing ended.".bright_green()),
        Err(e) => println!("{} {}", "Less failed:".bright_red(), e),
    }
}

pub fn systemctl_start(service: &str) {
    println!("{}", "🚀 Starting service...".bright_green());
    let output = std::process::Command::new("systemctl")
        .arg("start")
        .arg(service)
        .output()
        .expect("Failed to start service");
    
    if output.status.success() {
        println!("{}", "✅ Service started successfully!".bright_green());
    } else {
        println!("{}", "❌ Failed to start service:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn systemctl_stop(service: &str) {
    println!("{}", "🛑 Stopping service...".bright_yellow());
    let output = std::process::Command::new("systemctl")
        .arg("stop")
        .arg(service)
        .output()
        .expect("Failed to stop service");
    
    if output.status.success() {
        println!("{}", "✅ Service stopped successfully!".bright_green());
    } else {
        println!("{}", "❌ Failed to stop service:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn systemctl_restart(service: &str) {
    println!("{}", "🔄 Restarting service...".bright_cyan());
    let output = std::process::Command::new("systemctl")
        .arg("restart")
        .arg(service)
        .output()
        .expect("Failed to restart service");
    
    if output.status.success() {
        println!("{}", "✅ Service restarted successfully!".bright_green());
    } else {
        println!("{}", "❌ Failed to restart service:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn systemctl_enable(service: &str) {
    println!("{}", "🔌 Enabling service...".bright_blue());
    let output = std::process::Command::new("systemctl")
        .arg("enable")
        .arg(service)
        .output()
        .expect("Failed to enable service");
    
    if output.status.success() {
        println!("{}", "✅ Service enabled successfully!".bright_green());
    } else {
        println!("{}", "❌ Failed to enable service:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn systemctl_disable(service: &str) {
    println!("{}", "🔌 Disabling service...".bright_yellow());
    let output = std::process::Command::new("systemctl")
        .arg("disable")
        .arg(service)
        .output()
        .expect("Failed to disable service");
    
    if output.status.success() {
        println!("{}", "✅ Service disabled successfully!".bright_green());
    } else {
        println!("{}", "❌ Failed to disable service:".bright_red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn systemctl_status() {
    println!("{}", "📊 Checking system status...".bright_cyan());
    let output = std::process::Command::new("systemctl")
        .arg("status")
        .output()
        .expect("Failed to get system status");
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn systemctl_list() {
    println!("{}", "📋 Listing all services...".bright_cyan());
    let output = std::process::Command::new("systemctl")
        .arg("list-units")
        .arg("--type=service")
        .arg("--all")
        .output()
        .expect("Failed to list services");
    
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn service_status(service: &str) {
    println!("{}", format!("Checking status of service {}...", service).bright_cyan());
    let output = Command::new("service")
        .arg(service)
        .arg("status")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Service status check failed:".bright_red(), e),
    }
}

pub fn chkrootkit() {
    println!("{}", "Scanning for rootkits with chkrootkit...".bright_cyan());
    let output = Command::new("chkrootkit")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Chkrootkit scan failed:".bright_red(), e),
    }
}

pub fn rkhunter() {
    println!("{}", "Scanning for rootkits with rkhunter...".bright_cyan());
    let output = Command::new("rkhunter")
        .arg("--check")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Rkhunter scan failed:".bright_red(), e),
    }
}

pub fn lynis() {
    println!("{}", "Running system audit with Lynis...".bright_cyan());
    let output = Command::new("lynis")
        .arg("audit")
        .arg("system")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Lynis audit failed:".bright_red(), e),
    }
}

pub fn clamscan(path: &str) {
    println!("{}", format!("Scanning {} for viruses...", path).bright_cyan());
    let output = Command::new("clamscan")
        .arg("-r")
        .arg(path)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "ClamAV scan failed:".bright_red(), e),
    }
}

pub fn fail2ban() {
    println!("{}", "🔒 Checking fail2ban status...".bright_cyan());
    let output = std::process::Command::new("sudo")
        .arg("fail2ban-client")
        .arg("status")
        .output()
        .expect("Failed to execute fail2ban-client");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn gpg_encrypt(file: &str, recipient: &str) {
    println!("{}", format!("Encrypting {} for {}...", file, recipient).bright_cyan());
    let output = Command::new("gpg")
        .arg("--encrypt")
        .arg("--recipient")
        .arg(recipient)
        .arg(file)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "GPG encryption failed:".bright_red(), e),
    }
}

pub fn gpg_decrypt(file: &str) {
    println!("{}", format!("Decrypting {}...", file).bright_cyan());
    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg(file)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "GPG decryption failed:".bright_red(), e),
    }
}

pub fn openssl_encrypt(file: &str, password: &str) {
    println!("{}", format!("Encrypting {} with OpenSSL...", file).bright_cyan());
    let output = Command::new("openssl")
        .arg("enc")
        .arg("-aes-256-cbc")
        .arg("-salt")
        .arg("-in")
        .arg(file)
        .arg("-out")
        .arg(format!("{}.enc", file))
        .arg("-pass")
        .arg(format!("pass:{}", password))
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "OpenSSL encryption failed:".bright_red(), e),
    }
}

pub fn openssl_decrypt(file: &str, password: &str) {
    println!("{}", format!("Decrypting {} with OpenSSL...", file).bright_cyan());
    let output = Command::new("openssl")
        .arg("enc")
        .arg("-aes-256-cbc")
        .arg("-d")
        .arg("-salt")
        .arg("-in")
        .arg(file)
        .arg("-out")
        .arg(file.trim_end_matches(".enc"))
        .arg("-pass")
        .arg(format!("pass:{}", password))
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "OpenSSL decryption failed:".bright_red(), e),
    }
}

pub fn apt_install(package: &str) {
    println!("{}", format!("Installing {} with apt...", package).bright_cyan());
    let output = Command::new("sudo")
        .arg("apt")
        .arg("install")
        .arg("-y")
        .arg(package)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Apt install failed:".bright_red(), e),
    }
}

pub fn apt_remove(package: &str) {
    println!("{}", format!("Removing {} with apt...", package).bright_cyan());
    let output = Command::new("sudo")
        .arg("apt")
        .arg("remove")
        .arg("-y")
        .arg(package)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Apt remove failed:".bright_red(), e),
    }
}

pub fn apt_update() {
    println!("{}", "Updating package lists with apt...".bright_cyan());
    let output = Command::new("sudo")
        .arg("apt")
        .arg("update")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Apt update failed:".bright_red(), e),
    }
}

pub fn pacman_install(package: &str) {
    println!("{}", format!("Installing {} with pacman...", package).bright_cyan());
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg("--noconfirm")
        .arg(package)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pacman install failed:".bright_red(), e),
    }
}

pub fn pacman_remove(package: &str) {
    println!("{}", format!("Removing {} with pacman...", package).bright_cyan());
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-R")
        .arg("--noconfirm")
        .arg(package)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pacman remove failed:".bright_red(), e),
    }
}

pub fn pacman_update() {
    println!("{}", "Updating system with pacman...".bright_cyan());
    let output = Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .arg("--noconfirm")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pacman update failed:".bright_red(), e),
    }
}

pub fn yay_install(package: &str) {
    println!("{}", format!("Installing {} with yay...", package).bright_cyan());
    let output = Command::new("yay")
        .arg("-S")
        .arg("--noconfirm")
        .arg(package)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Yay install failed:".bright_red(), e),
    }
}

pub fn which(binary: &str) {
    println!("{}", format!("Finding binary {}...", binary).bright_cyan());
    let output = Command::new("which")
        .arg(binary)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Which failed:".bright_red(), e),
    }
}

pub fn whereis(binary: &str) {
    println!("{}", format!("Locating {}...", binary).bright_cyan());
    let output = Command::new("whereis")
        .arg(binary)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Whereis failed:".bright_red(), e),
    }
}

pub fn find_binary(name: &str) {
    println!("{}", format!("Searching for {}...", name).bright_cyan());
    let output = Command::new("find")
        .arg("/usr/bin")
        .arg("/usr/local/bin")
        .arg("-name")
        .arg(format!("*{}*", name))
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Find failed:".bright_red(), e),
    }
}

pub fn locate_binary(name: &str) {
    println!("{}", format!("Locating {}...", name).bright_cyan());
    let output = Command::new("locate")
        .arg(name)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Locate failed:".bright_red(), e),
    }
}

pub fn dpkg_list() {
    println!("{}", "Listing installed packages...".bright_cyan());
    let output = Command::new("dpkg")
        .arg("-l")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Dpkg list failed:".bright_red(), e),
    }
}

pub fn pacman_list() {
    println!("{}", "Listing installed packages...".bright_cyan());
    let output = Command::new("pacman")
        .arg("-Q")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pacman list failed:".bright_red(), e),
    }
} 