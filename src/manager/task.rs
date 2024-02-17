use std::{
    process::{Command, Stdio, ChildStdout, ChildStderr},
    io::Write,
    sync::{Mutex, mpsc, Arc},
    fs::{self, File, OpenOptions},
    thread,
    time::Duration,
    env::args,
    path::Path
};

use bincode;
use sysinfo::{ProcessStatus};

use crate::manager::command::{CommandData, CommandManager};
use crate::linux;

pub struct Files {
    pub process_dir: Box<Path>,
    pub stdout: File,
    pub stderr: File
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u32
}

pub struct TaskManager {}

impl TaskManager {
    pub fn get_tasks() -> Vec<Task> {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_file = Path::new(&tasks_dir_str).join("tasks.bin");
        if tasks_file.exists() {
            let tasks_encoded: Vec<u8> = fs::read(tasks_file).unwrap(); 
            let tasks_decoded: Vec<Task> = bincode::deserialize(&tasks_encoded[..]).unwrap();
            return tasks_decoded;
        }
        Vec::new()
    }

    pub fn write_tasks_file(new_tasks: Vec<Task>) {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_file_dir = Path::new(&tasks_dir_str).join("tasks.bin");
        let mut tasks_file = File::create(tasks_file_dir).unwrap();
        let encoded_data: Vec<u8> = bincode::serialize::<Vec<Task>>(&new_tasks).unwrap();
        tasks_file.write_all(&encoded_data).unwrap();
    }

    pub fn get_task_from_arg(nth_arg: usize) -> Result<(Task, CommandData, Vec<Task>), String> {
        let mut tasks: Vec<Task> = TaskManager::get_tasks();
        let task_id: u32 = match args().nth(nth_arg) {
            Some(arg) => match arg.parse::<u32>() {
                Ok(id) => id,
                Err(_) => return Err("Invalid id, usage: mult start \"[command]\"".to_string())
            },
            None => return Err("Missing/invalid id, usage: mult start \"[command]\"".to_string())
        };

        let task: Task = match tasks.iter().find(|&t| t.id == task_id).cloned() {
            Some(t) => t,
            None => return Err("No task exists with that id, use mult ls to see the available tasks.".to_string())
        };
        let command_data = match CommandManager::read_command_data(task.id) {
            Ok(data) => data,
            Err(message) => return Err(message)
        };

        Ok((task, command_data, tasks))
    }

    pub fn generate_task_files(task_id: u32, tasks: Vec<Task>) -> Files {
        let dir_str = format!(
            "{}/.multi-tasker/processes/{}",
            home::home_dir().unwrap().display(),
            &task_id 
        );
        let process_dir = Path::new(&dir_str);
        fs::create_dir_all(
            process_dir
        ).unwrap();

        let stdout = File::create(process_dir.join("stdout.out")).unwrap();
        let stderr = File::create(process_dir.join("stderr.err")).unwrap();

        TaskManager::write_tasks_file(tasks);

        Files {
            process_dir: process_dir.into(),
            stdout,
            stderr
        }
    }
}

