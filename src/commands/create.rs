use std::env::args;

use crate::{task::{Task, TaskManager}, windows};

#[cfg(target_os = "linux")]
use crate::linux;

pub fn run() -> Result<(), String> {
    let command = match args().nth(2) {
        Some(val) => val,
        None => return Err("Missing command, see 'mult help' for more.".to_string())
    };
    let mut new_task_id = 0;
    let mut tasks: Vec<Task> = TaskManager::get_tasks();
    if let Some(last_task) = tasks.last() {
        new_task_id = last_task.id + 1;
    }
    tasks.push(Task { id: new_task_id });
    println!("Running command...");
    if cfg!(target_os = "linux") {
        let files = TaskManager::generate_task_files(new_task_id, tasks);
        #[cfg(target_os = "linux")]
        linux::daemonize_task(files, &command);
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        // let batch_file_path = windows::generate_batch_file(new_task_id, &command).unwrap();
        windows::daemonize_task(new_task_id, &command);
    } else {
        println!("Linux is only supported at the moment");
    }
    Ok(())
}

