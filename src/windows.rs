#![cfg(target_os = "windows")]
use std::{
    ptr, ffi::{c_void, OsStr, OsString}, fs::File, io::{self, BufRead, BufReader, Write}, mem::{self, size_of}, os::{windows::{ffi::OsStrExt, io::{AsRawHandle, FromRawHandle, RawHandle}}}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{SystemTime, UNIX_EPOCH}
};
use home;
use windows::{core::{IntoParam, Error, HSTRING, PCWSTR, PWSTR}, Win32::{Foundation::{CloseHandle, SetHandleInformation, BOOL, HANDLE, HANDLE_FLAG_INHERIT}, Security::SECURITY_ATTRIBUTES, Storage::FileSystem::{ReadFile, WriteFile}, System::{Console::{GetStdHandle, STD_OUTPUT_HANDLE}, Pipes::CreatePipe, Services::{StartServiceCtrlDispatcherW, SERVICE_TABLE_ENTRYW}, Threading::{CreateProcessW, CreateThread, CREATE_NEW_CONSOLE, CREATE_NO_WINDOW, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOW}, IO::OVERLAPPED}}};

use crate::task::Files;
use crate::command::{CommandData, CommandManager};

// https://stackoverflow.com/questions/50384395/why-does-casting-from-a-reference-to-a-c-void-pointer-require-a-double-cast

pub fn generate_batch_file(task_id: u32, command: &String) -> Result<String, String> {
    let dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &task_id 
    );
    let dir_path = Path::new(&dir_str);
    let batch_file_path = dir_path.join("command.bat");
    let mut batch_file = File::create(&batch_file_path).unwrap();
    let batch_file_content = format!(
        "@echo off\n>{} 2>{} ({})",
        dir_path.join("stdout.out").display(),
        dir_path.join("stderr.err").display(),
        command
    );
    batch_file.write_all(batch_file_content.as_bytes()).unwrap();
    Ok(batch_file_path.display().to_string())
}

fn create_pipe() -> Result<(HANDLE, HANDLE), String> {
    let mut read_pipe: HANDLE = HANDLE::default();
    let mut write_pipe: HANDLE = HANDLE::default();
    let mut sec = SECURITY_ATTRIBUTES::default();
    sec.nLength = u32::try_from(size_of::<SECURITY_ATTRIBUTES>()).unwrap();
    sec.bInheritHandle = BOOL::from(true);
    println!("{:?}", sec);
    
    unsafe {
        CreatePipe(&mut read_pipe, &mut write_pipe, Some(&sec), 0).unwrap();
    }

    Ok((read_pipe, write_pipe))
}

// powershell "start test.bat -WindowStyle Hidden"
pub fn daemonize_task(task_id: u32, command: &String) {
    let dir_str = format!(
        "{}\\.multi-tasker\\processes\\{}",
        home::home_dir().unwrap().display(),
        &task_id 
    );

    let (out_read_pipe, out_write_pipe) = create_pipe().unwrap();
    unsafe {
        match SetHandleInformation(out_read_pipe.clone(), 0, HANDLE_FLAG_INHERIT) {
            Ok(_) => println!("Set handle for out pipe."),
            Err(msg) => println!("Can't set handle information.")
        };
    }
    let process_info = spawn_console_process(
        OsStr::new(command),
        out_write_pipe.clone()
    );
    println!("{:?}", process_info);

    let data = CommandData {
        command: command.to_string(),
        pid: process_info.dwProcessId
    };
    CommandManager::write_command_data(data, &Path::new(&dir_str));

    unsafe {
        CloseHandle(process_info.hThread).unwrap();
        CloseHandle(process_info.hProcess).unwrap();
        CloseHandle(out_write_pipe).unwrap();
    }

    let mut buffer: [u8; 4096] = [0; 4096];
    let mut overlapped = OVERLAPPED::default();

    unsafe {
        spawn_logger(out_read_pipe);
    }

    println!("Finsihed");
}

unsafe fn spawn_logger(out_read_pipe: HANDLE) {
    let mut buffer: [u8; 4096] = [0; 4096];
    let mut overlapped = OVERLAPPED::default();
    loop {
        let mut bytes_read = 0;
        match ReadFile(
            out_read_pipe,
            Some(&mut buffer),
            Some(&mut bytes_read),
            Some(&mut overlapped)
        ) {
            Ok(val) => {
                let f = File::create(Path::new("F:\\Dev\\Packages\\rust\\multi-tasker\\logs.log"));
                println!("{:?}", val)
            },
            Err(msg) => {
                println!("Error reading file {:?}", msg);
                break;
            }
        };
        if bytes_read == 0 {
            break;
        }
    }
    CloseHandle(out_read_pipe).unwrap();
}

// https://stackoverflow.com/questions/75767291/how-to-prevent-a-child-process-from-inheriting-the-standard-handles-in-rust-on-w
// https://stackoverflow.com/questions/42402673/createprocess-and-capture-stdout

pub fn spawn_console_process(
    command: &OsStr,
    out_write_pipe: HANDLE,
) -> PROCESS_INFORMATION {
    let mut startupinfo = STARTUPINFOW {
        cb: mem::size_of::<STARTUPINFOW>() as u32,
        hStdOutput: out_write_pipe.clone(),
        // hStdError: out_write_pipe.clone(),
        dwFlags: STARTF_USESTDHANDLES,
        ..Default::default()
    };

    let mut process_information = PROCESS_INFORMATION::default();
    let mut full_command = command.encode_wide().collect::<Vec<_>>();
    unsafe {
        CreateProcessW(
            PCWSTR::null(),
            PWSTR(full_command.as_mut_ptr()),
            None,
            None,
            BOOL::from(false),
            CREATE_NO_WINDOW,
            None,
            PCWSTR::null(),
            &mut startupinfo,
            &mut process_information,
        ).expect("Failed to create process.");
    }
    return process_information;
}
