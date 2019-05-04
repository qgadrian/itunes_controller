## iTunes controller

Simple iTunes command line controller written in [Rust](http://rust-lang.org/).

This application delegates on [Apple
script](https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/introduction/ASLR_intro.html)
to call the system application to execute the different actions.

## Build

Install [Rust](https://www.rust-lang.org/tools/install).

Compile the project with `cargo build --release`.

## Actions

If iTunes it's not open, any action will start the application first.

### Play/Pause

`itunes play`

`itunes pause`

### Next/previous track

`itunes next`

`itunes previous`

### Play a song

> To avoid confusions, you can use either `song` or `track` with this command.

`itunes play song "We will rock you"`

`itunes play track "We will rock you"`

### Play a playlist

`itunes play playlist "Classical music"`

### Love current song

`itunes love`

### Remove love current song

`itunes love clear`

### Dislike current song

`itunes dislike`

### Remove dislike current song

`itunes dislike clear`
