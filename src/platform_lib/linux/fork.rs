use std::{
    ffi::CString, fs::File, io::{BufRead, BufReader, Write}, path::Path, process::{Command, Stdio}, thread, time::{SystemTime, UNIX_EPOCH}
};

use libc::c_char;
use crate::managers::task::Files;
use crate::managers::command::{CommandManager, CommandData};

pub fn run_daemon(files: Files, command: String) -> Result<(), String> {
    let process_id;
    let sid;
    unsafe {
        process_id = libc::fork();
    }
    // Fork failed
    if process_id < 0 {
        println!("Fork failed");
        return Err("Fork failed".to_string())
    }
    // Parent process - need to kill it
    if process_id > 0 {
        println!("Process id of child process {}", process_id);
        return Ok(())
    }
    unsafe {
        libc::umask(0);
        sid = libc::setsid();
    }
    if sid < 0 {
        return Err("Setting sid failed".to_string())
    }
    unsafe {
        let c_str = CString::new("/").unwrap();
        libc::chdir(c_str.as_ptr() as *const c_char);
        libc::close(libc::STDIN_FILENO);
        libc::close(libc::STDOUT_FILENO);
        libc::close(libc::STDERR_FILENO);
    }
    // Do daemon stuff here
    run_command(&command, &files.process_dir);
    Ok(())
}

fn run_command(command: &str, process_dir: &Path) {
    let mut child = Command::new("sh")
        .args(&["-c", &command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command: command.to_string(),
        pid: child.id()
    };
    CommandManager::write_command_data(data, process_dir);

    let stdout = child.stdout.take().expect("");
    let stderr = child.stderr.take().expect("");
    
    let mut stdout_file = File::create(process_dir.join("stdout.out"))
        .expect("Could not open stdout file.");
    let mut stderr_file = File::create(process_dir.join("stderr.err"))
        .expect("Could not open stderr file.");

    thread::spawn(move || {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let formatted_line = format!(
                "{:}|{}",
                now,
                line.expect("Problem reading stdout.")
            ); 
            stdout_file.write_all(formatted_line.as_bytes())
                .expect("Problem writing to stdout.");
        }
    });

    thread::spawn(move || {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let formatted_line = format!(
                "{:}|{}",
                now,
                line.expect("Problem reading stderr.")
            ); 
            stderr_file.write_all(formatted_line.as_bytes())
                .expect("Problem writing to stderr.");
        }
    });
    child.wait().unwrap();
}
