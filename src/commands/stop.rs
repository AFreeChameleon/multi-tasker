use std::env;
use mult_lib::error::{MultError, MultErrorTuple};
use sysinfo::{Pid, System};

use mult_lib::task::TaskManager;
use mult_lib::command::CommandManager;

pub fn run() -> Result<(), MultErrorTuple> {
    let tasks = TaskManager::get_tasks()?;
    let task_id: u32 = TaskManager::parse_arg(env::args().nth(2))?;
    let task = TaskManager::get_task(&tasks, task_id)?;
    let command_data = CommandManager::read_command_data(task.id)?;
    kill_process(command_data.pid)?;
    println!("Process stopped.");
    Ok(())
}

pub fn kill_process(pid: u32) -> Result<(), MultErrorTuple> {
    let sys = System::new_all();
    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        process.kill();
    } else {
        return Err((MultError::ProcessNotRunning, None))
    }
    Ok(())
}
