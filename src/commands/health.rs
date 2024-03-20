use std::fs;
use std::path::Path;

use home::home_dir;
use mult_lib::args::parse_args;
use mult_lib::error::{print_error, print_success, MultError, MultErrorTuple};
use mult_lib::task::TaskManager;

const FIX_FLAG: &str = "--fix";
const FLAGS: [(&str, bool); 1] = [
    (FIX_FLAG, false)
];

pub fn run() -> Result<(), MultErrorTuple> {
    #[cfg(target_family = "windows")]
    colored::control::set_virtual_terminal(true).unwrap();
    let parsed_args = parse_args(&FLAGS, false)?;
    let mut fix_enabled = false;
    if parsed_args.flags.contains(&FIX_FLAG.to_string()) {
        println!("Fix flag enabled.");
        fix_enabled = true;
    }
    println!("Running health check...");
    match run_tests(fix_enabled) {
        Ok(()) => println!("No failures detected."),
        Err(Some((err, descriptor))) => print_error(err, descriptor),
        Err(_) => ()
    };
    println!("Health check finished.");
    Ok(())
}

fn run_tests(fix_enabled: bool) -> Result<(), Option<MultErrorTuple>> {
    // Initial checks
    let tasks_dir_str = format!("{}/.multi-tasker/", home::home_dir().unwrap().display());
    let tasks_dir = Path::new(&tasks_dir_str);
    if !tasks_dir.exists() && tasks_dir.is_dir() {
        if !fix_enabled {
            return Err(Some((MultError::MainDirNotExist, None)))
        }
        create_main_dir()?;
    }
    print_success("Main directory exists.");
    // Check tasks file exists
    let processes_dir = tasks_dir.join("processes");
    let mut processes = match check_processes_dir(&processes_dir) {
        Ok(val) => val,
        Err(msg) => {
            if !fix_enabled {
                return Err(Some(msg));
            }
            create_process_dir()?;
            Vec::new()
        }
    };
    // CARRY ON MAKING FIX FLAG
    print_success("Processes directory exists.");
    // Checking for processes while no task file exists
    if !tasks_dir.join("tasks.bin").exists() {
        for name in processes {
            print_error(MultError::UnknownProcessInDir, Some(name.to_string()));
        }
        return Err(None)
    }
    print_success("Tasks file exists.");
    let tasks = match TaskManager::get_tasks() {
        Ok(val) => val,
        Err(msg) => {
            return Err(Some(msg));
        }
    };
    print_success("Tasks file read.");
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
    print_success("Task logs read.");
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

fn create_main_dir() -> Result<(), MultErrorTuple> {
    let home_dir_string = home_dir().unwrap();
    let home = Path::new(&home_dir_string);
    let main_dir = home.join(".multi-tasker/");
    fs::create_dir(main_dir).unwrap();
    print_success("Created main dir.");
    Ok(())
}

fn create_process_dir() -> Result<(), MultErrorTuple> {
    let home_dir_string = home_dir().unwrap();
    let home = Path::new(&home_dir_string);
    let main_dir = home.join(".multi-tasker/processes/");
    fs::create_dir(main_dir).unwrap();
    print_success("Created processes dir.");
    Ok(())
}
