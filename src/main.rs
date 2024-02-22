use std::env::args;

mod commands;
mod managers;
mod linux;

use commands::{create, delete, ls, start, stop, logs, help};
use managers::{task, command, table};

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
            _ => println!("Command not start|stop|ls")
        };
    } else {
        println!("
            No mode given, usage: mult [start|stop|ls] [command|task id]\n
            For a full list of commands: mult help
        ");
    }
}

