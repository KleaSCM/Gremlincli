use std::io::{self, Write};

pub fn run() {
    println!("👷‍♀️ Launching Script Runner!");
    loop {
        println!("\nScript Runner Menu:");
        println!("1. Run Script");
        println!("2. List Scripts");
        println!("3. Create Script");
        println!("4. Back to Main Menu");

        print!("\nEnter your choice (1–4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => run_script(),
            "2" => list_scripts(),
            "3" => create_script(),
            "4" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn run_script() {
    println!("Running script...");
    // TODO: Implement script execution
}

fn list_scripts() {
    println!("Available scripts:");
    // TODO: Implement script listing
}

fn create_script() {
    println!("Creating new script...");
    // TODO: Implement script creation
} 