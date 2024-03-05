use std::{env, thread, time::Duration};
use prettytable::{Cell, Row, Table};
use sysinfo::{System, Pid};

use crate::{managers::table::format_bytes, table::{MainHeaders, ProcessHeaders}, task::{Task, TaskManager}};
use crate::table::TableManager;
use crate::command::CommandManager;

pub fn run() -> Result<(), String> {
    let mut table = TableManager {
        ascii_table: Table::new(),
        table_data: Vec::new()
    };
    table.create_headers();
    
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
    if env::args().nth(2).unwrap() == "--listen" {
        listen(&mut table);
    } else {
        table.print();
    }
    
    Ok(())
}

fn listen(table: &mut TableManager) -> Result<(), String>{
    let mut height = table.print();
    let mut terminal = term::stdout().unwrap();
    loop {
        thread::sleep(Duration::from_secs(2));
        let sys = System::new_all();
        let tasks: Vec<Task> = TaskManager::get_tasks();
        for (idx, task) in tasks.iter().enumerate() {
            let mut row = table.ascii_table.get_mut_row(idx).unwrap();
            let command = match CommandManager::read_command_data(task.id) {
                Ok(result) => result,
                Err(message) => return Err(message)
            };
            if let Some(process) = sys.process(Pid::from_u32(command.pid)) {
                // 7 columns
                row.set_cell(Cell::new(
                        &format_bytes(process.virtual_memory() as f64)
                    ), 4).unwrap();
                row.set_cell(Cell::new(
                        &process.cpu_usage().to_string()
                    ), 5).unwrap();
                row.set_cell(Cell::new(
                        &process.run_time().to_string()
                    ), 6).unwrap();
            } else {
                row.set_cell(Cell::new("N/A"), 4).unwrap();
                row.set_cell(Cell::new("N/A"), 5).unwrap();
                row.set_cell(Cell::new("N/A"), 6).unwrap();
            }
        }
        for _ in 0..(height+3) {
            terminal.cursor_up().unwrap();
            terminal.delete_line().unwrap();
        }
        height = table.print();
    }
}

