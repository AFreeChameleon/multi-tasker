use std::{sync::{Mutex, mpsc, Arc}, fs::File, thread, time::Duration};
use tokio::{io, net::UnixListener};
use daemonize::Daemonize;

mod commands;
mod process;

#[tokio::main]
async fn main() {
    let stdout = File::create("/tmp/main/daemon.out").unwrap();
    let stderr = File::create("/tmp/main/daemon.err").unwrap();
    let daemonize = Daemonize::new()
        .pid_file("/tmp/main/main.pid")
        .chown_pid_file(true)
        .user("nobody")
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => listen().await,
        Err(e) => eprintln!("Error, {}", e)
    };
}

const SOCKET_PATH: &str = "/tmp/multi-tasker.sock";
async fn listen() {
    let server = UnixListener::bind(SOCKET_PATH).unwrap();
    let (process_tx, process_rx) = mpsc::channel();
    let (log_tx, log_rx) = mpsc::channel();
    let mut process_manager = process::ProcessManager {
        processes: Arc::new(Vec::new()),
        channel: process::Channel {
            sender: process_tx,
            receiver: Arc::new(Mutex::new(process_rx)),
        },
    };
    let mut log_manager = process::LogManager {
        logs: Arc::new(Vec::new()),
        channel: process::Channel {
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
    loop {
        match server.accept().await {
            Ok((stream, addr)) => {
                println!("new client");
            },
            Err(e) => {}
        }
    }
}

