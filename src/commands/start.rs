use std::env;

use crate::task::TaskManager;
use crate::command::CommandManager;

#[cfg(target_os = "linux")]
use crate::linux;

pub fn run() -> Result<(), String> {
    let tasks = TaskManager::get_tasks();
    for idx in 2..env::args().len() {
        let task_id: u32 = TaskManager::parse_arg(env::args().nth(idx)).unwrap();
        let task = TaskManager::get_task(&tasks, task_id).unwrap();
        let files = TaskManager::generate_task_files(task.id, &tasks);
        let command_data = match CommandManager::read_command_data(task.id) {
            Ok(data) => data,
            Err(message) => return Err(message)
        };
        println!("Running process with id {}...", env::args().nth(idx).unwrap());
        #[cfg(target_os = "linux")]
        linux::daemonize_task(files, command_data.command).unwrap();
        println!("FINISHED");

        // TYGAYKUGFUE
        // println!("test {}", env::args().len());
        // let (task, command_data, tasks) = match TaskManager::get_task_from_arg(idx) {
        //     Ok(val) => val,
        //     Err(msg) => return Err(msg)
        // };
        // let files = TaskManager::generate_task_files(task.id, &tasks);
        // println!("Running process...");
        // if cfg!(target_os = "linux") {
        //     #[cfg(target_os = "linux")]
        //     linux::daemonize_task(files, command_data.command);
        // } else {
        //     println!("Linux is only supported at the moment");
        // }
    }
    Ok(())
}
