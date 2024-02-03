use std::{env::{args}, sync::mpsc::Sender};

use crate::process::{Channel, Process, ProcessManager};
#[path = "commands/start.rs"] mod start;


pub fn run(sender: Sender<Process>) {
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");
    // let process_channel = manager.process_channel.clone();
    match mode.as_str() {
        "start" => start::run(&command, sender),
        _ => println!("Nuffin here")
    };

}
