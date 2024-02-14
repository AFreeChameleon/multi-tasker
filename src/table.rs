use ascii_table::AsciiTable;
use sysinfo::ProcessStatus;

pub struct MainHeaders {
    pub id: u32,
    pub command: String,
    pub pid: u32
}

pub struct ProcessHeaders {
    pub memory: u64,
    pub cpu: f32,
    pub runtime: u64,
    pub status: ProcessStatus
}

pub struct TableRow {
    pub id: u32,
    pub command: String,
    pub pid: u32,
    pub memory: u64,
    pub cpu: f32,
    pub runtime: u64,
    pub status: ProcessStatus
}

pub struct TableManager {
    pub ascii_table: AsciiTable,
    pub table_data: Vec<Vec<String>>
}

impl TableManager {
    pub fn create_headers(&mut self) {
        self.ascii_table.column(0).set_header("id");
        self.ascii_table.column(1).set_header("command");
        self.ascii_table.column(2).set_header("pid");
        self.ascii_table.column(3).set_header("memory");
        self.ascii_table.column(4).set_header("cpu");
        self.ascii_table.column(5).set_header("runtime");
        self.ascii_table.column(6).set_header("status");
    }

    pub fn insert_row(
        &mut self,
        headers: MainHeaders,
        process: Option<ProcessHeaders>
    ) {
        let mut row = vec![
            headers.id.to_string(),
            headers.command,
            headers.pid.to_string(),
        ];
        if let Some(p) = process {
            row.extend(vec![
                p.memory.to_string(),
                p.cpu.to_string(),
                p.runtime.to_string(),
                p.status.to_string()
            ]);
        } else {
            row.extend(vec![
                "N/A".to_string(),
                "N/A".to_string(),
                "N/A".to_string(),
                "Stopped".to_string()
            ]);
        }
        println!("{:?}", row);
        self.table_data.push(row);
    }

    pub fn print(&self) {
        println!("{:?}", &self.table_data);
        self.ascii_table.print(&self.table_data);
    }
}

