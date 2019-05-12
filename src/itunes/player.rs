use crate::itunes;

pub fn play() {
    do_action("play");
}

pub fn pause() {
    do_action("pause");
}

pub fn playpause() {
    do_action("playpause");
}

pub fn stop() {
    do_action("stop");
}

pub fn next() {
    do_action("play next track");
}

pub fn previous() {
    do_action("play previous track");
}

// Target play

pub fn play_song(song: &str) {
    let action: &str = &&format!("play track \"{}\"", song);
    do_action(action);
}

pub fn play_playlist(playlist: &str) {
    let action: &str = &&format!("play playlist \"{}\"", playlist);
    do_action(action);
}

fn do_action(action: &str) {
    itunes::client::execute(action)
}
