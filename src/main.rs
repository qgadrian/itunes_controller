use std::process::Command;
use std::{thread, time};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = &args[1];
    let mut itunes = itunes();

    if command == "open" {
        itunes
            .output()
            .expect("failed to execute command");
    }
    else if command == "play" {
        if args.len() > 3 {
            let kind = &args[2];

            if kind == "playlist" {
                let target = &args[3];
                let action: &str = &&format!("play playlist \"{}\"", target);
                execute(&mut itunes, action)
            } else if kind == "track" || kind == "song" {
                let target = &args[3];
                let action: &str = &&format!("play track \"{}\"", target);
                execute(&mut itunes, action)
            }
        } else {
            execute(&mut itunes, command)
        }
    }
    else if command == "pause" {
        execute(&mut itunes, command)
    }
    else if command == "stop" {
        execute(&mut itunes, command)
    }
    else if command == "next" {
        execute(&mut itunes, "play next track")
    }
    else if command == "previous" {
        execute(&mut itunes, "play previous track")
    }
    else if command == "next" {
        execute(&mut itunes,"play next track")
    }
    else if command == "previous" {
        execute(&mut itunes, "play previous track")
    }
    else if command == "love" {
        if args.len() > 2 {
            let target = &args[2];
            if target == "clear" {
                execute(&mut itunes, "set loved of current track to false")
            }
        } else {
            execute(&mut itunes, "set loved of current track to true")
        }
    }
    else if command == "dislike" {
        if args.len() > 2 {
            let target = &args[2];
            if target == "clear" {
                execute(&mut itunes, "set disliked of current track to false")
            }
        } else {
            execute(&mut itunes, "set disliked of current track to true")
        }

    }
    else if command == "playlists" {
        let action = "get name of playlists";
        let execute = format!("tell application \"iTunes\" to {}", action);

        let output = itunes
            .arg("-e")
            .arg(execute)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout);

        let playlists: Vec<&str> =
            output
            .split(",")
            .collect();

        for playlist in &playlists {
            println!("{}", playlist);
        }
    }
    else {
        println!("Unknown action: {}", command)
    }
}

fn is_running() -> bool {
    let output =
        Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to (name of processes) contains \"iTunes\"")
        .output()
        .expect("failed to execute process");

    let is_running = String::from_utf8_lossy(&output.stdout);

    // output string seems to be trimmed since it contains a new line character at the end
    return is_running.trim() == "true";
}

fn itunes() -> Command {
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

fn execute(command: &mut Command, action: &str) {
    let execute = format!("tell application \"iTunes\" to {}", action);

    command
        .arg("-e")
        .arg(execute)
        .stdout(std::process::Stdio::inherit())
        .output()
        .expect("failed to execute process");
}
