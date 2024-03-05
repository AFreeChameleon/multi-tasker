use std::{io::stdout, thread::{self, sleep}, time::Duration};

use prettytable::{Table, Cell, Row, format};
use sysinfo::Process;

pub struct MainHeaders {
    pub id: u32,
    pub command: String,
}

pub struct ProcessHeaders {
    pub pid: u32,
    pub memory: u64,
    pub cpu: f32,
    pub runtime: u64,
    pub status: String
}

pub struct TableManager {
    pub ascii_table: Table,
    pub table_data: Vec<Vec<String>>,
}

impl TableManager {
    pub fn create_headers(&mut self) {
        self.ascii_table.set_format(
            format::FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top],
                    format::LineSeparator::new('─', '┬', '┌', '┐'),
                )
                .separators(
                    &[format::LinePosition::Title],
                    format::LineSeparator::new('─', '┼', '├', '┤'),
                )
                .separators(
                    &[format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '┴', '└', '┘'),
                )
                .padding(1, 1)
                .build(),
        );
        self.ascii_table.set_titles(Row::new(vec![
            Cell::new("id").style_spec("b"),
            Cell::new("command").style_spec("b"),
            Cell::new("pid").style_spec("b"),
            Cell::new("status").style_spec("b"),
            Cell::new("memory").style_spec("b"),
            Cell::new("cpu").style_spec("b"),
            Cell::new("runtime").style_spec("b"),
        ]));
    }

    pub fn insert_row(
        &mut self,
        headers: MainHeaders,
        process: Option<ProcessHeaders>
    ) {
        let mut row: Vec<Cell> = vec![
            Cell::new(&headers.id.to_string()),
            Cell::new(&headers.command),
        ];
        if let Some(p) = process {
            row.extend(vec![
                Cell::new(&p.pid.to_string()),
                Cell::new(&p.status.to_string()).style_spec("Fgb"),
                Cell::new(&format_bytes(p.memory as f64)),
                Cell::new(&p.cpu.to_string()),
                Cell::new(&p.runtime.to_string())
            ]);
        } else {
            row.extend(vec![
                Cell::new("N/A"),
                Cell::new("Stopped").style_spec("Frb"),
                Cell::new("N/A"),
                Cell::new("N/A"),
                Cell::new("N/A")
            ]);
        }

        self.ascii_table.add_row(Row::new(row));
    }

    pub fn print(&mut self) -> usize {
        self.ascii_table.print_tty(false).unwrap()
    }

    pub fn print_watch(&mut self) {
        let mut height = self.ascii_table.print_tty(false).unwrap();
        let mut terminal = term::stdout().unwrap();
        loop {
            thread::sleep(Duration::from_secs(2));
            for _ in 0..height {
                terminal.cursor_up().unwrap();
                terminal.delete_line().unwrap();
            }
            height = self.ascii_table.print_tty(false).unwrap();
        }
    }

    pub fn refresh_rows(&self, row: &mut Row, process: &Process) {
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
    }
}

const SUFFIX: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
const UNIT: f64 = 1000.0;
pub fn format_bytes(bytes: f64) -> String {
    if bytes <= 0.0 {
        return "0 B".to_string();
    }
    let base = bytes.log10() / UNIT.log10();

    let result = format!("{:.1}", UNIT.powf(base - base.floor()),)
        .trim_end_matches(".0")
        .to_owned();

    [&result, SUFFIX[base.floor() as usize]].join(" ")
}

