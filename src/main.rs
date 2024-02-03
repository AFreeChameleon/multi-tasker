use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start {
        list: bool
    }
}

fn main() {
    let args: Args = Args::parse();
    match &args.command {
        Some(Commands::Start { list }) => {
            if *list {
                println!("Print test");
            } else {
                println!("not printing");
            }
        },
        None => {}
    }
    println!("Hello, world!");
}
