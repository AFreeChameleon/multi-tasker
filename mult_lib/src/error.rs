use colored::Colorize;

pub type MultErrorTuple = (MultError, Option<String>);

pub enum MultError {
    MainDirNotExist,
    ProcessDirNotExist,
    FailedReadingProcessDir,
    TaskDirNotExist,
    TaskFileNotExist,
    TaskBinFileUnreadable,
    TaskNotFound,
    InvalidTaskId,
    UnknownProcessInDir,
    FailedFormattingProcessEntry,
    FailedConvertingProcessEntry
}

pub fn print_error(error: MultError, descriptor: Option<String>) {
    let message = match error {
        MultError::MainDirNotExist => "Main directory doesn't exist.".to_string(),
        MultError::ProcessDirNotExist => "Process directory doesn't exist.".to_string(),
        MultError::FailedReadingProcessDir => "Failed reading processes directory.".to_string(),
        // TODO: Add params in like the task id
        MultError::TaskDirNotExist => format!("Could not get task directory {}.", descriptor.unwrap()),
        MultError::TaskFileNotExist => format!("Could not get task file {}.", descriptor.unwrap()),
        MultError::TaskBinFileUnreadable => "Failed to read from tasks file.".to_string(),
        MultError::TaskNotFound => format!("No task exists with id: {}, use mult ls to see the available tasks.", descriptor.unwrap()),
        MultError::InvalidTaskId => "Invalid id, see 'mult help' for more.".to_string(),
        MultError::UnknownProcessInDir => format!("Unknown process in dir: {}", descriptor.unwrap()),
        MultError::FailedFormattingProcessEntry => "Failed formatting entry.".to_string(),
        MultError::FailedConvertingProcessEntry => "Failed converting file name from processes directory.".to_string(),
    };
    println!("{} {}", "Error:".red(), message);
}
