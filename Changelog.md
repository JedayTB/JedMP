# 2025-07-10 Change: Working on popup windows
Broken: PopupWindow opens on wrong screen if 2 monitor, possible fix is to add it as child of main window
Added popup_window.rs
Inside SongIdentifier constructor, added handling for mouse event on right click to spawn a new popup window
