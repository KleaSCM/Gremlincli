use std::collections::HashMap;

pub fn get_command_names() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Process Management
    map.insert("ps", "Show running processes (classic flat listing)");
    map.insert("htop", "Interactive process viewer");
    map.insert("kill", "Terminate a process gracefully");
    map.insert("kill9", "Terminate like a gremlin on a rampage");
    map.insert("pkill", "Kill processes by name");
    map.insert("pgrep", "Find PIDs by name");

    // Resource Monitoring
    map.insert("free", "Human-readable memory stats");
    map.insert("vmstat", "Memory + process + CPU stats");
    map.insert("iostat", "CPU and disk I/O stats");
    map.insert("mpstat", "Per-core CPU stats");
    map.insert("sar", "Historical system stats");

    // Disk and Storage
    map.insert("df", "Disk space usage (human readable)");
    map.insert("du", "Folder sizes");
    map.insert("ncdu", "Interactive disk usage explorer");
    map.insert("lsblk", "Block devices");
    map.insert("mount", "Show mounted filesystems");
    map.insert("umount", "Unmount a device");

    // Log Diving
    map.insert("journalctl", "System logs (errors)");
    map.insert("journalctl-service", "Logs for specific services");
    map.insert("dmesg", "Kernel ring buffer (boot messages, hardware)");
    map.insert("tail-syslog", "Classic log watching");
    map.insert("auth-log", "Security/auth events");

    // Networking Ninja Tools
    map.insert("ip", "Network interfaces and IPs");
    map.insert("ping", "Basic connectivity testing");
    map.insert("traceroute", "Network path tracing");
    map.insert("netstat", "Open ports and connections");
    map.insert("nmap", "Service scanning");
    map.insert("curl", "HTTP/API testing");
    map.insert("dig", "DNS record checking");

    // Service Summoning / Banishing
    map.insert("start", "Start a service");
    map.insert("stop", "Stop a service");
    map.insert("restart", "Restart a service");
    map.insert("enable", "Enable service at boot");
    map.insert("disable", "Disable service at boot");
    map.insert("status", "Check service status");

    // Security Scans / Secrets Ops
    map.insert("chkrootkit", "Rootkit scanner");
    map.insert("rkhunter", "Rootkit hunter");
    map.insert("lynis", "System security audit");
    map.insert("clamscan", "Virus scanner");
    map.insert("fail2ban", "Show banned IPs");
    map.insert("gpg-encrypt", "Encrypt file with GPG");
    map.insert("gpg-decrypt", "Decrypt GPG file");
    map.insert("openssl-encrypt", "Encrypt file with OpenSSL");
    map.insert("openssl-decrypt", "Decrypt OpenSSL file");

    // Package Gremlin
    map.insert("apt-install", "Install package with apt");
    map.insert("apt-remove", "Remove package with apt");
    map.insert("apt-update", "Update package lists with apt");
    map.insert("pacman-install", "Install package with pacman");
    map.insert("pacman-remove", "Remove package with pacman");
    map.insert("pacman-update", "Update system with pacman");
    map.insert("yay-install", "Install package with yay");
    map.insert("which", "Find binary location");
    map.insert("whereis", "Locate binary and related files");
    map.insert("find-binary", "Search for binaries");
    map.insert("locate-binary", "Quick binary search");
    map.insert("dpkg-list", "List installed packages (Debian)");
    map.insert("pacman-list", "List installed packages (Arch)");

    // Service Management
    map.insert("systemctl-start", "Start a service");
    map.insert("systemctl-stop", "Stop a service");
    map.insert("systemctl-restart", "Restart a service");
    map.insert("systemctl-enable", "Enable a service at boot");
    map.insert("systemctl-disable", "Disable a service at boot");
    map.insert("systemctl-status", "Show system status");
    map.insert("systemctl-list", "List all services");

    map
} 