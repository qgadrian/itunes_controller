#[macro_use]
extern crate clap;
use clap::App;

mod itunes;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("play") {
        itunes::player::play();
    }

    if matches.is_present("pause") {
        itunes::player::pause();
    }

    if matches.is_present("playpause") {
        itunes::player::playpause();
    }

    if matches.is_present("stop") {
        itunes::player::stop();
    }

    if matches.is_present("next") {
        itunes::player::next();
    }

    if matches.is_present("previous") {
        itunes::player::previous();
    }

    if let Some(matches) = matches.subcommand_matches("playlist") {
        if matches.is_present("list") {
            let action = "get name of playlists";
            let execute = format!("tell application \"iTunes\" to {}", action);

            let output = Command::new("osascript")
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
            let song = matches.value_of("song").unwrap();
            itunes::player::play_song(song);
        } else if matches.is_present("playlist") {
            let playlist = matches.value_of("playlist").unwrap();
            itunes::player::play_playlist(playlist);
        } else {
            itunes::client::execute("play")
        }
    }

    if let Some(matches) = matches.subcommand_matches("flag") {
        if matches.is_present("love") {
            itunes::library::love_current_song();
        } else if matches.is_present("dislike") {
            itunes::library::dislike_current_song();
        } else if matches.is_present("clear") {
            itunes::library::clear_flags();
        }
    }

    if let Some(matches) = matches.subcommand_matches("add-to") {
        if matches.is_present("library") {
            itunes::library::add_to_library();
        }
    }
}
