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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CommandData {
    pub command: String,
    pub pid: u32
}

pub struct CommandManager {}

impl CommandManager {
    pub fn read_command_data(task_id: u32) -> Result<CommandData, String> {
        let dir_str = format!("{}/.multi-tasker/processes/{}", home::home_dir().unwrap().display(), task_id);
        let data_file = Path::new(&dir_str).join("data.bin");
        println!("{}", dir_str);
        if data_file.exists() {
            let data_encoded: Vec<u8> = fs::read(data_file).unwrap(); 
            let data_decoded: CommandData = bincode::deserialize(&data_encoded[..]).unwrap();
            return Ok(data_decoded)
        }
        return Err("No task exists with that id.".to_string())
    }

    pub fn write_command_data(command: CommandData, process_dir: &Path) {
        let encoded_data: Vec<u8> = bincode::serialize::<CommandData>(&command).unwrap();
        let mut process_file = File::create(process_dir.join("data.bin")).unwrap();
        process_file.write_all(&encoded_data).unwrap();
    }
}
