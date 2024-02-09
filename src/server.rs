use std::{
    process,
    io::Write,
    sync::{Mutex, mpsc, Arc},
    fs::{File, OpenOptions},
    thread, time::Duration,
    error::Error
};
use tokio::{io::{self, Interest}, net::{UnixStream, UnixListener, unix::SocketAddr}};

use crate::commands;
use crate::process::{ProcessManager, LogManager, Channel};
use crate::constants::Constants;

pub async fn listen() {
    let mut server = UnixListener::bind(&Constants::get_socket_path()).unwrap();
    let mut status_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&Constants::get_status_file())
        .unwrap();
    println!("{}", std::process::id());
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
            // Ok((stream, addr)) => listen_socket(stream, addr).await.unwrap(),
            Ok((stream, addr)) => {
                println!("NEW CLIENTTTT");
            },
            Err(e) => {
                status_file.write_all(
                    format!(
                        "{}\n{}\n{}\n{}",
                        "main",
                        "Stopped".to_string(),
                        std::process::id(),
                        whoami::username(),
                    ).as_bytes()
                );
            }
        }
    }
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
}

async fn listen_socket(stream: UnixStream, addr: SocketAddr) -> Result<(), Box<dyn Error>>{
    println!("new client");
    loop {
        let ready = stream.ready(Interest::READABLE).await.unwrap();

        if ready.is_readable() {
            let mut data = vec![0; 1024];

            match stream.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes {:?}", n, data);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}
