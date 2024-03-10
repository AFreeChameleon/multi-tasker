use std::env;

use mult_lib::task::TaskManager;
use mult_lib::command::CommandManager;

use crate::stop::kill_process;

use super::start::start_process;

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    let task_id: u32 = TaskManager::parse_arg(env::args().nth(2))?;
    let task = TaskManager::get_task(&tasks, task_id)?;
    let command_data = CommandManager::read_command_data(task.id)?;
    println!("Killing process...");
    kill_process(command_data.pid)?;
    let files = TaskManager::generate_task_files(task.id, &tasks);
    println!("Restarting process...");
    match start_process(files, command_data) {
        Ok(_) => {},
        Err(msg) => return Err(msg)
    };
    Ok(())
}

