pub fn run() -> Result<(), String> {
    println!("usage: mult [options] [value]
    options:
        create  Create a process and run it. [value] must be a command e.g \"ping google.com\"
        stop    Stops a process. [value] must be a task id e.g 0
        start   Starts a process. [value] must be a task id e.g 0
        ls      Shows all processes.
        logs    Shows output from process. [value] must be a task id e.g 0
        delete  Deletes process. [value] must be a task id e.g 0
        help    Shows available options.
    ");
    Ok(())
}