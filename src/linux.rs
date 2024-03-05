#![cfg(target_os = "linux")]
use std::{
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use fork::{self, Fork};

use crate::task::Files;
use crate::command::{CommandData, CommandManager};

pub fn daemonize_task(files: Files, command: String) {
    if let Ok(Fork::Child) = fork::daemon(false, false) {
       run_command(&command, &files.process_dir); 
    }
}

fn run_command(command: &str, process_dir: &Path) {
    let mut child = Command::new("sh")
        .args(&["-c", &command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command: command.to_string(),
        pid: child.id()
    };
    CommandManager::write_command_data(data, process_dir);

    let stdout = child.stdout.take().expect("");
    let stderr = child.stderr.take().expect("");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            println!("{:}|{}", now, line.expect("Problem reading stdout.")); 
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            eprintln!("{:}|{}", now, line.expect("Problem reading stderr.")); 
        }
    });
    child.wait().unwrap();
}
