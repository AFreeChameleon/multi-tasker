use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub struct Process {
    pub id: u8,
    pub command: String,
    pub started_at: DateTime<Utc>,
    pub pid: u32,
    pub status: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub user: String
}

#[derive(Debug, Clone)]
pub struct Log {
    pub process_id: u8,
    pub content: String
}

pub struct Channel<T> {
    pub sender: Sender<T>,
    pub receiver: Arc<Mutex<Receiver<T>>>,
}

pub struct ProcessManager {
    pub process_channel: Channel<Process>,
    pub processes: Arc<Vec<Process>>,
}

pub struct LogManager {
    pub log_channel: Channel<Log>,
    pub logs: Arc<Vec<Log>>
}

impl LogManager {
    pub fn log_add(&mut self, log: Log) {
        let mut logs = Arc::clone(&self.logs).to_vec();
        logs.push(log);
        self.logs = Arc::new(logs);
    }

    pub fn log_listen(mut self) {
        let rc_receiver = Arc::clone(&self.log_channel.receiver);
        thread::spawn(move || {
            loop {
                let log = rc_receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .expect("Couldn't receive log.");
                println!("Log created: {:?}", log);
                self.log_add(log);
            }
        });
    }
}

impl ProcessManager {
    pub fn process_add(&mut self, process: Process) {
        let mut processes = Arc::clone(&self.processes).to_vec();
        processes.push(process);
        self.processes = Arc::new(processes);
    }

    pub fn process_listen(&mut self) {
        let rc_receiver = Arc::clone(&self.process_channel.receiver);
        loop {
            let process = rc_receiver
                .lock()
                .unwrap()
                .recv()
                .expect("Couldn't receive message.");
            println!("Process created: {:?}", process);
            self.process_add(process);
        }
    }
}



// pub fn test_processes() {
//     let mut process_manager = ProcessManager {
//         processes: Vec::new()
//     };
//     let new_process = Process {
//         id: 0,
//         command: "npm start".to_string(),
//         started_at: Utc::now(),
//         pid: 3000,
//         status: "Running".to_string(),
//         cpu_usage: 3.02,
//         memory_usage: 1000,
//         user: "root".to_string()
//     };
//     process_manager.add(new_process);
//     println!("{:?}", process_manager.processes);
// }
