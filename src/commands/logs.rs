use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, Event};
use std::fs::{self, File};
use std::path::Path;
use std::io::{Read, Seek, SeekFrom, BufReader, BufRead};
use std::sync::mpsc::{self, Sender};
use std::time::Duration;
use rev_lines::RevLines;

use crate::task::TaskManager;

pub fn run() -> Result<(), String> {
    let (task, command_data, tasks) = match TaskManager::get_task_from_arg(2) {
        Ok(val) => val,
        Err(msg) => return Err(msg)
    };

    let file_path = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &task.id 
    );
    let out_file_path = format!("{}/stdout.out", &file_path);
    let err_file_path = format!("{}/stderr.err", &file_path);

    let (tx, rx) = mpsc::channel();

    let mut out_file = File::open(&out_file_path).unwrap();
    let mut out_pos = fs::metadata(&out_file_path).unwrap().len();

    let mut err_file = File::open(&err_file_path).unwrap();
    let mut err_pos = fs::metadata(&err_file_path).unwrap().len();

    let mut test_file = File::open("/home/bean/test.txt").unwrap();
    let rev_lines = RevLines::new(test_file).take(15);
    for line in rev_lines {
        println!("{:?}", line);
    }

    let mut out_watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(_event) => {
                if out_file.metadata().unwrap().len() != out_pos {
                    out_file.seek(SeekFrom::Start(out_pos + 1)).unwrap();
                    out_pos = out_file.metadata().unwrap().len();
                    let reader = BufReader::new(&out_file);
                    for line in reader.lines() {
                        tx.send(line).unwrap();
                    }
                }

                if err_file.metadata().unwrap().len() != err_pos {
                    err_file.seek(SeekFrom::Start(err_pos + 1)).unwrap();
                    err_pos = err_file.metadata().unwrap().len();
                    let reader = BufReader::new(&err_file);
                    for line in reader.lines() {
                        tx.send(line).unwrap();
                    }
                }
            }
            Err(error) => println!("File watch error {error:?}")
        }
    }).unwrap();

    out_watcher.watch(Path::new(&file_path), RecursiveMode::Recursive).unwrap();
    
    for res in rx {
        match res {
            // Ok(line) => println!("{line}"),
            Ok(line) => {},
            Err(error) => println!("Reciever error {error:?}")
        }
    }

    Ok(())
}

