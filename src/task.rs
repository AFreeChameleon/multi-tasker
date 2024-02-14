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
        return Vec::new();
    }

    pub fn write_tasks_file(new_tasks: Vec<Task>) {
        let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
        let tasks_file_dir = Path::new(&tasks_dir_str).join("tasks.bin");
        let mut tasks_file = File::create(tasks_file_dir).unwrap();
        let encoded_data: Vec<u8> = bincode::serialize::<Vec<Task>>(&new_tasks).unwrap();
        tasks_file.write_all(&encoded_data).unwrap();
    }
}

pub struct Process {
    pub memory_usage: u64,
    pub status: ProcessStatus,
    pub cpu_usage: f32,
    pub runtime: u64
}

