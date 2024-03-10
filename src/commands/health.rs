use mult_lib::task::{TaskManager, Files};
use mult_lib::command::{CommandData, CommandManager};

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    Ok(())
}

