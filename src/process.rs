use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Process {
    id: u8,
    command: String,
    started_at: DateTime<Utc>,
    pid: u32,
    status: String,
    cpu_usage: f32,
    memory_usage: u64,
    user: String
}

pub struct ProcessManager {
    pub processes: Vec<Process>
}

impl ProcessManager {
    fn add(&mut self, process: Process) {
        self.processes.push(process);
    }
}

pub fn test_processes() {
    let mut process_manager = ProcessManager {
        processes: Vec::new()
    };
    let new_process = Process {
        id: 0,
        command: "npm start".to_string(),
        started_at: Utc::now(),
        pid: 3000,
        status: "Running".to_string(),
        cpu_usage: 3.02,
        memory_usage: 1000,
        user: "root".to_string()
    };
    process_manager.add(new_process);
    println!("{:?}", process_manager.processes);
}
