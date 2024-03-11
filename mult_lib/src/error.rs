use colored::Colorize;

pub type MultErrorTuple = (MultError, Option<String>);

pub enum MultError {
    MainDirNotExist,
    ProcessNotRunning,
    ProcessDirNotExist,
    UnknownProcessInDir,
    FailedReadingProcessDir,
    TaskDirNotExist,
    TaskFileNotExist,
    TaskBinFileUnreadable,
    TaskNotFound,
    InvalidTaskId,
    FailedFormattingProcessEntry,
    FailedConvertingProcessEntry,
    MissingCommand,
    ExeDirNotFound,
    WindowsNotSupported,
    InvalidArgument,
    CannotReadOutputFile
}

pub fn print_error(error: MultError, descriptor: Option<String>) {
    let message = match error {
        MultError::MainDirNotExist => "Main directory doesn't exist.".to_string(),
        MultError::ProcessDirNotExist => "Process directory doesn't exist.".to_string(),
        MultError::FailedReadingProcessDir => "Failed reading processes directory.".to_string(),
        MultError::TaskDirNotExist => format!("Could not get task directory {}.", descriptor.unwrap()),
        MultError::TaskFileNotExist => format!("Could not get task file {}.", descriptor.unwrap()),
        MultError::TaskBinFileUnreadable => "Failed to read from tasks file.".to_string(),
        MultError::TaskNotFound => "No task exists with that id, use mult ls to see the available tasks.".to_string(),
        MultError::InvalidTaskId => "Invalid id, see 'mult help' for more.".to_string(),
        MultError::UnknownProcessInDir => format!("Unknown process in dir: {}", descriptor.unwrap()),
        MultError::FailedFormattingProcessEntry => "Failed formatting entry.".to_string(),
        MultError::FailedConvertingProcessEntry => "Failed converting file name from processes directory.".to_string(),
        MultError::MissingCommand => "Missing command, see 'mult help' for more.".to_string(),
        MultError::ExeDirNotFound => "Could not get directory of executable.".to_string(),
        MultError::ProcessNotRunning => "Process is not running.".to_string(),
        MultError::WindowsNotSupported => format!("Windows does not support {}.", descriptor.unwrap()),
        MultError::InvalidArgument => format!("Invalid argument {}.", descriptor.unwrap()),
        MultError::CannotReadOutputFile => "Error while reading output file.".to_string(),
    };
    println!("{} {}", "Error:".red(), message);
}
