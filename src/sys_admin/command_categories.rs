use std::collections::HashMap;

pub fn get_command_categories() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    
    // Process Management
    map.insert("ps", "Process Mischief");
    map.insert("htop", "Process Mischief");
    map.insert("kill", "Process Mischief");
    map.insert("kill9", "Process Mischief");
    map.insert("pkill", "Process Mischief");
    map.insert("pgrep", "Process Mischief");

    // Resource Monitoring
    map.insert("free", "Resource Spying");
    map.insert("vmstat", "Resource Spying");
    map.insert("iostat", "Resource Spying");
    map.insert("mpstat", "Resource Spying");
    map.insert("sar", "Resource Spying");

    // Disk and Storage
    map.insert("df", "Disk and Storage Shenanigans");
    map.insert("du", "Disk and Storage Shenanigans");
    map.insert("ncdu", "Disk and Storage Shenanigans");
    map.insert("lsblk", "Disk and Storage Shenanigans");
    map.insert("mount", "Disk and Storage Shenanigans");
    map.insert("umount", "Disk and Storage Shenanigans");

    // Log Diving
    map.insert("journalctl", "Log Diving");
    map.insert("journalctl-service", "Log Diving");
    map.insert("dmesg", "Log Diving");
    map.insert("tail-syslog", "Log Diving");
    map.insert("auth-log", "Log Diving");

    // Networking Ninja Tools
    map.insert("ip", "Networking Ninja Tools");
    map.insert("ping", "Networking Ninja Tools");
    map.insert("traceroute", "Networking Ninja Tools");
    map.insert("netstat", "Networking Ninja Tools");
    map.insert("nmap", "Networking Ninja Tools");
    map.insert("curl", "Networking Ninja Tools");
    map.insert("dig", "Networking Ninja Tools");

    // Service Summoning / Banishing
    map.insert("start", "Service Summoning / Banishing");
    map.insert("stop", "Service Summoning / Banishing");
    map.insert("restart", "Service Summoning / Banishing");
    map.insert("enable", "Service Summoning / Banishing");
    map.insert("disable", "Service Summoning / Banishing");
    map.insert("status", "Service Summoning / Banishing");

    // Security Scans / Secrets Ops
    map.insert("chkrootkit", "Security Scans / Secrets Ops");
    map.insert("rkhunter", "Security Scans / Secrets Ops");
    map.insert("lynis", "Security Scans / Secrets Ops");
    map.insert("clamscan", "Security Scans / Secrets Ops");
    map.insert("fail2ban", "Security Scans / Secrets Ops");
    map.insert("gpg-encrypt", "Security Scans / Secrets Ops");
    map.insert("gpg-decrypt", "Security Scans / Secrets Ops");
    map.insert("openssl-encrypt", "Security Scans / Secrets Ops");
    map.insert("openssl-decrypt", "Security Scans / Secrets Ops");

    // Package Gremlin
    map.insert("apt-install", "Package Gremlin");
    map.insert("apt-remove", "Package Gremlin");
    map.insert("apt-update", "Package Gremlin");
    map.insert("pacman-install", "Package Gremlin");
    map.insert("pacman-remove", "Package Gremlin");
    map.insert("pacman-update", "Package Gremlin");
    map.insert("yay-install", "Package Gremlin");
    map.insert("which", "Package Gremlin");
    map.insert("whereis", "Package Gremlin");
    map.insert("find-binary", "Package Gremlin");
    map.insert("locate-binary", "Package Gremlin");
    map.insert("dpkg-list", "Package Gremlin");
    map.insert("pacman-list", "Package Gremlin");

    // Service Management
    map.insert("systemctl-start", "Service Management");
    map.insert("systemctl-stop", "Service Management");
    map.insert("systemctl-restart", "Service Management");
    map.insert("systemctl-enable", "Service Management");
    map.insert("systemctl-disable", "Service Management");
    map.insert("systemctl-status", "Service Management");
    map.insert("systemctl-list", "Service Management");

    map
} 