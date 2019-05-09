#[macro_use]
extern crate clap;
use clap::App;

use std::process::Command;
use std::{thread, time};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut itunes = itunes();

    if matches.is_present("play") {
        execute(&mut itunes, "play")
    }

    if matches.is_present("pause") {
        execute(&mut itunes, "pause")
    }

    if matches.is_present("next") {
        execute(&mut itunes, "play next track")
    }

    if matches.is_present("previous") {
        execute(&mut itunes, "play previous track")
    }

    if matches.is_present("stop") {
        execute(&mut itunes, "stop")
    }

    if let Some(matches) = matches.subcommand_matches("playlist") {
        if matches.is_present("list") {
            let action = "get name of playlists";
            let execute = format!("tell application \"iTunes\" to {}", action);

            let output = itunes
                .arg("-e")
                .arg(execute)
                .output()
                .expect("failed to execute process");

            let output = String::from_utf8_lossy(&output.stdout);

            let playlists: Vec<&str> = output.split(",").collect();

            for playlist in &playlists {
                println!("{}", playlist);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("play") {
        if matches.is_present("song") {
            let target = matches.value_of("song").unwrap();
            let action: &str = &&format!("play track \"{}\"", target);
            execute(&mut itunes, action)
        } else if matches.is_present("playlist") {
            let target = matches.value_of("song").unwrap();
            let action: &str = &&format!("play playlist \"{}\"", target);
            execute(&mut itunes, action)
        } else {
            execute(&mut itunes, "play")
        }
    }

    if let Some(matches) = matches.subcommand_matches("flag") {
        if matches.is_present("love") {
            execute(&mut itunes, "set loved of current track to true")
        } else if matches.is_present("dislike") {
            execute(&mut itunes, "set disliked of current track to true")
        } else if matches.is_present("clear") {
            execute(&mut itunes, "set loved of current track to false");
            execute(&mut itunes, "set disliked of current track to false")
        }
    }

    if let Some(matches) = matches.subcommand_matches("add-to") {
        if matches.is_present("library") {
            execute(&mut itunes, "duplicate current track to source \"Library\"")
        }
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
