use std::io::{self, Write};
use std::fs;
use std::process::Command;
use colored::*;
use crate::dev::github_utils::GitHubCli;

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

fn load_template_ascii(template: &str) -> String {
    match template {
        "standard" => load_ascii("ascii/goGirl.txt"),
        "gin" => load_ascii("ascii/ginGirl.txt"),
        "cobra" => load_ascii("ascii/cobraGirl.txt"),
        "echo" => load_ascii("ascii/echoGirl.txt"),
        _ => load_ascii("ascii/goGirl.txt"),
    }
}

fn initialize_git(project_path: &str, is_private: bool) -> Result<(), String> {
    // Initialize git repository
    Command::new("git")
        .arg("init")
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to initialize git: {}", e))?;

    // Create initial commit
    Command::new("git")
        .args(&["add", "."])
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to add files: {}", e))?;

    Command::new("git")
        .args(&["commit", "-m", "Gremlin Project init commit"])
        .current_dir(project_path)
        .status()
        .map_err(|e| format!("Failed to create initial commit: {}", e))?;

    if is_private {
        // Create private GitHub repository
        let github = GitHubCli::new();
        if let Ok(()) = github.authenticate() {
            if let Some(_username) = github.get_username() {
                let _repo_name = project_path.split('/').last().unwrap_or("new-project");
                github.create_repository(project_path, true)
                    .map_err(|e| format!("Failed to create repository: {}", e))?;
            }
        }
    }

    Ok(())
}

fn generate_gitignore() -> String {
    r#"# Binaries for programs and plugins
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test binary, built with `go test -c`
*.test

# Output of the go coverage tool, specifically when used with LiteIDE
*.out

# Dependency directories (remove the comment below to include it)
# vendor/

# Go workspace file
go.work

# IDE specific files
.idea/
.vscode/
*.swp
*.swo
.DS_Store

# Environment variables
.env
.env.local

# Debug files
*.log
*.dSYM/

# Build output
dist/
build/
out/

# Generated documentation
/doc/
/docs/

# Temporary files
*.tmp
*.temp
/tmp/
"#.to_string()
}

