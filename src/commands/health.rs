use std::fs;
use std::path::Path;
use colored::Colorize;

use mult_lib::error::{MultError, MultErrorTuple, print_error};
use mult_lib::task::TaskManager;

pub fn run() -> Result<(), MultErrorTuple> {
    #[cfg(target_os = "windows")]
    colored::control::set_virtual_terminal(true).unwrap();
    println!("Running health check...");
    match run_tests() {
        Ok(()) => println!("No failures detected."),
        Err(Some((err, descriptor))) => print_error(err, descriptor),
        Err(_) => ()
    };
    println!("Health check finished.");
    Ok(())
}

fn run_tests() -> Result<(), Option<MultErrorTuple>> {
    // Initial checks
    let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
    let tasks_dir = Path::new(&tasks_dir_str);
    if !tasks_dir.exists() && tasks_dir.is_dir() {
        return Err(Some((MultError::MainDirNotExist, None)))
    }
    // Check tasks file exists
    let processes_dir = tasks_dir.join("processes");
    let mut processes = match check_processes_dir(&processes_dir) {
        Ok(val) => val,
        Err(msg) => return Err(Some(msg))
    };
    // Checking for processes while no task file exists
    if !tasks_dir.join("tasks.bin").exists() {
        for name in processes {
            print_error(MultError::UnknownProcessInDir, Some(name.red().to_string()));
        }
        return Err(None)
    }
    let tasks = match TaskManager::get_tasks() {
        Ok(val) => val,
        Err(msg) => {
            return Err(Some(msg));
        }
    };
    // Check process dir, log files & data binary
    for task in tasks.iter() {
        if processes.contains(&task.id.to_string()) {
            processes = processes
                .into_iter()
                .filter(|process: &String| process != &task.id.to_string())
                .collect();
        }
        match TaskManager::test_task_files(task.id) {
            Ok(()) => (),
            Err((err, desc)) => { print_error(err, desc); } 
        };
    }
    for process in processes {
        print_error(MultError::UnknownProcessInDir, Some(process));
    }
    Ok(())
}

fn check_processes_dir(processes_dir: &Path) -> Result<Vec<String>, MultErrorTuple> {
    let mut name_entries = Vec::new();
    if processes_dir.exists() && processes_dir.is_dir() {
        let entries = match fs::read_dir(processes_dir) {
            Ok(val) => val,
            Err(_) => return Err((MultError::FailedReadingProcessDir, None))
        };
        for entry in entries {
            let entry = match entry {
                Ok(val) => val,
                Err(_) => return Err((MultError::FailedFormattingProcessEntry, None))
            };
            let Ok(file_name) = entry.file_name().into_string() else {
                print_error(MultError::FailedConvertingProcessEntry, None);
                continue;
            };
            name_entries.push(file_name);
        }
    } else {
        return Err((MultError::ProcessDirNotExist, None))
    }
    Ok(name_entries)
}

