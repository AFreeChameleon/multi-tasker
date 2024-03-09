#![cfg(target_os = "windows")]

use std::{env, path::Path, process::Command};

use mult_lib::task::Files;

pub fn run_daemon(files: Files, command: String) -> Result<(), String> {
    let exe_dir = env::current_exe().unwrap();
    let spawn_dir = Path::new(&exe_dir);
    Command::new("cmd")
        .args(&[
              "/c",
              &spawn_dir.join("mult_spawn.exe").display().to_string(),
              &files.process_dir.display().to_string(),
              &command
        ])
        .spawn()
        .expect("Spawning process has failed.");
    Ok(())
}

