use std::fs::OpenOptions;
use std::process::{Command, Stdio, ChildStdout, ChildStderr};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, stdout, stderr};
use std::sync::{mpsc::Sender, Arc};
use std::thread;
use chrono::{DateTime, Duration, Utc};

use crate::process::{Channel, Log, Process};

pub fn run(
    command_ref: &String,
    process_sender: Sender<Process>,
    log_sender: Sender<Log>,
    processes: Vec<Process>
) {
    let command = command_ref.clone();

    thread::spawn(move || {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("logs.log")
            .unwrap();
        let child = Command::new("cmd")
            .args(&["/C", &command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("The spawn messed up");
        let stdout = child
            .stdout
            .ok_or_else(|| 
                Error::new(
                    ErrorKind::Other,
                    "Could not capture standard output."
                )
            ).expect("Error getting stdout.");
        let stderr = child
            .stderr
            .ok_or_else(|| 
                Error::new(
                    ErrorKind::Other,
                    "Could not capture standard error."
                )
            ).expect("Error getting stderr.");

        let last_process = processes.last();
        let id = match last_process {
            Some(..) => last_process.unwrap().id + 1,
            None => 0
        };
        let process = Process {
            id,
            command,
            started_at: Utc::now(),
            pid: 0,
            status: "Running".to_string(),
            cpu_usage: 3.02,
            memory_usage: 1000,
            user: "root".to_string()
        };
        process_sender.send(process).expect("Error sending process.");
        let stdout_reader: BufReader<ChildStdout> = BufReader::new(stdout);
        stdout_reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                println!("{}", line);
                let log = Log {
                   process_id: id,
                   content: line.clone(),
                   error: false
                };
                log_sender.send(log).expect("Error sending log.");
                if let Err(e) = writeln!(log_file, "{}", line) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            });

        let stderr_reader: BufReader<ChildStderr> = BufReader::new(stderr);
        stderr_reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                println!("{}", line);
                let log = Log {
                   process_id: id,
                   content: line.clone(),
                   error: true
                };
                log_sender.send(log).expect("Error sending log.");
                if let Err(e) = writeln!(log_file, "{}", line) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            });
    });
}
