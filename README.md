## iTunes controller

Simple iTunes command line controller written in [Rust](http://rust-lang.org/).

This application delegates on [Apple
script](https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/introduction/ASLR_intro.html)
to call the system application to execute the different actions.

## Build

Install [Rust](https://www.rust-lang.org/tools/install).

Compile the project with `cargo build --release`.

## Usage

If iTunes it's not open, any action will start the application first. To see the
help with all the available options run `itunes help`.

## Examples

### Player

Basic player controls supported: `play`, `pause`, `playpause`, `stop`, `next`
and `previous`.

Either songs or playlists can be played, for example `itunes play song "We will
rock you"`.

### Library

The current song can be added to the library with `itunes add-to library`.
It can be marked as loved with `itunes flag love`, or dislike it with `itunes
flag dislike`.
