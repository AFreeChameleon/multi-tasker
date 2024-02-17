#![cfg(target_os = "linux")]
use daemonize::Daemonize;
use std::{
    process::Command,
    path::Path
};

use crate::manager::task::{Files, TaskManager};
use crate::manager::command::{CommandData, CommandManager};

pub fn daemonize_task(files: Files, command: String) {
    let daemonize = Daemonize::new()
        .umask(0o112)
        .stdout(files.stdout)
        .stderr(files.stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => run_command(&command, &files.process_dir),
        Err(e) => eprintln!("Error, {}", e)
    };
}

fn run_command(command: &str, process_dir: &Path) {
    let mut child = Command::new("sh")
        .args(&["-c", &command])
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command: command.to_string(),
        pid: child.id()
    };
    CommandManager::write_command_data(data, process_dir);
}
