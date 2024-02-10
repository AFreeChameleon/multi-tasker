use std::{env::args, io::Write, sync::{Mutex, mpsc, Arc}, fs::{self, File, OpenOptions}, thread, time::Duration};
use daemonize::Daemonize;
use whoami;

mod start;
mod stop;
mod task;
mod command;

fn main() {
    if let Some(mode) = args().nth(1) {
        match mode.as_str() {
            "start" => match start::run() {
                Ok(()) => println!("Command finished."),
                Err(message) => println!("{}", message)
            },
            "stop" => match stop::run() {
                Ok(()) => println!("Process stopped."),
                Err(message) => println!("{}", message)
            },
            _ => println!("Command not start|stop|ls")
        };
    } else {
        println!("No mode given, usage: mult [start|stop|ls] [command|task id]");
    }
}

