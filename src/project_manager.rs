use std::io::{self, Write};

pub fn run() {
    loop {
        println!("\nProject Manager Menu:");
        println!("1. List Projects");
        println!("2. Open Project");
        println!("3. Delete Project");
        println!("4. Back to Main Menu");

        print!("\nEnter your choice (1–4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => list_projects(),
            "2" => open_project(),
            "3" => delete_project(),
            "4" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn list_projects() {
    println!("Available projects:");
    // TODO: Implement project listing
}

fn open_project() {
    println!("Opening project...");
    // TODO: Implement project opening
}

fn delete_project() {
    println!("Deleting project...");
    // TODO: Implement project deletion
} 