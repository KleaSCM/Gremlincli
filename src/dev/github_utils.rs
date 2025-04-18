use std::process::Command;
use colored::*;

pub struct GitHubCli {
    is_installed: bool,
    is_authenticated: bool,
}

impl GitHubCli {
    pub fn new() -> Self {
        let is_installed = Self::check_installation();
        let is_authenticated = if is_installed {
            Self::check_authentication()
        } else {
            false
        };

        Self {
            is_installed,
            is_authenticated,
        }
    }

    fn check_installation() -> bool {
        Command::new("which")
            .arg("gh")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn check_authentication() -> bool {
        Command::new("gh")
            .arg("auth")
            .arg("status")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn install(&self) -> bool {
        if self.is_installed {
            return true;
        }

        println!("{}", "Installing GitHub CLI...".bright_cyan());
        
        let install_status = if Command::new("which")
            .arg("apt")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false) {
            Command::new("sudo")
                .args(&["apt", "update"])
                .status()
                .and_then(|_| {
                    Command::new("sudo")
                        .args(&["apt", "install", "-y", "gh"])
                        .status()
                })
        } else if Command::new("which")
            .arg("pacman")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false) {
            Command::new("sudo")
                .args(&["pacman", "-Syu", "--noconfirm"])
                .status()
                .and_then(|_| {
                    Command::new("sudo")
                        .args(&["pacman", "-S", "--noconfirm", "github-cli"])
                        .status()
                })
        } else {
            println!("{}", "⚠️ Unsupported package manager. Please install GitHub CLI manually.".bright_red());
            return false;
        };

        match install_status {
            Ok(status) if status.success() => {
                println!("{}", "✅ GitHub CLI installed successfully!".bright_green());
                true
            },
            _ => {
                println!("{}", "⚠️ Failed to install GitHub CLI. Please install it manually.".bright_red());
                false
            }
        }
    }

    pub fn authenticate(&self) -> bool {
        if !self.is_installed {
            println!("{}", "⚠️ GitHub CLI is not installed.".bright_red());
            return false;
        }

        if self.is_authenticated {
            return true;
        }

        println!("{}", "Let's authenticate with GitHub...".bright_cyan());
        let auth_status = Command::new("gh")
            .args(&["auth", "login"])
            .status();
            
        match auth_status {
            Ok(status) if status.success() => {
                println!("{}", "✅ GitHub authentication successful!".bright_green());
                true
            },
            _ => {
                println!("{}", "⚠️ GitHub authentication failed. Please run 'gh auth login' manually.".bright_red());
                false
            }
        }
    }

    pub fn create_repository(&self, project_path: &str, is_private: bool) -> Result<(), String> {
        if !self.is_installed || !self.is_authenticated {
            return Err("GitHub CLI not properly set up".to_string());
        }

        let repo_name = std::path::Path::new(project_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        
        // Create GitHub repository
        Command::new("gh")
            .args(&["repo", "create", repo_name, if is_private { "--private" } else { "--public" }])
            .current_dir(project_path)
            .status()
            .map_err(|e| format!("Failed to create GitHub repository: {}", e))?;

        // Push to GitHub
        Command::new("git")
            .args(&["push", "-u", "origin", "main"])
            .current_dir(project_path)
            .status()
            .map_err(|e| format!("Failed to push to GitHub: {}", e))?;

        // Get the GitHub username and repository URL
        let username_output = Command::new("gh")
            .arg("api")
            .arg("user")
            .arg("--jq")
            .arg(".login")
            .output()
            .map_err(|e| format!("Failed to get GitHub username: {}", e))?;
        
        let username = String::from_utf8_lossy(&username_output.stdout).trim().to_string();
        let repo_url = format!("https://github.com/{}/{}", username, repo_name);
        
        println!("{}", "✅ GitHub repository created successfully!".bright_green());
        println!("{} {}", "Repository URL:".bright_cyan(), repo_url.bright_blue().underline());

        Ok(())
    }
} 