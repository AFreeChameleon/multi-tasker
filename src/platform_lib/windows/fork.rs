#![cfg(target_os = "windows")]

use std::{
    os::windows::process::CommandExt,
    fs::File,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    thread,
    time::{SystemTime, UNIX_EPOCH}
};

use windows::Win32::System::Console::FreeConsole;

use crate::managers::task::Files;
use crate::managers::command::{CommandManager, CommandData};

pub fn run_daemon(files: Files, command: String) -> Result<(), String> {
    let process_dir = &files.process_dir;
    let output_file = File::create("out.log").unwrap();
    let stdout = Stdio::from(output_file.try_clone().unwrap());
    let stderr = Stdio::from(output_file.try_clone().unwrap());
    let mut child = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(&["/c", &command])
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command,
        pid: child.id()
    };
    CommandManager::write_command_data(data, &files.process_dir);

    let stdout = child.stdout.take().expect("Failed to take stdout.");
    let stderr = child.stderr.take().expect("Failed to take stderr.");
    
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
    Ok(())
}

