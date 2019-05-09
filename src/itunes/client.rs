use std::process::Command;
use std::{thread, time};

pub fn new() -> Command {
    let mut command = Command::new("osascript");

    command
        .arg("-e")
        .arg("tell application \"iTunes\" to launch");

    while !is_running() {
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }

    command
}

pub fn execute(command: &mut Command, action: &str) {
    let execute = format!("tell application \"iTunes\" to {}", action);

    command
        .arg("-e")
        .arg(execute)
        .stdout(std::process::Stdio::inherit())
        .output()
        .expect("failed to execute process");
}

fn is_running() -> bool {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to (name of processes) contains \"iTunes\"")
        .output()
        .expect("failed to execute process");

    let is_running = String::from_utf8_lossy(&output.stdout);

    // output string seems to be trimmed since it contains a new line character at the end
    return is_running.trim() == "true";
}
