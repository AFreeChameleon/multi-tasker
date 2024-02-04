use std::{sync::{Mutex, mpsc, Arc}, thread, time::Duration};

mod commands;
mod process;

fn main() {
    let (process_tx, process_rx) = mpsc::channel();
    let mut process_manager = process::ProcessManager {
        process_channel: process::Channel {
            sender: process_tx,
            receiver: Arc::new(Mutex::new(process_rx)),
        },
        processes: Arc::new(Vec::new())
    };
    commands::run(
        process_manager.process_channel.sender.clone(),
        &process_manager.processes   
    );
    process_manager.process_listen();
    thread::sleep(Duration::from_secs(5));
}
