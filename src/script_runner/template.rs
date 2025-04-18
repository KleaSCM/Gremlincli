use std::fs;
use colored::*;

#[allow(dead_code)]
pub fn run() {
    let splash_art = load_ascii("ascii/TemplateGremlin.txt");
    println!("{}", splash_art.bright_magenta());
    println!("\n{}", "📋 Script Templates 📋".bright_purple().bold().blink());
    
    
}

fn load_ascii(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "✨ [ASCII missing] ✨".to_string())
}

pub fn get_python_template() -> &'static str {
    r#"def main():
    print("Hello, World!")

if __name__ == "__main__":
    main()"#
}

pub fn get_bash_template() -> &'static str {
    r#"#!/bin/bash

main() {
    echo "Hello, World!"
}

main "$@""#
}

pub fn get_lua_template() -> &'static str {
    r#"local function main()
    print("Hello, World!")
end

main()"#
}

pub fn get_powershell_template() -> &'static str {
    r#"function Main {
    Write-Host "Hello, World!"
}

Main"#
}

pub fn get_go_template() -> &'static str {
    r#"package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}"#
}

pub fn get_rust_template() -> &'static str {
    r#"fn main() {
    println!("Hello, World!");
}"#
}

pub fn get_template(language: &str) -> Option<&'static str> {
    match language {
        "Python" => Some(get_python_template()),
        "Bash" => Some(get_bash_template()),
        "Lua" => Some(get_lua_template()),
        "PS1" => Some(get_powershell_template()),
        "Go" => Some(get_go_template()),
        "Rust" => Some(get_rust_template()),
        _ => None,
    }
} 