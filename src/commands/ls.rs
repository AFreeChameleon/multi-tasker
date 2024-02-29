use prettytable::Table;
use sysinfo::{System, Pid};

use crate::{table::{MainHeaders, ProcessHeaders}, task::{Task, TaskManager}};
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

        // test
        for (pid, process) in sys.processes() {
            println!("{} {}", pid, process.name());
        }

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
    table.print();
    
    Ok(())
}
