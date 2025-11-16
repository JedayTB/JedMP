# JedMP
I couldn't find a music player I liked on linux. So,I decided to make an open source music player written using Rust!

## TODO
Most recent TODO update - 2025-11-16 02:20 EST

RODIO Supports - MP3, WAV, VORBIS, FLAC , MP4 and AAC (Disabled by default, only handle by symphonia)

TODO List:

- Artist view that sorts Music Library for each Playlist
- Current playing song 
- Better UI. (Probable Custom drawn elements)

## Starting development

There are a few things that you must have first to begin developing on JedMP, those being Rust, and the dependencies used - So not much! Of course, you must clone the repo first. \Here is the snippet to do so


```
git clone https://github.com/JedayTB/JedMP.git
```


To Download rust do: \
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Rust Crates Dependencies 

Rodio - Music playing\
FLTK-RS - GUI\
taglib - Reading Music file metadata\
whoami - Getting information about user's system.\
libtags - Getting tag info from audio files
