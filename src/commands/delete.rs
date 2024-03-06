use std::{env, fs};

use crate::managers::command::CommandManager;
use crate::task::{TaskManager, Task};
use crate::stop::kill_process;

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    let mut new_tasks = tasks.clone();
    for idx in 2..env::args().len() {
        let task_id: u32 = TaskManager::parse_arg(env::args().nth(idx)).unwrap();
        let command_data = match CommandManager::read_command_data(task_id) {
            Ok(data) => data,
            Err(message) => return Err(message)
        };
        match kill_process(command_data.pid) {
            Ok(_) => {},
            Err(msg) => println!("{msg}")
        };
        new_tasks = new_tasks.into_iter().filter(|t| t.id != task_id).collect();
        let process_dir_str = format!(
            "{}/.multi-tasker/processes/{}",
            home::home_dir().unwrap().display(),
            task_id
        );
        match fs::remove_dir_all(process_dir_str) {
            Ok(()) => {},
            Err(msg) => return Err(format!("{:?}", msg))
        };
    }
    TaskManager::write_tasks_file(&new_tasks);
    Ok(())
}
