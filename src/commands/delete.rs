use std::{env, fs};

use mult_lib::command::CommandManager;
use mult_lib::error::{MultError, MultErrorTuple};
use mult_lib::task::TaskManager;
use crate::stop::kill_process;

pub fn run() -> Result<(), MultErrorTuple> {
    let tasks = TaskManager::get_tasks()?;
    let mut new_tasks = tasks.clone();
    let task_id: u32 = TaskManager::parse_arg(env::args().nth(2))?;
    let task = TaskManager::get_task(&tasks, task_id)?;
    let command_data = CommandManager::read_command_data(task.id)?;
    kill_process(command_data.pid)?;
    new_tasks = new_tasks.into_iter().filter(|t| t.id != task_id).collect();
    let process_dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        task_id
    );
    match fs::remove_dir_all(process_dir_str) {
        Ok(()) => {},
        Err(_) => return Err((MultError::ProcessDirNotExist, None))
    };
    TaskManager::write_tasks_file(&new_tasks);
    println!("Process deleted.");
    Ok(())
}
