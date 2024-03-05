use std::{env, thread, time::Duration};
use prettytable::Table;
use sysinfo::{System, Pid};

use crate::table::{MainHeaders, ProcessHeaders, TableManager};
use crate::task::{Task, TaskManager};
use crate::command::CommandManager;

pub fn run() -> Result<(), String> {
    let mut table = TableManager {
        ascii_table: Table::new(),
        table_data: Vec::new()
    };
    table.create_headers();
    setup_table(&mut table).unwrap();
    
    match env::args().nth(2) {
        Some(val) => {
            if val == "--listen" {
                listen().unwrap();
                return Ok(());
            }
            println!("Invalid argument.");
        },
        None => {
            table.print();
        }
    };

    Ok(())
}

fn listen() -> Result<(), String>{
    let mut table = TableManager {
        ascii_table: Table::new(),
        table_data: Vec::new()
    };
    table.create_headers();
    setup_table(&mut table).unwrap();
    let mut height = table.print();
    let mut terminal = term::stdout().unwrap();
    loop {
        thread::sleep(Duration::from_secs(2));
        table = TableManager {
            ascii_table: Table::new(),
            table_data: Vec::new()
        };
        table.create_headers();
        setup_table(&mut table).unwrap();
        for _ in 0..height {
            terminal.cursor_up().unwrap();
            terminal.delete_line().unwrap();
        }
        height = table.print();
    }
}

pub fn setup_table(table: &mut TableManager) -> Result<(), String> {
    let tasks: Vec<Task> = TaskManager::get_tasks();
    for task in tasks.iter() {
        let command = match CommandManager::read_command_data(task.id) {
            Ok(result) => result,
            Err(message) => return Err(message)
        };
        let sys = System::new_all();

        let main_headers = MainHeaders {
            id: task.id,
            command: command.command,
        };
        if let Some(process) = sys.process(Pid::from_u32(command.pid)) {
            // Get memory stats
            let process_headers = ProcessHeaders {
                pid: command.pid,
                memory: process.virtual_memory(),
                cpu: process.cpu_usage(),
                runtime: process.run_time(),
                status: "Running".to_string()
            };

            table.insert_row(main_headers, Some(process_headers));
        } else {
            table.insert_row(main_headers, None);
        }
    }
    Ok(())
}

