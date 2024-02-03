use std::fs::OpenOptions;
use std::process::{Command, Stdio, ChildStdout};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::thread;

use crate::process::ProcessManager;

pub fn run(command_ref: &String, &manager: &ProcessManager) {
    let command = command_ref.clone();
    thread::spawn(move || {
        let mut log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("logs.log")
            .unwrap();
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
        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line: Result<String, Error>| line.ok())
            .for_each(|line: String| {
                
            });
    });

}
