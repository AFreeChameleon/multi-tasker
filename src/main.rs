use std::env::args;

mod commands;
mod manager;
mod linux;

use commands::{create, delete, ls, start, stop};
use manager::{task, command, table};

fn main() {
    if let Some(mode) = args().nth(1) {
        match mode.as_str() {
            "create" => match create::run() {
                Ok(()) => println!("Command finished."),
                Err(message) => println!("{}", message)
            },
            "start" => match start::run() {
                Ok(()) => println!("Process finished."),
                Err(message) => println!("{}", message)
            },
            "stop" => match stop::run() {
                Ok(()) => println!("Process stopped."),
                Err(message) => println!("{}", message)
            },
            "delete" => match delete::run() {
                Ok(()) => println!("Process deleted."),
                Err(message) => println!("{}", message)
            },
            "ls" => match ls::run() {
                Ok(()) => (),
                Err(message) => println!("{}", message)
            },
            "test" => table::TableManager::print_test(),
            _ => println!("Command not start|stop|ls")
        };
    } else {
        println!("No mode given, usage: mult [start|stop|ls] [command|task id]");
    }
}

