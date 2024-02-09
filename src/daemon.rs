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
use daemonize::Daemonize;
use bincode;

struct Process {
    command: String,
    pid: i32
}

pub fn start() {
    let process_dir = Path::new(&format!("~/.multi-tasker/processes/{}", std::process::id()));
    fs::create_dir_all(
        process_dir
    ).unwrap();
    let mode = args().nth(1).expect("No mode given.");
    let command = args().nth(2).expect("No command given.");
    let process = Process {
        command,
        pid: std::process::id()
    };
    let encoded_data: Vec<u8> = bincode::serialize(&process).unwrap();
    let mut process_file = File::create(process_dir.join("data.bin")).unwrap();
    process_file.write_all(&encoded_data).unwrap();

    let stdout = File::create(process_dir.join("stdout.out")).unwrap();
    let stderr = File::create(process_dir.join("stderr.err")).unwrap();
    let daemonize = Daemonize::new()
        .umask(0o112)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => run_command(&command),
        Err(e) => eprintln!("Error, {}", e)
    };
}

fn run_command(command: &str) {
    let mut child = Command::new("cmd")
        .args(&["/C", &command])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Command has failed.");
}
