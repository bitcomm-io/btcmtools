use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::process;
// use nix::sys::{kill, Signal};
// use nix::unistd::Pid;
use slog::info;

use crate::LOGGER;

static PID_FILE_NAME: &str = "bitcomm.pid";


pub fn save_pid() {
    // 获取当前进程的 PID
    let pid = process::id();

    // 将 PID 写入文件
    let mut file = File::create(PID_FILE_NAME).expect("Failed to create file");
    write!(file, "{}", pid).expect("Failed to write to file");

    // println!("PID written to pid.txt");
}


pub fn read_pid() -> i32 {
    if let Ok(metadata) = fs::metadata(PID_FILE_NAME) {
        if metadata.is_file() {
            if let Ok(mut file) = File::open(PID_FILE_NAME) {
                let mut pid_str = String::new();
                if let Ok(_) = file.read_to_string(&mut pid_str) {
                    if let Ok(pid) = pid_str.trim().parse::<i32>() {
                        return pid;
                    }
                }
            }
        }
    }
    -1
}

pub fn dele_pid() {
    let rs = fs::remove_file(PID_FILE_NAME);
    match rs {
        Ok(_) =>{}
        Err(_) =>{}
    }
}

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

pub fn kill_pid(pid:i32) {
    if pid != -1 {
        // 发送 SIGTERM 信号给指定的进程
        match kill(Pid::from_raw(pid), Signal::SIGTERM) {
            Ok(_) => info!(LOGGER,"Successfully sent SIGTERM signal to PID {}", pid),
            Err(err) => info!(LOGGER,"Failed to send SIGTERM signal to PID {}: {}", pid, err),
        }
    }
}