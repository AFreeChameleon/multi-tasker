#![cfg(target_os = "windows")]

use std::{env, path::Path, process::Command};

use mult_lib::task::Files;

pub fn run_daemon(files: Files, command: String) -> Result<(), String> {
    if let Ok(exe_dir) = env::current_exe() {
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
    } else {
        return Err("Could not get directory of executable.".to_string());
    }
    Ok(())
}

