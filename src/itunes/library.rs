use crate::itunes;

pub fn add_to_library() {
    itunes::client::execute("duplicate current track to source \"Library\"");
}

pub fn love_current_song() {
    itunes::client::execute("set loved of current track to true");
}

pub fn dislike_current_song() {
    itunes::client::execute("set disliked of current track to true");
}

pub fn clear_flags() {
    itunes::client::execute("set loved of current track to false");
    itunes::client::execute("set disliked of current track to false");
}
