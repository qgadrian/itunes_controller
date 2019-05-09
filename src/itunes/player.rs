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

fn do_action(action: &str) {
    let mut itunes = itunes::client::new();
    itunes::client::execute(&mut itunes, action)
}
