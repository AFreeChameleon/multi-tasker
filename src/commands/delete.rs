use std::{env, fs};

use mult_lib::args::parse_args;
use mult_lib::command::CommandManager;
use mult_lib::error::{print_success, MultError, MultErrorTuple};
use mult_lib::task::TaskManager;
use crate::stop::kill_process;

pub fn run() -> Result<(), MultErrorTuple> {
    let args = env::args();
    let parsed_args = parse_args(&args.collect::<Vec<String>>()[1..], &[], true)?;
    let tasks = TaskManager::get_tasks()?;
    let mut new_tasks = tasks.clone();
    for arg in parsed_args.values.iter() {
        let task_id: u32 = TaskManager::parse_arg(Some(arg.to_string()))?;
        let task = TaskManager::get_task(&tasks, task_id)?;
        let command_data = CommandManager::read_command_data(task.id)?;
        match kill_process(command_data.pid) {
            Ok(_) => (),
            Err(_) => { println!("Process {} is not running.", task_id) }
        };
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
        print_success(&format!("Process {} deleted.", task_id));
    }
    TaskManager::write_tasks_file(&new_tasks);
    println!("Processes saved.");
    Ok(())
}
