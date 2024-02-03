use std::process::{Command, Stdio, ChildStdout};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::thread;

pub fn start(command_ref: &String) {
    let command = command_ref.clone();
    thread::spawn(move || {
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
