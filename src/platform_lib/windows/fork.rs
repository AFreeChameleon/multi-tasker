#![cfg(target_os = "windows")]

use std::process::Command;

use mult_lib::task::Files;

pub fn run_daemon(files: Files, command: String) -> Result<(), String> {
    Command::new("cmd")
        .args(&[
              "/c",
              "F:\\Dev\\Packages\\rust\\multi-tasker\\target\\debug\\mult_spawn.exe",
              &files.process_dir.display().to_string(),
              &command
        ])
        .spawn()
        .expect("Spawning process has failed.");
    Ok(())
}

