use std::{io::Write, sync::{Mutex, mpsc, Arc}, fs::{File, OpenOptions}, thread, time::Duration};
use tokio::{io, net::UnixListener};
use daemonize::Daemonize;
use whoami;

mod commands;
mod process;
mod server;
mod client;

#[tokio::main]
async fn main() {
    let status_file = std::fs::read_to_string("/tmp/multi-tasker/main/status.tmp")
        .expect("Status file does not exist."); 
    
    let stdout = File::create("/tmp/multi-tasker/main/daemon.out").unwrap();
    let stderr = File::create("/tmp/multi-tasker/main/daemon.err").unwrap();
    let daemonize = Daemonize::new()
        .user("bean")
        .umask(0o112)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => server::listen().await,
        Err(e) => eprintln!("Error, {}", e)
    };
}

