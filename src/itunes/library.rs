use crate::itunes;

pub fn add_to_library() {
    let mut itunes = itunes::client::new();
    itunes::client::execute(&mut itunes, "duplicate current track to source \"Library\"");
}

pub fn love_current_song() {
    let mut itunes = itunes::client::new();
    itunes::client::execute(&mut itunes, "set loved of current track to true");
}

pub fn dislike_current_song() {
    let mut itunes = itunes::client::new();
    itunes::client::execute(&mut itunes, "set disliked of current track to true");
}

pub fn clear_flags() {
    let mut itunes = itunes::client::new();
    itunes::client::execute(&mut itunes, "set loved of current track to false");
    itunes::client::execute(&mut itunes, "set disliked of current track to false");
}
