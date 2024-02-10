use std::{
    process::{Command, Stdio, ChildStdout, ChildStderr},
    io::Write,
    sync::{Mutex, mpsc, Arc},
    fs::{self, File, OpenOptions},
    thread,
    time::Duration,
    env::args,
    path::Path
};
use home;
use serde::Serialize;
use daemonize::Daemonize;
use bincode;
use glob;

use crate::task::{Task, TaskManager};

#[derive(serde::Serialize)]
struct Data {
    command: String,
    pid: u32
}

pub fn start() {
    let mut new_task_id = 0;
    let mut tasks: Vec<Task> = TaskManager::get_tasks();
    if let Some(last_task) = tasks.last() {
        new_task_id = last_task.id + 1;
    }
    tasks.push(Task { id: new_task_id });
    let dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &new_task_id 
    );
    let process_dir = Path::new(&dir_str);
    fs::create_dir_all(
        process_dir
    ).unwrap();
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");

    let stdout = File::create(process_dir.join("stdout.out")).unwrap();
    let stderr = File::create(process_dir.join("stderr.err")).unwrap();
    let daemonize = Daemonize::new()
        .umask(0o112)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    println!("Running command...");
    TaskManager::write_tasks_file(tasks);
    match daemonize.start() {
        Ok(_) => run_command(&command, &process_dir),
        Err(e) => eprintln!("Error, {}", e)
    };
}

fn run_command(command: &str, process_dir: &Path) {
    let mut child = Command::new("sh")
        .args(&["-c", &command])
        .spawn()
        .expect("Command has failed.");

    let data = Data {
        command: command.to_string(),
        pid: child.id()
    };
    let encoded_data: Vec<u8> = bincode::serialize::<Data>(&data).unwrap();
    let mut process_file = File::create(process_dir.join("data.bin")).unwrap();
    process_file.write_all(&encoded_data).unwrap();
}

