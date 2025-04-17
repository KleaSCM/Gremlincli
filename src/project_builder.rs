use std::io::{self, Write};

pub fn run() {
    println!("👷‍♀️ Launching Project Builder!");
    loop {
        println!("\nProject Builder Menu:");
        println!("1. Create New Project");
        println!("2. Project Templates");
        println!("3. Project Settings");
        println!("4. Back to Main Menu");

        print!("\nEnter your choice (1–4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => create_new_project(),
            "2" => show_templates(),
            "3" => show_settings(),
            "4" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn create_new_project() {
    println!("Creating new project...");
    // TODO: Implement project creation
}

fn show_templates() {
    println!("Available templates:");
    // TODO: Implement template listing
}

fn show_settings() {
    println!("Project settings:");
    // TODO: Implement settings management
} 