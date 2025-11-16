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

### Package Depedencies

```
linux-vdso && libtag_c && libasound && libX11 && libXinerama && libXcursor && libXfixes && libfontconfig && libpango-1.0 && libgobject-2.0 && libcairo && libpangocairo-1.0 && libgcc_s && libm && libc && libtag && libstdc++ && libxcb && libXext && libXrender && libfreetype && libexpat && libglib-2.0 && libgio-2.0 && libfribidi && libthai && libharfbuzz && libffi && libz && libpng16 && libxcb-render && libxcb-shm && libpixman-1 && libpangoft2-1.0 && libXau && libXdmcp && libbz2 && libbrotlidec && libpcre2-8 && libgmodule-2.0 && libmount && libselinux && libdatrie &&  libgraphite2 && libbsd && libbrotlicommon && libblkid && libmd
```
