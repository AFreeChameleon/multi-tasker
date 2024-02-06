use std::fs::{File, OpenOptions};

pub fn check_server_exists() {
    let tmp_file = std::fs::read_to_string("/tmp/multi-tasker/main/status.tmp");
    match tmp_file {
        Ok(content) => {
             let stats: Vec<&str> = content.split("\n").collect(); 
             return stats[1] == "Running";
        },
        Error(error) => {
           println!("Problem opening status file: {:?}", error);
           return false;
        }
    }
}

pub fn send() {

}
