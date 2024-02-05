use std::fs::{self, OpenOptions};
use std::process::{Command, Stdio, ChildStdout, ChildStderr};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, stdout, stderr};
use std::sync::{mpsc::Sender, Arc};
use std::thread;
use chrono::{DateTime, Duration, Utc};

use crate::process::{Channel, Log, Process, ProcessCommand};

pub fn run(
    command_ref: &String,
    process_sender: Sender<ProcessCommand>,
    log_sender: Sender<Log>,
    processes: Vec<Process>
) {
    let command = command_ref.clone();

    thread::spawn(move || {
        let mut child = Command::new("cmd")
            .args(&["/C", &command])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("The spawn messed up");
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| 
                Error::new(
                    ErrorKind::Other,
                    "Could not capture standard output."
                )
            ).expect("Error getting stdout.");
        let stderr = child
            .stderr
            .take()
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
            pid: child.id(),
            status: "Running".to_string(),
            cpu_usage: 3.02,
            memory_usage: 1000,
            user: "root".to_string(),
        };
        process_sender.send(ProcessCommand {
            command: "add".to_string(),
            process: process.clone()
        }).expect("Error sending process.");
        fs::create_dir_all(format!("/tmp/main/{}/", child.id()));
        let mut stdout_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("/tmp/main/{}/daemon.out", child.id()))
            .unwrap();
        let mut stderr_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("/tmp/main/{}/daemon.err", child.id()))
            .unwrap();
        let stdout_reader: BufReader<ChildStdout> = BufReader::new(stdout);
        stdout_reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                println!("{}", line);
                // let log = Log {
                //    process_id: id,
                //    content: line.clone(),
                //    error: false
                // };
                // log_sender.send(log).expect("Error sending log.");
                if let Err(e) = writeln!(stdout_file, "{}", line) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            });

        let stderr_reader: BufReader<ChildStderr> = BufReader::new(stderr);
        stderr_reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                println!("{}", line);
                // let log = Log {
                //    process_id: id,
                //    content: line.clone(),
                //    error: true
                // };
                // log_sender.send(log).expect("Error sending log.");
                if let Err(e) = writeln!(stderr_file, "{}", line) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            });
        process_sender.send(ProcessCommand {
            command: "remove".to_string(),
            process: process.clone()
        }).expect("Error terminating process.");
    });
}
