use mult_lib::error::MultErrorTuple;

const HELP_TEXT: &str = "usage: mult [options] [value]
options:
    create  Create a process and run it. [value] must be a command e.g \"ping google.com\"

    stop    Stops a process. [value] must be a task id e.g 0

    start   Starts a process. [value] must be a task id e.g 0

    restart Restarts a process. [value] must be a task id e.g 0

    ls      Shows all processes.

            --watch    Provides updating tables every 2 seconds.

    logs    Shows output from process. [value] must be a task id e.g 0

    delete  Deletes process. [value] must be a task id e.g 0

    help    Shows available options.
";

pub fn run() -> Result<(), MultErrorTuple> {
    println!("{HELP_TEXT}");
    Ok(())
}
