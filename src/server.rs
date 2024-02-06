use std::{process, io::Write, sync::{Mutex, mpsc, Arc}, fs::{File, OpenOptions}, thread, time::Duration};
use tokio::{io, net::UnixListener};

use crate::commands;
use crate::process::{ProcessManager, LogManager, Channel};

const SOCKET_PATH: &str = "/tmp/main/multi-tasker.sock";
pub async fn listen() {
    let server = UnixListener::bind(SOCKET_PATH).unwrap();
    let (process_tx, process_rx) = mpsc::channel();
    let (log_tx, log_rx) = mpsc::channel();
    let mut process_manager = ProcessManager {
        processes: Arc::new(Vec::new()),
        channel: Channel {
            sender: process_tx,
            receiver: Arc::new(Mutex::new(process_rx)),
        },
    };
    let mut log_manager = LogManager {
        logs: Arc::new(Vec::new()),
        channel: Channel {
            sender: log_tx,
            receiver: Arc::new(Mutex::new(log_rx)),
        }
    };
    commands::run(
        process_manager.channel.sender.clone(),
        log_manager.channel.sender.clone(),
        &process_manager.processes   
    );
    log_manager.listen();
    process_manager.listen();
    let mut status_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/tmp/multi-tasker/main/status.tmp")
        .unwrap();
    // id, status, pid, user
    status_file.write_all(format!(
        "{}\n{}\n{}\n{}",
        "main",
        "Running".to_string(),
        std::process::id(),
        whoami::username(),
    ).as_bytes());
    println!("Listening...");
    loop {
        match server.accept().await {
            Ok((stream, addr)) => {
                println!("new client");
            },
            Err(e) => {}
        }
    }
    status_file.write_all(format!(
        "{}\n{}\n{}\n{}",
        "main",
        "Stopped".to_string(),
        std::process::id(),
        whoami::username(),
    ).as_bytes());
}
