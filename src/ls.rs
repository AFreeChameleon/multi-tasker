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
use sysinfo::{System, Pid};

use crate::task::{Task, TaskManager};
use crate::command::{CommandData, CommandManager};

pub fn run() {
    let tasks: Vec<Task> = TaskManager::get_tasks();
    for task in tasks.iter() {
        let command = match CommandManager::read_command_data(task.id) {
            Ok(result) => result,
            Err(message) => return Err(message)
        };
        let sys = System::new_all();
        if let Some(process) = sys.process(Pid::from_u32(command.pid)) {
            // Get memory stats

        }
    }
}
