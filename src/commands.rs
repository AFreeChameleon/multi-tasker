use std::env::{args};

use crate::process::ProcessManager;
#[path = "commands/start.rs"] mod start;


pub fn run(&manager: &ProcessManager) {
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");
    match mode.as_str() {
        "start" => start::run(&command, &manager),
        _ => println!("Nuffin here")
    };

}
