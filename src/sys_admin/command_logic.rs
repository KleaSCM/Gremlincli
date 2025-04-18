use std::process::Command;
use colored::*;

pub fn ip_a() {
    println!("{}", "Checking network interfaces and IPs...".bright_cyan());
    let output = Command::new("ip")
        .arg("a")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "IP command failed:".bright_red(), e),
    }
}

pub fn ping(target: &str) {
    println!("{}", format!("Pinging {}...", target).bright_cyan());
    let output = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg(target)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Ping failed:".bright_red(), e),
    }
}

pub fn traceroute(target: &str) {
    println!("{}", format!("Tracing route to {}...", target).bright_cyan());
    let output = Command::new("traceroute")
        .arg(target)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Traceroute failed:".bright_red(), e),
    }
}

pub fn netstat() {
    println!("{}", "Checking open ports...".bright_cyan());
    let output = Command::new("netstat")
        .arg("-tuln")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Netstat failed:".bright_red(), e),
    }
}

pub fn nmap(target: &str) {
    println!("{}", format!("Scanning {} for open services...", target).bright_cyan());
    let output = Command::new("nmap")
        .arg(target)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Nmap failed:".bright_red(), e),
    }
}

pub fn curl(url: &str) {
    println!("{}", format!("Testing HTTP connection to {}...", url).bright_cyan());
    let output = Command::new("curl")
        .arg("-I")
        .arg(url)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Curl failed:".bright_red(), e),
    }
}

pub fn dig(domain: &str) {
    println!("{}", format!("Checking DNS records for {}...", domain).bright_cyan());
    let output = Command::new("dig")
        .arg(domain)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
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

pub fn kill_graceful(pid: &str) {
    println!("{}", format!("Attempting to kill {} gracefully...", pid).bright_cyan());
    let output = Command::new("kill")
        .arg(pid)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Kill failed:".bright_red(), e),
    }
}

pub fn kill_force(pid: &str) {
    println!("{}", format!("Brutally terminating {}...", pid).bright_cyan());
    let output = Command::new("kill")
        .arg("-9")
        .arg(pid)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Kill failed:".bright_red(), e),
    }
}

pub fn pkill(name: &str) {
    println!("{}", format!("Attempting to kill processes named {}...", name).bright_cyan());
    let output = Command::new("pkill")
        .arg(name)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pkill failed:".bright_red(), e),
    }
}

pub fn pgrep(name: &str) {
    println!("{}", format!("Searching for processes named {}...", name).bright_cyan());
    let output = Command::new("pgrep")
        .arg("-l")
        .arg(name)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Pgrep failed:".bright_red(), e),
    }
}

pub fn free() {
    println!("{}", "Checking memory usage...".bright_cyan());
    let output = Command::new("free")
        .arg("-h")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Free failed:".bright_red(), e),
    }
}

pub fn vmstat() {
    println!("{}", "Checking system stats...".bright_cyan());
    let output = Command::new("vmstat")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Vmstat failed:".bright_red(), e),
    }
}

pub fn iostat() {
    println!("{}", "Checking I/O stats...".bright_cyan());
    let output = Command::new("iostat")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Iostat failed:".bright_red(), e),
    }
}

pub fn mpstat() {
    println!("{}", "Checking per-core CPU stats...".bright_cyan());
    let output = Command::new("mpstat")
        .arg("-P")
        .arg("ALL")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Mpstat failed:".bright_red(), e),
    }
}

pub fn sar() {
    println!("{}", "Checking historical system stats...".bright_cyan());
    let output = Command::new("sar")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Sar failed:".bright_red(), e),
    }
}

pub fn df() {
    println!("{}", "Checking disk space usage...".bright_cyan());
    let output = Command::new("df")
        .arg("-h")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Df failed:".bright_red(), e),
    }
}

pub fn du(path: &str) {
    println!("{}", format!("Checking folder sizes in {}...", path).bright_cyan());
    let output = Command::new("du")
        .arg("-sh")
        .arg(path)
        .arg("*")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Du failed:".bright_red(), e),
    }
}

pub fn ncdu(path: &str) {
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
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Lsblk failed:".bright_red(), e),
    }
}

pub fn mount() {
    println!("{}", "Showing mounted filesystems...".bright_cyan());
    let output = Command::new("mount")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Mount failed:".bright_red(), e),
    }
}

pub fn umount(device: &str) {
    println!("{}", format!("Unmounting {}...", device).bright_cyan());
    let output = Command::new("umount")
        .arg(device)
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
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

pub fn fail2ban_status() {
    println!("{}", "Checking fail2ban status...".bright_cyan());
    let output = Command::new("fail2ban-client")
        .arg("status")
        .output();

    match output {
        Ok(out) => println!("{}", String::from_utf8_lossy(&out.stdout).bright_green()),
        Err(e) => println!("{} {}", "Fail2ban status check failed:".bright_red(), e),
    }
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