# Migrated to Codeberg - Fuck github
This repo has been migrated to Codeberg because Github sucks. Find it here:
```https://codeberg.org/Jeday_ERZ/JedMP```
 
 # JedMP
I couldn't find a music player I liked on Linux. So,I decided to make an open source music player written using Rust! JedMP is inspired by MusicBee on windows, unfortunately that program is Windows only and not open source.

Tauon is also a competent music player. Though, if you're like me.. It crashes. Too often. I suppose that'd the drawback of using Python

JedMP Makes use of LibVLC! They are amazing. You may ask "Why not just use VLC then?," to that we say. Have you seen their UI? No shade, of course. It just doesn't have the ease of use or functionality I would like out of my Music player. It does it's job spectacularly, however. Just different goals.

## TODO
Most recent TODO update - 2025-12-1 19:19 EST

TODO List (Unordered):

- Album view by artist's (MusicBee styled implementation)
- button on discord presence that direct's to this repo
- Shift and CTRL select
    - song_iden's 
    - artist_frames
    - album frames
- Current playing song
    - time updates
    - progress bar
    - artist
    - album art somewhere
- Library search that takes into account currently displayed artist
- Library Shuffle
- Non program blocking music scan + animation
- SVG assets for Last, Pause play and Next button

## Starting development

First, of course: 
```
git clone https://github.com/JedayTB/JedMP.git
```


To Download rust do: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This also downloads extra tools for development.

### Rust Crates Dependencies 

LibVlc - Music Playing and Metadata parsing\
FLTK-RS - GUI\
whoami - Getting information about user's system.\
discord-presence - Discord presence

Testing on a minimal debian 14 installation, I needed the following to start development on JedMP

- rust (rustup command from above)
- gcc
- g++
- libgtk-4-dev
- libvlc-dev

Hope this helps if you're having issues!

Missing system packages will likely propogate an error. Reference lddResponse_so_dependencies (Literally ran ldd on the binary file.)
