use std::{env, thread, time::Duration};
use mult_lib::error::{MultError, MultErrorTuple};
use prettytable::Table;
use sysinfo::{System, Pid};

use mult_lib::table::{MainHeaders, ProcessHeaders, TableManager};
use mult_lib::task::{Task, TaskManager};
use mult_lib::command::CommandManager;

pub fn run() -> Result<(), MultErrorTuple> {
    let mut table = TableManager {
        ascii_table: Table::new(),
        table_data: Vec::new()
    };
    table.create_headers();
    setup_table(&mut table)?;
    if let Some(new_arg) = env::args().nth(2) {
        if new_arg == "--listen" {
            if cfg!(target_os = "windows") {
                return Err((MultError::WindowsNotSupported, Some("--listen".to_string())));
            }
            listen()?;
        } else {
            return Err((MultError::InvalidArgument, Some(new_arg)))
        }
    } else {
        table.print();
    }

    Ok(())
}

fn listen() -> Result<(), MultErrorTuple> {
    let mut table = TableManager {
        ascii_table: Table::new(),
        table_data: Vec::new()
    };
    table.create_headers();
    setup_table(&mut table)?;
    let mut height = table.print();
    let mut terminal = term::stdout().unwrap();
    loop {
        thread::sleep(Duration::from_secs(2));
        table = TableManager {
            ascii_table: Table::new(),
            table_data: Vec::new()
        };
        table.create_headers();
        setup_table(&mut table)?;
        for _ in 0..height {
            terminal.cursor_up().unwrap();
            terminal.delete_line().unwrap();
        }
        height = table.print();
    }
}

pub fn setup_table(table: &mut TableManager) -> Result<(), MultErrorTuple> {
    let tasks: Vec<Task> = TaskManager::get_tasks()?;
    for task in tasks.iter() {
        let command = match CommandManager::read_command_data(task.id) {
            Ok(result) => result,
            Err(err) => return Err(err)
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

