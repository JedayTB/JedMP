# JedMP
I couldn't find a music player I liked on linux. So,I decided to make an open source music player written using Rust!

This is mostly a learning project for me (Jeday) to learn rust. But making a good music player is also cool

## TODO
Most recent TODO update - 2025-07-018 4:56 pm EST

RODIO Supports - MP3, WAV, VORBIS, FLAC , MP4 and AAC (Disabled by default, only handle by symphonia)

TODO List:


- Create play_queue, with functionality to change order of songs.
  - Play_queue functions (append, insert next, remove)
    Blockers
    - Popup window has odd unfocusing behavior
    - Right clicking on text region opens Copy text popup window native to FLTK
    - GUI Song frames to Play queue visualizer
- Music library view, scrollable 
  - Shuffle functions into queue
  - Full Music library 
  - Artist separated library
    - Album separated by artist library
- Current playing song 
- Better UI.

## Starting development

There are a few things that you must have first to begin developing on JedMP, those being Rust, and the dependencies used - So not much! Of course, you must clone the repo first. \Here is the snippet to do so


```
git clone https://github.com/JedayTB/JedMP.git
```


To Download rust do: \
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

There are multiple dependencies, but, this project uses cargo to handle them. For each dependency listed, do\
This may be unecessary as, the cargo.toml in the repo has these included.

```
cargo install {dependency_name}
```
### Dependencies 

Rodio - Music playing\
FLTK-RS - GUI\
whoami - Getting information about user's system.\
libtags - Getting tag info from audio files
