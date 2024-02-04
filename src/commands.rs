use std::{env::{args}, sync::{mpsc::Sender, Arc}};

use crate::process::{Channel, Log, Process, ProcessManager};
#[path = "commands/start.rs"] mod start;


pub fn run(
    process_sender: Sender<Process>,
    log_sender: Sender<Log>,
    processes: &Arc<Vec<Process>>
) {
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");
    if mode.as_str() == "start" {
        start::run(&command, process_sender, log_sender, Arc::clone(processes).to_vec());
    }
    // match mode.as_str() {
    //     "start" => start::run(&command, sender, Arc::clone(processes).to_vec()),
    //     _ => println!("Nuffin here")
    // };

}
