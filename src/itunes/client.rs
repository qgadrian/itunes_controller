use std::process::Command;
use std::{thread, time};

pub fn execute(action: &str) {
    ensure_is_running();

    let action_to_execute = format!("tell application \"iTunes\" to {}", action);

    Command::new("osascript")
        .arg("-e")
        .arg(action_to_execute)
        .stdout(std::process::Stdio::inherit())
        .output()
        .expect("failed to execute process");
}

fn ensure_is_running() {
    Command::new("osascript")
        .arg("-e")
        .arg("tell application \"iTunes\" to launch")
        .output()
        .expect("failed to start iTunes");

    while !is_running() {
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
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
