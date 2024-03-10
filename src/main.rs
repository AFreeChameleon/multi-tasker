use std::env::args;
use commands::{create, delete, ls, start, stop, logs, help, restart};

mod commands;
mod platform_lib;

const NO_MODE_TEXT: &str = "No mode given, usage: mult [start|stop|ls] [command|task id]\n
For a full list of commands: mult help";
fn main() {
    if let Some(mode) = args().nth(1) {
        match mode.as_str() {
            "create" => match create::run() {
                Ok(()) => println!("Process created."),
                Err(message) => println!("{}", message)
            },
            "start" => match start::run() {
                Ok(()) => println!("Process started."),
                Err(message) => println!("{}", message)
            },
            "stop" => match stop::run() {
                Ok(()) => println!("Process stopped."),
                Err(message) => println!("{}", message)
            },
            "restart" => match restart::run() {
                Ok(()) => println!("Process restarted."),
                Err(message) => println!("{}", message)
            },
            "logs" => match logs::run() {
                Ok(()) => println!("Logs stopped."),
                Err(message) => println!("{}", message)
            },
            "delete" => match delete::run() {
                Ok(()) => println!("Process deleted."),
                Err(message) => println!("{}", message)
            },
            "help" => match help::run() {
                Ok(()) => (),
                Err(message) => println!("{}", message)
            },
            "ls" => match ls::run() {
                Ok(()) => (),
                Err(message) => println!("{}", message)
            },
            _ => println!("Command not found.")
        };
    } else {
        println!("{NO_MODE_TEXT}");
    }
}

