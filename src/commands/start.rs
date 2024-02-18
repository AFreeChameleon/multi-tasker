use crate::task::TaskManager;
use crate::linux;

pub fn run() -> Result<(), String> {
    let (task, command_data, tasks) = match TaskManager::get_task_from_arg(2) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };
    let files = TaskManager::generate_task_files(task.id, tasks);
    println!("Running process...");
    if cfg!(target_os = "linux") {
        linux::daemonize_task(files, command_data.command);
    } else {
        println!("Linux is only supported at the moment");
    }
    Ok(())
}
