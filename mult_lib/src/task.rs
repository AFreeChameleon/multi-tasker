use std::{
    io::Write,
    fs::{self, File},
    env::args,
    path::Path
};

use bincode;
use home;
use colored::Colorize;

use crate::command::{CommandData, CommandManager};

const PROCESS_FILES: [&str; 3] = ["stdout.out", "stderr.err", "data.bin"];

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
    pub fn test_task_files(id: u32) -> Result<(), String> {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_dir = Path::new(&tasks_dir_str).join(id.to_string());
        if tasks_dir.exists() {
            return Err(format!("Could not get task directory {}.", id.to_string()))
        }
        for file in PROCESS_FILES.iter() {
            if tasks_dir.join(file).exists() {
                println!("Could not get {}", file.red());
            }
        }
        Ok(())
    }
    
    pub fn get_tasks() -> Result<Vec<Task>, String> {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_file = Path::new(&tasks_dir_str).join("tasks.bin");
        if tasks_file.exists() {
            let tasks_encoded: Vec<u8> = fs::read(tasks_file).unwrap(); 
            let tasks_decoded: Vec<Task> = match bincode::deserialize(&tasks_encoded[..]) {
                Ok(val) => val,
                Err(_) => return Err("Failed to read from tasks file.".to_string())
            };
            return Ok(tasks_decoded);
        }
        Ok(Vec::new())
    }

    pub fn get_task(tasks: &Vec<Task>, id: u32) -> Result<Task, String> {
        let task: Task = match tasks.iter().find(|&t| t.id == id).cloned() {
            Some(t) => t,
            None => return Err("No task exists with that id, use mult ls to see the available tasks.".to_string())
        };
        Ok(task)
    }

    pub fn write_tasks_file(new_tasks: &Vec<Task>) {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_file_dir = Path::new(&tasks_dir_str).join("tasks.bin");
        let mut tasks_file = File::create(tasks_file_dir).unwrap();
        let encoded_data: Vec<u8> = bincode::serialize::<Vec<Task>>(&new_tasks).unwrap();
        tasks_file.write_all(&encoded_data).unwrap();
    }

    pub fn get_task_from_arg(nth_arg: usize) -> Result<(Task, CommandData, Vec<Task>), String> {
        let tasks: Vec<Task> = TaskManager::get_tasks()?;
        let task_id: u32 = match args().nth(nth_arg) {
            Some(arg) => match arg.parse::<u32>() {
                Ok(id) => id,
                Err(_) => return Err("Invalid id, see 'mult help' for more.".to_string())
            },
            None => return Err("Missing/invalid id, see 'mult help' for more.".to_string())
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

    pub fn generate_task_files(task_id: u32, tasks: &Vec<Task>) -> Files {
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

    pub fn parse_arg(arg: Option<String>) -> Result<u32, String> {
        let task_id: u32 = match arg {
            Some(arg) => match arg.parse::<u32>() {
                Ok(id) => id,
                Err(_) => return Err("Invalid id, see 'mult help' for more.".to_string())
            },
            None => return Err("Missing/invalid id, see 'mult help' for more.".to_string())
        };
        Ok(task_id)
    }
}

