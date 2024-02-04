use std::fs::OpenOptions;
use std::process::{Command, Stdio, ChildStdout};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::sync::{mpsc::Sender, Arc};
use std::thread;
use chrono::{DateTime, Duration, Utc};

use crate::process::{Process, Channel};

pub fn run(command_ref: &String, sender: Sender<Process>, processes: Vec<Process>) {
    let command = command_ref.clone();
    println!("{}", command);
    thread::spawn(move || {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("logs.log")
            .unwrap();
        println!("threaddd");
        let stdout = Command::new("cmd")
            .arg("/C")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().expect("Unable to start command.")
            .stdout
            .ok_or_else(|| Error::new(
                ErrorKind::Other, "Could not capture standard output."
             )).expect("An error occurred.");
        println!("RAHH");
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
        println!("{:?}", process);
        sender.send(process).expect("Error sending process.");
        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                println!("line: {}", line);        
            });
    });

}
