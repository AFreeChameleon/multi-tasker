use std::{env::{args}, sync::{mpsc::Sender, Arc}};

use crate::process::{Channel, Process, ProcessManager};
#[path = "commands/start.rs"] mod start;


pub fn run(sender: Sender<Process>, processes: &Arc<Vec<Process>>) {
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");
    match mode.as_str() {
        "start" => start::run(&command, sender, Arc::clone(processes).to_vec()),
        _ => println!("Nuffin here")
    };

}
