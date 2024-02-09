use std::{io::Write, sync::{Mutex, mpsc, Arc}, fs::{self, File, OpenOptions}, thread, time::Duration};
use tokio::{io, net::UnixListener};
use daemonize::Daemonize;
use whoami;

mod commands;
mod process;
mod server;
mod client;
mod constants;

#[tokio::main]
async fn main() {
    let server_exists = client::check_server_exists();
   
    if server_exists {
        println!("SERVER EXISTS");
        client::send();
    } else {
        fs::create_dir_all("/tmp/multi-tasker/main").unwrap();
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

}

