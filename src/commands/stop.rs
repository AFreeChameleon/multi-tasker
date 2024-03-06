use std::env;
use sysinfo::{Pid, System};

use crate::task::TaskManager;
use crate::command::CommandManager;

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    for idx in 2..env::args().len() {
        let task_id: u32 = TaskManager::parse_arg(env::args().nth(idx)).unwrap();
        let command_data = match CommandManager::read_command_data(task_id) {
            Ok(data) => data,
            Err(message) => return Err(message)
        };
        match kill_process(command_data.pid) {
            Ok(_) => {},
            Err(msg) => return Err(msg)
        };
    }
    Ok(())
}

pub fn kill_process(pid: u32) -> Result<(), String> {
    let sys = System::new_all();
    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        process.kill();
    } else {
        return Err("Process is not running.".to_string())
    }
    Ok(())
}
