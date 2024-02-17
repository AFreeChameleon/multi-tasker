use std::{
    env::args,
    fs
};

use crate::task::{Task, TaskManager, Files};
use crate::command::{CommandData, CommandManager};
use crate::linux;
use crate::stop::kill_process;

pub fn run() -> Result<(), String> {
    let (task, command_data, tasks) = match TaskManager::get_task_from_arg(2) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    match kill_process(command_data.pid) {
        Ok(_) => {},
        Err(msg) => println!("{msg}")
    };
    let new_tasks = tasks.into_iter().filter(|t| t.id != task.id);
    let process_dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        task.id
    );
    match fs::remove_dir_all(process_dir_str) {
        Ok(()) => {},
        Err(msg) => return Err(format!("{:?}", msg))
    };
    TaskManager::write_tasks_file(new_tasks.collect());
    println!("Process deleted.");
    Ok(())
}
