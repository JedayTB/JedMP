# 2025-07-10 Change: Play_queue visualization partly implemented
popup_window now uses the mouse position on the screen instead of window, this fixes the window from appearing on the wrong screen
Created Play_queue_box inside GUI handler thats populated with flexboxes
# 2025-07-10 Change: Working on popup windows
Broken: PopupWindow opens on wrong screen if 2 monitor, possible fix is to add it as child of main window
Added popup_window.rs
Inside SongIdentifier constructor, added handling for mouse event on right click to spawn a new popup window
