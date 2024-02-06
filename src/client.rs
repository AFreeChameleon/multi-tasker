use std::{path::Path, fs::{File, OpenOptions}};

const STATUS_FILE: &str = "/tmp/multi-tasker/main/status.tmp";
pub fn check_server_exists() -> bool {
    if !Path::new(STATUS_FILE).exists() {
        return false;
    }
    let tmp_file = std::fs::read_to_string(STATUS_FILE)
        .expect("Error while opening status file.");
    let stats: Vec<&str> = tmp_file.split("\n").collect(); 
    return stats[1] == "Running";
}

pub fn send() {

}
