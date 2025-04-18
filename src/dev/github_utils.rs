use std::process::Command;
use std::path::Path;
use colored::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitHubError {
    #[error("GitHub CLI not installed")]
    NotInstalled,
    #[error("GitHub CLI not authenticated")]
    NotAuthenticated,
    #[error("Command execution failed: {0}")]
    CommandFailed(String),
    #[error("Repository already exists: {0}")]
    RepositoryExists(String),
    #[error("Invalid repository name: {0}")]
    InvalidRepoName(String),
    #[error("Failed to get GitHub username")]
    UsernameError,
    #[error("Failed to create repository: {0}")]
    CreateRepoError(String),
    #[error("Failed to push to repository: {0}")]
    PushError(String),
}

pub struct GitHubCli {
    is_installed: bool,
    is_authenticated: bool,
    username: Option<String>,
    use_ssh: bool,
    interactive: bool,
}

impl GitHubCli {
    pub fn new() -> Self {
        let is_installed = Self::check_installation();
        let (is_authenticated, username) = if is_installed {
            Self::check_authentication()
        } else {
            (false, None)
        };

        Self {
            is_installed,
            is_authenticated,
            username,
            use_ssh: Self::check_ssh_key(),
            interactive: true,
        }
    }

    #[allow(dead_code)]
    pub fn with_ssh(mut self, use_ssh: bool) -> Self {
        self.use_ssh = use_ssh;
        self
    }

    #[allow(dead_code)]
    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    fn check_installation() -> bool {
        Command::new("which")
            .arg("gh")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn check_authentication() -> (bool, Option<String>) {
        let status = Command::new("gh")
            .arg("auth")
            .arg("status")
            .output();
        
        if let Ok(output) = status {
            if output.status.success() {
                // Try to get username
                let username = Command::new("gh")
                    .arg("api")
                    .arg("user")
                    .arg("--jq")
                    .arg(".login")
                    .output()
                    .ok()
                    .and_then(|out| String::from_utf8(out.stdout).ok())
                    .map(|s| s.trim().to_string());
                
                return (true, username);
            }
        }
        (false, None)
    }

    fn check_ssh_key() -> bool {
        Command::new("ssh")
            .args(&["-T", "git@github.com"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn check_repo_exists(&self, repo_name: &str) -> Result<bool, GitHubError> {
        let username = self.username.as_deref()
            .ok_or(GitHubError::UsernameError)?;

        let check_repo = Command::new("gh")
            .args(&["api", &format!("/repos/{}/{}", username, repo_name)])
            .output();

        match check_repo {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false)
        }
    }

    #[allow(dead_code)]
    pub fn get_username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn install(&self) -> Result<(), GitHubError> {
        if self.is_installed {
            return Ok(());
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
            return Err(GitHubError::NotInstalled);
        };

        match install_status {
            Ok(status) if status.success() => {
                println!("{}", "✅ GitHub CLI installed successfully!".bright_green());
                Ok(())
            },
            _ => {
                println!("{}", "⚠️ Failed to install GitHub CLI. Please install it manually.".bright_red());
                Err(GitHubError::NotInstalled)
            }
        }
    }

    pub fn authenticate(&self) -> Result<(), GitHubError> {
        if !self.is_installed {
            return Err(GitHubError::NotInstalled);
        }

        if self.is_authenticated {
            return Ok(());
        }

        println!("{}", "Let's authenticate with GitHub...".bright_cyan());
        println!("{}", "A browser window will open for authentication.".bright_cyan());
        println!("{}", "Please follow the prompts to complete the authentication.".bright_cyan());
        
        let auth_status = Command::new("gh")
            .args(&["auth", "login", "--web"])
            .status();
            
        match auth_status {
            Ok(status) if status.success() => {
                println!("{}", "✅ GitHub authentication successful!".bright_green());
                Ok(())
            },
            _ => {
                println!("{}", "⚠️ GitHub authentication failed. Please try again.".bright_red());
                Err(GitHubError::NotAuthenticated)
            }
        }
    }

    pub fn create_repository(&self, project_path: &str, is_private: bool) -> Result<String, GitHubError> {
        if !self.is_installed {
            return Err(GitHubError::NotInstalled);
        }

        if !self.is_authenticated {
            return Err(GitHubError::NotAuthenticated);
        }

        let repo_name = Path::new(project_path)
            .file_name()
            .ok_or_else(|| GitHubError::InvalidRepoName("Invalid project path".to_string()))?
            .to_str()
            .ok_or_else(|| GitHubError::InvalidRepoName("Invalid repository name".to_string()))?;

        // Check if repository already exists
        if self.check_repo_exists(repo_name)? {
            return Err(GitHubError::RepositoryExists(repo_name.to_string()));
        }

        // Create GitHub repository
        let mut create_args = vec!["repo", "create", repo_name];
        if is_private {
            create_args.push("--private");
        } else {
            create_args.push("--public");
        }
        if self.interactive {
            create_args.push("--clone");
            create_args.push("--source=.");
            create_args.push("--push");
        }

        let create_repo = Command::new("gh")
            .args(&create_args)
            .current_dir(project_path)
            .status()
            .map_err(|e| GitHubError::CreateRepoError(e.to_string()))?;

        if !create_repo.success() {
            return Err(GitHubError::CreateRepoError("Failed to create repository".to_string()));
        }

        // Initialize git if not already initialized
        if !Path::new(&format!("{}/.git", project_path)).exists() {
            Command::new("git")
                .arg("init")
                .current_dir(project_path)
                .status()
                .map_err(|e| GitHubError::CommandFailed(e.to_string()))?;

            Command::new("git")
                .args(&["add", "."])
                .current_dir(project_path)
                .status()
                .map_err(|e| GitHubError::CommandFailed(e.to_string()))?;

            Command::new("git")
                .args(&["commit", "-m", "Initial commit"])
                .current_dir(project_path)
                .status()
                .map_err(|e| GitHubError::CommandFailed(e.to_string()))?;
        }

        // Add remote and push
        self.add_remote(project_path, repo_name)?;
        self.push_to_remote(project_path, "main")?;

        let username = self.username.as_deref()
            .ok_or(GitHubError::UsernameError)?;
        let repo_url = format!("https://github.com/{}/{}", username, repo_name);
        
        println!("{}", "✅ GitHub repository created successfully!".bright_green());
        println!("{} {}", "Repository URL:".bright_cyan(), repo_url.bright_blue().underline());

        Ok(repo_url)
    }

    fn add_remote(&self, project_path: &str, repo_name: &str) -> Result<(), GitHubError> {
        let username = self.username.as_deref()
            .ok_or(GitHubError::UsernameError)?;
        
        let remote_url = if self.use_ssh {
            format!("git@github.com:{}/{}.git", username, repo_name)
        } else {
            format!("https://github.com/{}/{}.git", username, repo_name)
        };
        
        Command::new("git")
            .args(&["remote", "add", "origin", &remote_url])
            .current_dir(project_path)
            .status()
            .map_err(|e| GitHubError::CommandFailed(e.to_string()))?;

        Ok(())
    }

    pub fn push_to_remote(&self, project_path: &str, branch: &str) -> Result<(), GitHubError> {
        Command::new("git")
            .args(&["push", "-u", "origin", branch])
            .current_dir(project_path)
            .status()
            .map_err(|e| GitHubError::PushError(e.to_string()))?;

        Ok(())
    }
} 