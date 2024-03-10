use std::env;

use mult_lib::task::{TaskManager, Files};
use mult_lib::command::{CommandData, CommandManager};

#[cfg(target_os = "linux")]
use crate::platform_lib::linux::fork;

#[cfg(target_os = "windows")]
use crate::platform_lib::windows::fork;

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    let task_id: u32 = TaskManager::parse_arg(env::args().nth(2)).unwrap();
    let task = TaskManager::get_task(&tasks, task_id).unwrap();
    let files = TaskManager::generate_task_files(task.id, &tasks);
    let command_data = match CommandManager::read_command_data(task.id) {
        Ok(data) => data,
        Err(message) => return Err(message)
    };
    println!("Running process with id {}...", env::args().nth(2).unwrap());
    match start_process(files, command_data) {
        Ok(()) => return Ok(()),
        Err(msg) => return Err(msg)
    }
}

pub fn start_process(files: Files, command_data: CommandData) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        match fork::run_daemon(files, command_data.command) {
            Ok(()) => (),
            Err(msg) => return Err(msg)
        };
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        match fork::run_daemon(files, command_data.command) {
            Ok(()) => (),
            Err(msg) => return Err(msg)
        };
    } else {
        println!("Windows & linux is only supported at the moment");
    }
    Ok(())
}
