use std::env;

use mult_lib::error::{print_success, MultError, MultErrorTuple};
use mult_lib::task::{TaskManager, Files};
use mult_lib::command::{CommandData, CommandManager};
use sysinfo::{Pid, System};

#[cfg(target_os = "linux")]
use crate::platform_lib::linux::fork;

#[cfg(target_os = "windows")]
use crate::platform_lib::windows::fork;

pub fn run() -> Result<(), MultErrorTuple> {
    let tasks = TaskManager::get_tasks()?;
    let task_id: u32 = TaskManager::parse_arg(env::args().nth(2))?;
    let task = TaskManager::get_task(&tasks, task_id)?;
    let files = TaskManager::generate_task_files(task.id, &tasks);
    let command_data = CommandManager::read_command_data(task.id)?;
    let sys = System::new_all();
    if let Some(_) = sys.process(Pid::from_u32(command_data.pid)) {
        return Err((MultError::ProcessAlreadyRunning, None))
    }
    let current_dir = env::current_dir().unwrap();
    env::set_current_dir(&command_data.dir).unwrap();
    start_process(files, command_data)?;
    env::set_current_dir(&current_dir).unwrap();
    print_success("Process started.");
    Ok(())
}

pub fn start_process(files: Files, command_data: CommandData) -> Result<(), MultErrorTuple> {
    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        match fork::run_daemon(files, command_data.command) {
            Ok(()) => (),
            Err(msg) => return Err(msg)
        };
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        fork::run_daemon(files, command_data.command)?;
    } else {
        println!("Windows & linux is only supported at the moment");
    }
    Ok(())
}
