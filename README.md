# JedMP
I couldn't find a music player I liked on Linux. So,I decided to make an open source music player written using Rust! JedMP is inspired by MusicBee on windows, unfortunately that program is Windows only and not open source.

Tauon is also a competent music player. Though, if you're like me.. It crashes. Too often. I suppose that'd the drawback of using Python

## TODO
Most recent TODO update - 2025-11-27 17:51 EST

RODIO Supports - MP3, WAV, VORBIS, FLAC , MP4 and AAC (Disabled by default, only handle by symphonia)

TODO List:

- Music playing that accepts almost all audio codecs
- Current playing song
    - time updates
    - progress bar
    - artist
    - album art somewhere
- Library search that takes into account currently displayed artist
- Library Shuffle


## Starting development

First, of course: \
```
git clone https://github.com/JedayTB/JedMP.git
```


To Download rust do: \
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This also downloads extra tools for development.

### Rust Crates Dependencies 

Rodio - Music playing\
FLTK-RS - GUI\
taglib - Reading Music file metadata\
whoami - Getting information about user's system.\
discord-presence - Discord presence\

### Package Depedencies

```
linux-vdso && libtag_c && libasound && libX11 && libXinerama && libXcursor && 
libXfixes && libfontconfig && libpango-1.0 && libgobject-2.0 && libcairo && 
libpangocairo-1.0 && libgcc_s && libm && libc && libtag && libstdc++ && libxcb 
&& libXext && libXrender && libfreetype && libexpat && libglib-2.0 && libgio-2.0
&& libfribidi && libthai && libharfbuzz && libffi && libz && libpng16 && libxcb-render && 
libxcb-shm && libpixman-1 && libpangoft2-1.0 && libXau && libXdmcp && libbz2 && libbrotlidec 
&& libpcre2-8 && libgmodule-2.0 && libmount && libselinux && libdatrie &&  libgraphite2 && 
libbsd && libbrotlicommon && libblkid && libmd
```
