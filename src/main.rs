use std::sync::{Mutex, mpsc, Arc};

mod commands;
mod process;

fn main() {
    let (process_tx, process_rx) = mpsc::channel();
    let mut process_manager = process::ProcessManager {
        process_channel: process::Channel {
            sender: process_tx,
            receiver: Arc::new(Mutex::new(process_rx)),
        },
        processes: Vec::new()
    };
    commands::run(process_manager.process_channel.sender.clone());
    process_manager.process_listen();
}
