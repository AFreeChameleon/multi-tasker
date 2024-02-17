use std::{
    env::args,
};

use crate::task::{Task, TaskManager, Files};
use crate::command::{CommandData, CommandManager};
use crate::linux;

pub fn run() -> Result<(), String> {
    let command = match args().nth(2) {
        Some(val) => val,
        None => return Err("Missing command, usage: mult create \"[command]\"".to_string())
    };
    let mut new_task_id = 0;
    let mut tasks: Vec<Task> = TaskManager::get_tasks();
    if let Some(last_task) = tasks.last() {
        new_task_id = last_task.id + 1;
    }
    tasks.push(Task { id: new_task_id });
    let files = TaskManager::generate_task_files(new_task_id, tasks);
    println!("Running command...");
    if cfg!(target_os = "linux") {
        linux::daemonize_task(files, command);
    } else {
        println!("Linux is only supported at the moment");
    }
    Ok(())
}

