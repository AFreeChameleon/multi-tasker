use std::{sync::{Mutex, mpsc, Arc}, thread, time::Duration};

use process::LogManager;

mod commands;
mod process;

fn main() {
    let (process_tx, process_rx) = mpsc::channel();
    let (log_tx, log_rx) = mpsc::channel();
    let mut process_manager = process::ProcessManager {
        process_channel: process::Channel {
            sender: process_tx,
            receiver: Arc::new(Mutex::new(process_rx)),
        },
        processes: Arc::new(Vec::new()),
    };
    let mut log_manager = process::LogManager {
        logs: Arc::new(Vec::new()),
        log_channel: process::Channel {
            sender: log_tx,
            receiver: Arc::new(Mutex::new(log_rx)),
        }
    };
    commands::run(
        process_manager.process_channel.sender.clone(),
        log_manager.log_channel.sender.clone(),
        &process_manager.processes   
    );
    log_manager.log_listen();
    process_manager.process_listen();
}
