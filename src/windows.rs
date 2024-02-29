#![cfg(target_os = "windows")]
use std::{
    ptr, ffi::{c_void, OsStr, OsString}, fs::File, io::{self, BufRead, BufReader, Write}, mem::{self, size_of}, os::{windows::{ffi::OsStrExt, io::{AsRawHandle, FromRawHandle, RawHandle}}}, path::{Path, PathBuf}, process::{Command, Stdio}, thread, time::{SystemTime, UNIX_EPOCH}
};
use home;
use windows::{core::{IntoParam, Error, HSTRING, PCWSTR, PWSTR}, Win32::{Foundation::{CloseHandle, SetHandleInformation, BOOL, HANDLE, HANDLE_FLAG_INHERIT}, Security::SECURITY_ATTRIBUTES, Storage::FileSystem::ReadFile, System::{Pipes::CreatePipe, Threading::{CreateProcessW, CREATE_NEW_CONSOLE, CREATE_NO_WINDOW, PROCESS_INFORMATION, STARTUPINFOW}, IO::OVERLAPPED}}};

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

fn create_pipe() -> Result<(*mut HANDLE, *mut HANDLE), String> {
    let read_pipe: *mut HANDLE =  ptr::from_mut(&mut HANDLE::default());
    let write_pipe: *mut HANDLE = ptr::from_mut(&mut HANDLE::default());
    let mut sec = SECURITY_ATTRIBUTES::default();
    sec.nLength = u32::try_from(size_of::<SECURITY_ATTRIBUTES>()).unwrap();
    sec.lpSecurityDescriptor = ptr::null_mut();
    sec.bInheritHandle = true.into();

    unsafe {
        CreatePipe(read_pipe, write_pipe, Some(&mut sec), 0).unwrap();
        SetHandleInformation(read_pipe.as_ref(), 0, HANDLE_FLAG_INHERIT).unwrap();
    }

    Ok((read_pipe, write_pipe))
}

// powershell "start test.bat -WindowStyle Hidden"

pub fn daemonize_task(task_id: u32, command: &String) {
    let dir_str = format!(
        "{}/.multi-tasker/processes/{}",
        home::home_dir().unwrap().display(),
        &task_id 
    );

    let (read_pipe, write_pipe) = create_pipe().unwrap();
    let process_info = spawn_console_process(OsStr::new(command));
    println!("{:?}", process_info);

    let data = CommandData {
        command: command.to_string(),
        pid: process_info.dwProcessId
    };
    CommandManager::write_command_data(data, &Path::new(&dir_str));

    unsafe {
        CloseHandle(process_info.hThread).unwrap();
        CloseHandle(process_info.hProcess).unwrap();
        CloseHandle(*write_pipe).unwrap();
    }

    let mut buffer: [u8; 4096] = [0; 4096];
    let mut overlapped = OVERLAPPED::default();
    loop {
        let mut bytes_read = 0;
        unsafe {
            match ReadFile(
                *read_pipe,
                Some(&mut buffer),
                Some(&mut bytes_read),
                Some(&mut overlapped)
            ) {
                Ok(val) => val,
                Err(_) => println!("Unable to read file.")
            };
        };
        if bytes_read == 0 {
            break;
        }
        println!("{:?}", buffer);
    }
    
    unsafe {
        CloseHandle(*read_pipe).unwrap();
    }

    //std::option::Option::<*mut u32>::from(
    //    <*mut u32 as Into<*mut u32>>::into(buffer.len())
    //),

    // thread::spawn(move || {
    //     let reader = BufReader::new(stdout);

    //     for line in reader.lines() {
    //         let now = SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_millis();
    //         println!("{:}|{}", now, line.expect("Problem reading stdout.")); 
    //     }
    // });

    // thread::spawn(move || {
    //     let reader = BufReader::new(stderr);

    //     for line in reader.lines() {
    //         let now = SystemTime::now()
    //             .duration_since(UNIX_EPOCH)
    //             .unwrap()
    //             .as_millis();
    //         eprintln!("{:}|{}", now, line.expect("Problem reading stderr.")); 
    //     }
    // });
}

// https://stackoverflow.com/questions/75767291/how-to-prevent-a-child-process-from-inheriting-the-standard-handles-in-rust-on-w

pub fn spawn_console_process(
    command: &OsStr
) -> PROCESS_INFORMATION {
    let mut startupinfo = STARTUPINFOW {
        cb: mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };

    let mut process_information = PROCESS_INFORMATION::default();
    let mut full_command = command.encode_wide().collect::<Vec<_>>();
    unsafe {
        CreateProcessW(
            PCWSTR::null(),
            PWSTR(full_command.as_mut_ptr()),
            Some(ptr::null_mut()),
            Some(ptr::null_mut()),
            BOOL::from(false),
            CREATE_NO_WINDOW,
            Some(ptr::null_mut()),
            PCWSTR::null(),
            ptr::addr_of_mut!(startupinfo),
            ptr::addr_of_mut!(process_information),
        ).expect("Failed to create process.");
    }
    println!("GRRRR");
    return process_information;
}
