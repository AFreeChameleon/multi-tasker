use std::env::args;

use mult_lib::{error::{MultError, MultErrorTuple}, task::{Task, TaskManager}};

#[cfg(target_os = "linux")]
use crate::platform_lib::linux::fork;
#[cfg(target_os = "windows")]
use crate::platform_lib::windows::fork;

pub fn run() -> Result<(), MultErrorTuple> {
    let command = match args().nth(2) {
        Some(val) => val,
        None => return Err((MultError::MissingCommand, None))
    };
    let mut new_task_id = 0;
    let mut tasks: Vec<Task> = TaskManager::get_tasks()?;
    if let Some(last_task) = tasks.last() {
        new_task_id = last_task.id + 1;
    }
    tasks.push(Task { id: new_task_id });
    println!("Running command...");
    let files = TaskManager::generate_task_files(new_task_id, &tasks);
    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        match fork::run_daemon(files, command) {
            Ok(()) => (),
            Err(msg) => return Err(msg)
        };
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        fork::run_daemon(files, command)?;
    } else {
        println!("Linux is only supported at the moment");
    }
    println!("Process created.");
    Ok(())
}

