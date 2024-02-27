#![cfg(target_os = "windows")]
use std::{
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{SystemTime, UNIX_EPOCH},
    fs::File
};
use home;

use crate::task::Files;
use crate::command::{CommandData, CommandManager};

pub fn generate_batch_file(task_id: u32, command: &String) -> Result<String, String> {
    let dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &task_id 
    );
    let dir_path = Path::new(&dir_str);
    let batch_file_path = dir_path.join("command.bat");
    let mut batch_file = File::create(&batch_file_path).unwrap();
    let batch_file_content = format!(
        "@echo off\n>{} 2>{} ({})",
        dir_path.join("/stdout.out").display(),
        dir_path.join("/stderr.err").display(),
        command
    );
    batch_file.write_all(batch_file_content.as_bytes()).unwrap();
    Ok(batch_file_path.display().to_string())
}

// Using cmd /c start /b COMMAND > outfile.stdout 2> errfile.stderr
// powershell "start test.bat -WindowStyle Hidden"

pub fn daemonize_task(task_id: u32, command: &String, batch_file_path: String) {
    let dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &task_id 
    );
    let mut child = Command::new("cmd")
        .args(&[
              "/c",
              "START",
              "/B",
              &command,
              ">",
              &format!("{}/stdout.out", dir_str),
              "2>",
              &format!("{}/stderr.err", dir_str),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command: command.to_string(),
        pid: child.id()
    };
    CommandManager::write_command_data(data, &Path::new(&dir_str));

    let stdout = child.stdout.take().expect("");
    let stderr = child.stderr.take().expect("");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            println!("{:}|{}", now, line.expect("Problem reading stdout.")); 
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            eprintln!("{:}|{}", now, line.expect("Problem reading stderr.")); 
        }
    });
    child.wait().unwrap();
}
