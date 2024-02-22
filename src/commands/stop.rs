use sysinfo::{Pid, System};

use crate::task::TaskManager;

pub fn run() -> Result<(), String> {
    let (_, command_data, _) = match TaskManager::get_task_from_arg(2) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    match kill_process(command_data.pid) {
        Ok(_) => {},
        Err(msg) => return Err(msg)
    };
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
