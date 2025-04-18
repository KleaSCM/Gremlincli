pub mod command_logic;
pub mod command_matcher;
pub mod command_names;
pub mod command_categories;
pub mod networking;
pub mod process;
pub mod resource;
pub mod disk;
pub mod logs;
pub mod services;
pub mod security;
pub mod packages;

pub fn run() {
    command_matcher::run();
} 