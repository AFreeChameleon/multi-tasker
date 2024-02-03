use std::env::{Args, args};

mod commands;
mod process;

fn main() {
    let process_manager = process::ProcessManager {
        processes: Vec::new()
    };
    commands::run(&process_manager);
    process::test_processes();
}
