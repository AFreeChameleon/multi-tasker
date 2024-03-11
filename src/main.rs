use std::env::args;
use commands::{create, delete, ls, start, stop, logs, help, restart, health};
use colored::Colorize;

mod commands;
mod platform_lib;

const NO_MODE_TEXT: &str = "No mode given, usage: mult [start|stop|ls] [command|task id]\n
For a full list of commands: mult help";
fn main() {
    if let Some(mode) = args().nth(1) {
        if let Err(message) = match mode.as_str() {
            "create" => create::run(),
            "start" => start::run(),
            "stop" => stop::run(),
            "restart" => restart::run(),
            "logs" => logs::run(),
            "delete" => delete::run(),
            "help" => help::run(),
            "ls" => ls::run(),
            "health" => health::run(),
            _ => Err("Command not found.".to_string())
        } {
            println!("{} {message}", "Error:".red());
        }
    } else {
        println!("{NO_MODE_TEXT}");
    }
}

