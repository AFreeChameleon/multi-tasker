use std::io::Error;
use std::fs;
use std::path::Path;
use colored::{control, Colorize};

use mult_lib::task::{TaskManager, Files};
use mult_lib::command::{CommandData, CommandManager};

pub fn run() -> Result<(), String> {
    control::set_virtual_terminal(true).unwrap();
    println!("Running tests...");
    // Initial checks
    let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
    let tasks_dir = Path::new(&tasks_dir_str);
    if !tasks_dir.exists() {
        println!("Main directory doesn't exist.");
        return Ok(())
    }
    // Check tasks file exists
    let processes_dir = tasks_dir.join("processes");
    let Ok(processes) = check_processes_dir(&processes_dir) else {
        println!("Failed reading process dir.");
        return Ok(())
    };
    // Checking for processes while no task file exists
    if !tasks_dir.join("tasks.bin").exists() {
        for name in processes {
            println!("{} is not a task, found in processes.", name.red());
        }
        return Ok(())
    }
    let tasks = TaskManager::get_tasks()?;
    // Check process dir, log files & data binary
    for task in tasks.iter() {
        match TaskManager::test_task_files(task.id) {
            Ok(()) => (),
            Err(msg) => println!("{}", msg.red())
        };
    }
    println!("Tests finished.");
    Ok(())
}

fn check_processes_dir(processes_dir: &Path) -> Result<Vec<String>, Error> {
    let mut name_entries = Vec::new();
    if processes_dir.exists() && processes_dir.is_dir() {
        let entries = fs::read_dir(processes_dir)?;
        for entry in entries {
            let entry = entry?;
            let Ok(file_name) = entry.file_name().into_string() else {
                println!("Failed converting file name from processes directory.");
                continue;
            };
            name_entries.push(file_name);
        }
    } else {
        println!("Process directory doesn't exist.");
    }
    return Ok(name_entries)
}