fn generate_readme(project_name: &str, template: &str) -> String {
    match template {
        "standard" => format!(r#"# {}

A Go project created with Gremlin CLI.

## Getting Started

### Prerequisites

- Go (latest stable version)

### Installation

```bash
git clone <repository-url>
cd {}
go mod download
```

### Running

```bash
go run main.go
```

### Testing

```bash
go test ./...
```

## Project Structure

```
{}
├── cmd/
│   └── main.go
├── internal/
│   ├── handlers/
│   ├── models/
│   └── services/
├── pkg/
│   ├── config/
│   └── utils/
├── go.mod
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name),
        "gin" => format!(r#"# {}

A Gin Web API project created with Gremlin CLI.

## Getting Started

### Prerequisites

- Go (latest stable version)

### Installation

```bash
git clone <repository-url>
cd {}
go mod download
```

### Running

```bash
go run cmd/main.go
```

### Testing

```bash
go test ./...
```

## Project Structure

```
{}
├── cmd/
│   └── main.go
├── internal/
│   ├── handlers/
│   │   └── user_handler.go
│   ├── models/
│   │   └── user.go
│   └── services/
│       └── user_service.go
├── pkg/
│   ├── config/
│   │   └── config.go
│   └── utils/
│       └── response.go
├── go.mod
└── README.md
```

## API Endpoints

- GET /api/v1/users - Get all users
- GET /api/v1/users/:id - Get user by ID
- POST /api/v1/users - Create new user
- PUT /api/v1/users/:id - Update user
- DELETE /api/v1/users/:id - Delete user

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name),
        "cobra" => format!(r#"# {}

A Cobra CLI project created with Gremlin CLI.

## Getting Started

### Prerequisites

- Go (latest stable version)
- Git

### Installation

```bash
git clone <repository-url>
cd {}
go mod download
```

### Building

```bash
go build -o {} cmd/main.go
```

### Running

```bash
./{} [command]
```

### Available Commands

- `{} version` - Show version information
- `{} config` - Manage configuration
- `{} [subcommand]` - Run subcommand

## Project Structure

```
{}
├── cmd/
│   ├── main.go
│   ├── root.go
│   ├── version.go
│   └── config.go
├── internal/
│   ├── commands/
│   └── utils/
├── pkg/
│   ├── config/
│   └── utils/
├── go.mod
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name, project_name, project_name, project_name, project_name, project_name),
        "echo" => format!(r#"# {}

An Echo Web API project created with Gremlin CLI.

## Getting Started

### Prerequisites

- Go (latest stable version)

### Installation

```bash
git clone <repository-url>
cd {}
go mod download
```

### Running

```bash
go run cmd/main.go
```

### Testing

```bash
go test ./...
```

## Project Structure

```
{}
├── cmd/
│   └── main.go
├── internal/
│   ├── handlers/
│   │   └── user_handler.go
│   ├── models/
│   │   └── user.go
│   └── services/
│       └── user_service.go
├── pkg/
│   ├── config/
│   │   └── config.go
│   └── utils/
│       └── response.go
├── go.mod
└── README.md
```

## API Endpoints

- GET /api/v1/users - Get all users
- GET /api/v1/users/:id - Get user by ID
- POST /api/v1/users - Create new user
- PUT /api/v1/users/:id - Update user
- DELETE /api/v1/users/:id - Delete user

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name),
        _ => format!(r#"# {}

A Go project created with Gremlin CLI.

## Getting Started

### Prerequisites

- Go (latest stable version)

### Installation

```bash
git clone <repository-url>
cd {}
go mod download
```

### Running

```bash
go run main.go
```

### Testing

```bash
go test ./...
```

## Project Structure

```
{}
├── cmd/
│   └── main.go
├── internal/
│   ├── handlers/
│   ├── models/
│   └── services/
├── pkg/
│   ├── config/
│   └── utils/
├── go.mod
└── README.md
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
"#, project_name, project_name, project_name)
    }
}

fn create_standard_project(project_name: &str, with_git: bool) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", load_template_ascii("standard").bright_cyan());
    println!("{}", format!("Creating standard Go project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/cmd", project_path),
        format!("{}/internal", project_path),
        format!("{}/internal/handlers", project_path),
        format!("{}/internal/models", project_path),
        format!("{}/internal/services", project_path),
        format!("{}/pkg", project_path),
        format!("{}/pkg/config", project_path),
        format!("{}/pkg/utils", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.go
    let main_go = r#"package main

import (
	"fmt"
	"log"
)

func main() {
	fmt.Println("Hello, Gremlin!")
	log.Println("Application started")
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/main.go", project_path), main_go) {
        println!("{} {}", "Failed to create main.go:".bright_red(), e);
        return;
    }
    
    // Create go.mod
    let go_mod = format!(r#"module {}

go 1.21

require (
	github.com/sirupsen/logrus v1.9.3
)"#, project_name);
    
    if let Err(e) = std::fs::write(format!("{}/go.mod", project_path), go_mod) {
        println!("{} {}", "Failed to create go.mod:".bright_red(), e);
        return;
    }
    
    // Add README.md and .gitignore
    if let Err(e) = fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "standard")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }

    if let Err(e) = fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    if with_git {
        print!("{}", "Is this a private repository? (y/n): ".bright_blue());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let is_private = input.trim().to_lowercase() == "y";

        match initialize_git(&project_path, is_private) {
            Ok(_) => println!("{}", "✅ Git repository initialized successfully!".bright_green()),
            Err(e) => println!("{} {}", "Failed to initialize git:".bright_red(), e),
        }
    }
    
    println!("{}", "✅ Standard Go project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
}

fn create_gin_project(project_name: &str, with_git: bool) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", load_template_ascii("gin").bright_cyan());
    println!("{}", format!("Creating Gin Web API project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/cmd", project_path),
        format!("{}/internal", project_path),
        format!("{}/internal/handlers", project_path),
        format!("{}/internal/models", project_path),
        format!("{}/internal/services", project_path),
        format!("{}/pkg", project_path),
        format!("{}/pkg/config", project_path),
        format!("{}/pkg/utils", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.go
    let main_go = r#"package main

import (
	"log"
	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()
	
	// Health check endpoint
	r.GET("/health", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"status": "ok",
		})
	})
	
	// Start server
	if err := r.Run(":8080"); err != nil {
		log.Fatal("Failed to start server: ", err)
	}
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/main.go", project_path), main_go) {
        println!("{} {}", "Failed to create main.go:".bright_red(), e);
        return;
    }
    
    // Create user model
    let user_model = r#"package models

type User struct {
	ID        uint   `json:"id"`
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Email     string `json:"email"`
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/internal/models/user.go", project_path), user_model) {
        println!("{} {}", "Failed to create user.go:".bright_red(), e);
        return;
    }
    
    // Create user handler
    let user_handler = r#"package handlers

import (
	"net/http"
	"github.com/gin-gonic/gin"
)

func GetUsers(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Get all users",
	})
}

func GetUser(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Get user by ID",
	})
}

func CreateUser(c *gin.Context) {
	c.JSON(http.StatusCreated, gin.H{
		"message": "Create user",
	})
}

func UpdateUser(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Update user",
	})
}

func DeleteUser(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Delete user",
	})
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/internal/handlers/user_handler.go", project_path), user_handler) {
        println!("{} {}", "Failed to create user_handler.go:".bright_red(), e);
        return;
    }
    
    // Create go.mod
    let go_mod = format!(r#"module {}

go 1.21

require (
	github.com/gin-gonic/gin v1.9.1
	github.com/sirupsen/logrus v1.9.3
)"#, project_name);
    
    if let Err(e) = std::fs::write(format!("{}/go.mod", project_path), go_mod) {
        println!("{} {}", "Failed to create go.mod:".bright_red(), e);
        return;
    }
    
    // Add README.md and .gitignore
    if let Err(e) = fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "gin")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }

    if let Err(e) = fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    if with_git {
        print!("{}", "Is this a private repository? (y/n): ".bright_blue());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let is_private = input.trim().to_lowercase() == "y";

        match initialize_git(&project_path, is_private) {
            Ok(_) => println!("{}", "✅ Git repository initialized successfully!".bright_green()),
            Err(e) => println!("{} {}", "Failed to initialize git:".bright_red(), e),
        }
    }
    
    println!("{}", "✅ Gin Web API project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
}

fn create_cobra_project(project_name: &str, with_git: bool) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", load_template_ascii("cobra").bright_cyan());
    println!("{}", format!("Creating Cobra CLI project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/cmd", project_path),
        format!("{}/internal", project_path),
        format!("{}/internal/commands", project_path),
        format!("{}/internal/utils", project_path),
        format!("{}/pkg", project_path),
        format!("{}/pkg/config", project_path),
        format!("{}/pkg/utils", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.go
    let main_go = r#"package main

import (
	"{}/cmd"
)

func main() {
	cmd.Execute()
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/main.go", project_path), main_go.replace("{}", project_name)) {
        println!("{} {}", "Failed to create main.go:".bright_red(), e);
        return;
    }
    
    // Create root.go
    let root_go = r#"package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "{}",
	Short: "A brief description of your application",
	Long: `A longer description that spans multiple lines and likely contains
examples and usage of using your application.`,
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		panic(err)
	}
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/root.go", project_path), root_go.replace("{}", project_name)) {
        println!("{} {}", "Failed to create root.go:".bright_red(), e);
        return;
    }
    
    // Create version.go
    let version_go = r#"package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
)

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version number",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("{} v0.1.0")
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/version.go", project_path), version_go.replace("{}", project_name)) {
        println!("{} {}", "Failed to create version.go:".bright_red(), e);
        return;
    }
    
    // Create go.mod
    let go_mod = format!(r#"module {}

go 1.21

require (
	github.com/spf13/cobra v1.7.0
	github.com/sirupsen/logrus v1.9.3
)"#, project_name);
    
    if let Err(e) = std::fs::write(format!("{}/go.mod", project_path), go_mod) {
        println!("{} {}", "Failed to create go.mod:".bright_red(), e);
        return;
    }
    
    // Add README.md and .gitignore
    if let Err(e) = fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "cobra")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }

    if let Err(e) = fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    if with_git {
        print!("{}", "Is this a private repository? (y/n): ".bright_blue());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let is_private = input.trim().to_lowercase() == "y";

        match initialize_git(&project_path, is_private) {
            Ok(_) => println!("{}", "✅ Git repository initialized successfully!".bright_green()),
            Err(e) => println!("{} {}", "Failed to initialize git:".bright_red(), e),
        }
    }
    
    println!("{}", "✅ Cobra CLI project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
}

fn create_echo_project(project_name: &str, with_git: bool) {
    let base_path = "/home/klea/Documents/Dev";
    let project_path = format!("{}/{}", base_path, project_name);
    
    println!("{}", load_template_ascii("echo").bright_cyan());
    println!("{}", format!("Creating Echo Web API project '{}'...", project_name).bright_cyan());
    
    // Create project directory structure
    let dirs = vec![
        format!("{}/cmd", project_path),
        format!("{}/internal", project_path),
        format!("{}/internal/handlers", project_path),
        format!("{}/internal/models", project_path),
        format!("{}/internal/services", project_path),
        format!("{}/pkg", project_path),
        format!("{}/pkg/config", project_path),
        format!("{}/pkg/utils", project_path),
    ];
    
    for dir in dirs {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            println!("{} {}", "Failed to create directory:".bright_red(), e);
            return;
        }
    }
    
    // Create main.go
    let main_go = r#"package main

import (
	"log"
	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()
	
	// Health check endpoint
	e.GET("/health", func(c echo.Context) error {
		return c.JSON(200, map[string]string{
			"status": "ok",
		})
	})
	
	// Start server
	if err := e.Start(":8080"); err != nil {
		log.Fatal("Failed to start server: ", err)
	}
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/cmd/main.go", project_path), main_go) {
        println!("{} {}", "Failed to create main.go:".bright_red(), e);
        return;
    }
    
    // Create user model
    let user_model = r#"package models

type User struct {
	ID        uint   `json:"id"`
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Email     string `json:"email"`
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/internal/models/user.go", project_path), user_model) {
        println!("{} {}", "Failed to create user.go:".bright_red(), e);
        return;
    }
    
    // Create user handler
    let user_handler = r#"package handlers

import (
	"net/http"
	"github.com/labstack/echo/v4"
)

func GetUsers(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]string{
		"message": "Get all users",
	})
}

func GetUser(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]string{
		"message": "Get user by ID",
	})
}

func CreateUser(c echo.Context) error {
	return c.JSON(http.StatusCreated, map[string]string{
		"message": "Create user",
	})
}

func UpdateUser(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]string{
		"message": "Update user",
	})
}

func DeleteUser(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]string{
		"message": "Delete user",
	})
}"#;
    
    if let Err(e) = std::fs::write(format!("{}/internal/handlers/user_handler.go", project_path), user_handler) {
        println!("{} {}", "Failed to create user_handler.go:".bright_red(), e);
        return;
    }
    
    // Create go.mod
    let go_mod = format!(r#"module {}

go 1.21

require (
	github.com/labstack/echo/v4 v4.11.1
	github.com/sirupsen/logrus v1.9.3
)"#, project_name);
    
    if let Err(e) = std::fs::write(format!("{}/go.mod", project_path), go_mod) {
        println!("{} {}", "Failed to create go.mod:".bright_red(), e);
        return;
    }
    
    // Add README.md and .gitignore
    if let Err(e) = fs::write(format!("{}/README.md", project_path), generate_readme(project_name, "echo")) {
        println!("{} {}", "Failed to create README.md:".bright_red(), e);
        return;
    }

    if let Err(e) = fs::write(format!("{}/.gitignore", project_path), generate_gitignore()) {
        println!("{} {}", "Failed to create .gitignore:".bright_red(), e);
        return;
    }
    
    if with_git {
        print!("{}", "Is this a private repository? (y/n): ".bright_blue());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let is_private = input.trim().to_lowercase() == "y";

        match initialize_git(&project_path, is_private) {
            Ok(_) => println!("{}", "✅ Git repository initialized successfully!".bright_green()),
            Err(e) => println!("{} {}", "Failed to initialize git:".bright_red(), e),
        }
    }
    
    println!("{}", "✅ Echo Web API project created successfully!".bright_green());
    println!("{}", format!("Project location: {}", project_path).bright_cyan());
}

fn standard_project_options() {
    println!("\n{}", "Standard Go Project Options:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Create without git".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Create with git and GitHub".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Templates 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-2): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" | "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            create_standard_project(project_name, input.trim() == "2");
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

fn gin_project_options() {
    println!("\n{}", "Gin Web API Project Options:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Create without git".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Create with git and GitHub".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Templates 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-2): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" | "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            create_gin_project(project_name, input.trim() == "2");
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

fn cobra_project_options() {
    println!("\n{}", "Cobra CLI Project Options:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Create without git".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Create with git and GitHub".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Templates 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-2): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" | "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            create_cobra_project(project_name, input.trim() == "2");
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

fn echo_project_options() {
    println!("\n{}", "Echo Web API Project Options:".bright_cyan());
    println!("{} {}", "1.".bright_green(), "Create without git".bright_magenta());
    println!("{} {}", "2.".bright_green(), "Create with git and GitHub".bright_magenta());
    println!("\n{} {}", "0.".bright_green(), "Back to Templates 🔙".bright_blue());

    print!("\n{}", "Enter choice (0-2): ".bright_blue());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "0" => return,
        "1" | "2" => {
            print!("{}", "Enter project name: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut project_name = String::new();
            io::stdin().read_line(&mut project_name).unwrap();
            let project_name = project_name.trim();
            
            create_echo_project(project_name, input.trim() == "2");
        },
        _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
    }
}

pub fn run() {
    let splash_art = load_ascii("ascii/GOgirl.txt");
    println!("{}", splash_art.bright_cyan());
    println!("{}", "🐹 Go Project Dashboard 🐹".bright_cyan().bold().blink());
    
    loop {
        println!("\n{}", "Go Project Menu:".bright_cyan());
        println!("{} {}", "1.".bright_green(), "Create New Project 🏗️".bright_cyan());
        println!("{} {}", "2.".bright_green(), "Return to Project Builder 🔙".bright_blue());
        println!("{} {}", "3.".bright_green(), "Return to Gremlin Dashboard 🏠".bright_purple());

        print!("\n{}", "Enter your choice (1-3): ".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                println!("\n{}", "Go Project Templates:".bright_cyan());
                println!("{} {}", "1.".bright_green(), "Standard Go Project 🐹".bright_cyan());
                println!("{} {}", "2.".bright_green(), "Gin Web API Project 🌐".bright_cyan());
                println!("{} {}", "3.".bright_green(), "Cobra CLI Project 🛠️".bright_cyan());
                println!("{} {}", "4.".bright_green(), "Echo Web API Project 🌐".bright_cyan());
                println!("{} {}", "5.".bright_green(), "Back to Go Dashboard 🔙".bright_blue());

                print!("\n{}", "Select template (1-5): ".bright_blue());
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim() {
                    "1" => standard_project_options(),
                    "2" => gin_project_options(),
                    "3" => cobra_project_options(),
                    "4" => echo_project_options(),
                    "5" => continue,
                    _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
                }
            },
            "2" => {
                println!("{}", "Returning to Project Builder...".bright_blue());
                return;
            },
            "3" => {
                println!("{}", "Returning to Gremlin Dashboard...".bright_purple());
                return;
            },
            _ => println!("{}", "⚠️ Invalid choice. Try again.".bright_red()),
        }
    }
}

