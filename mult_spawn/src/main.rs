#![windows_subsystem = "windows"]

use std::{
    env::args, fs::File, io::{BufRead, BufReader, Write}, os::windows::process::CommandExt, path::Path, process::{Command, Stdio}, thread, time::{SystemTime, UNIX_EPOCH}
};

use mult_lib::command::{CommandManager, CommandData};

// Usage: mult_spawn process_dir command
fn main() -> Result<(), String> {
    let dir_string = args().nth(1).unwrap();
    let process_dir = Path::new(&dir_string);
    let command = args().nth(2).unwrap();
    let mut child = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(&["/c", &command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Command has failed.");

    let data = CommandData {
        command,
        pid: child.id()
    };
    CommandManager::write_command_data(data, process_dir);

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
                "{:}|{}\n",
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
                "{:}|{}\n",
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
