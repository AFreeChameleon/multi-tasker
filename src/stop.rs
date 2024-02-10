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
use sysinfo::{Pid, System};
use home;
use serde::Serialize;
use daemonize::Daemonize;
use bincode;
use glob;

use crate::task::{Task, TaskManager};
use crate::command::{CommandData, CommandManager};

pub fn run() -> Result<(), String> {
    let task_id: u32 = match args().nth(2) {
        Some(val) => val.parse::<u32>().unwrap(),
        None => return Err("Missing task id, usage: mult stop \"[task id]\"".to_string())
    };
    let mut tasks: Vec<Task> = TaskManager::get_tasks();
    let task = tasks.iter().find(|&t: &&Task| t.id == task_id).unwrap();
    let command = match CommandManager::read_command_data(task.id) {
        Ok(data) => data,
        Err(message) => return Err(message)
    };
    let s = System::new_all();
    if let Some(process) = s.process(Pid::from_u32(command.pid)) {
        process.kill();
    } else {
        return Err("Process is not running.".to_string())
    }
    Ok(())
}
