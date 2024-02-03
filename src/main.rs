use std::env::args;

mod start;
mod process;

struct Args {
    mode: String
}

struct Process {
    id: u8,
    command: String,
    started_at: String
}

fn main() {
    let mode = args().nth(1).expect("No mode given.");
    let args = Args {
        mode
    };
    match args.mode.as_str() {
        "start" => println!("Started"),
        _ => println!("Nuffin here")
    };
    process::test_processes();
}
