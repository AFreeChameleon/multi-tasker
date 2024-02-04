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

pub struct Channel {
    pub sender: Sender<Process>,
    pub receiver: Arc<Mutex<Receiver<Process>>>,
}

pub struct ProcessManager {
    pub process_channel: Channel,
    pub processes: Arc<Vec<Process>>
}

impl ProcessManager {
    pub fn add(&mut self, process: Process) {
        let mut processes = Arc::clone(&self.processes).to_vec();
        processes.push(process);
        self.processes = Arc::new(processes);
    }

    pub fn process_listen(mut self) {
        let rc_receiver = Arc::clone(&self.process_channel.receiver);
        thread::spawn(move || {
            loop {
                let process = rc_receiver
                    .lock()
                    .unwrap()
                    .recv()
                    .expect("Couldn't receive message.");
                println!("Process created: {:?}", process);
                self.add(process);
            }
        });
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
