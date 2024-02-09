use std::{
    env,
    io,
    path::Path,
    fs::{File, OpenOptions},
    error::Error
};
use tokio::{io::Interest, net::UnixStream};
use procfs::{ProcError, process::Process};

use crate::constants::Constants;

pub fn check_server_exists() -> bool {
    if !Path::new(&Constants::get_status_file()).exists() {
        return false;
    }
    let tmp_file = std::fs::read_to_string(&Constants::get_status_file())
        .expect("Error while opening status file.");
    let stats: Vec<&str> = tmp_file.split("\n").collect(); 
    let server_process_pid: i32 = match stats[2].to_string().parse() {
        Ok(pid) => pid,
        Err(_) => -1
    };
    let server_process: bool = match Process::new(server_process_pid) {
        Ok(process) => true,
        Err(_) => false
    };
    // return stats[1] == "Running";
    return server_process;
}

pub async fn send() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let command = format!("{} {}", &args[1], &args[2]);

    println!("Connecting to socket...");
    let stream = UnixStream::connect(&Constants::get_socket_path()).await.unwrap();
    let ready = stream.ready(Interest::WRITABLE).await.unwrap();
    println!("Ready to write");

    loop {
        if ready.is_writable() {
            match stream.try_write(command.as_bytes()) {
                Ok(n) => {
                    println!("Written {} bytes", n);
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}
