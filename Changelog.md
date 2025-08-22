# 20225-08-22 Change: A few globals. Functions in play_queue songs work. Full Directory scanning.
I didn't bother with keeping track of my changes because of the large gap between now and my last change. Alls to know is that play_queue is a little.. eh. Jank (vec global only access [0th] element.) And that it's a mostly functional Music Player now.

# 2025-07-18 Change: Refactored All Play queue indexes as global inside music_play_queue_handler as RwLock type
As name suggests, play queue indexes no longer use dodgy Refcell types and now reference the Global

# 2025-07-13 Change: Play_queue_visualization finished
Added: SongIdentifierType: Enum, LIBRARYOPTIONS: &'static String, PLAYQUEUEOPTIONS: &'static String

# 2025-07-10 Change: Play_queue visualization partly implemented
popup_window now uses the mouse position on the screen instead of window, this fixes the window from appearing on the wrong screen
Created Play_queue_box inside GUI handler thats populated with flexboxes
# 2025-07-10 Change: Working on popup windows
Broken: PopupWindow opens on wrong screen if 2 monitor, possible fix is to add it as child of main window
Added popup_window.rs
Inside SongIdentifier constructor, added handling for mouse event on right click to spawn a new popup window
