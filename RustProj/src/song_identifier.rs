// Song link IS used.. Stupid.
#![allow(dead_code)]

use crate::{play_queue_song::PlayQueueSong, popup_window};
use fltk::{
    enums::{Align, Event},
    group::Flex,
    prelude::*,
    text::*,
    *,
};
// Refactor to SongDisplayer
pub enum SongIdentifierType {
    LIBRARY,
    PLAYQUEUE,
}
// Having PlayQueueSong is bad. But because Library views haven't had much work, its somewhat
// necessary.
// Add to group by using group.add(&*SongIdentifier), because it needs to be dereferenced
pub struct SongIdentifier {
    group: Flex,
    song_link: PlayQueueSong,
    index_in_list: Option<usize>,
}
// Constructor functions
impl SongIdentifier {
    pub fn new(
        w: i32,
        h: i32,
        song_name: &str,
        alignment: Align,
        iden_type: SongIdentifierType,
        song_link: PlayQueueSong,
        index_in_list: Option<usize>,
    ) -> SongIdentifier {
        let mut group = Flex::default().with_size(w, h);

        let mut _song_name_text = text::TextDisplay::default().center_of(&group);
        let mut txt_buffer = TextBuffer::default();
        txt_buffer.set_text(song_name);
        _song_name_text.set_buffer(txt_buffer);
        _song_name_text.set_align(alignment);
        _song_name_text.set_frame(enums::FrameType::NoBox);
        group.set_align(alignment);
        group.set_frame(enums::FrameType::GtkUpBox);
        let song_clone = song_link.clone();

        group.handle(move |_widg, event| match event {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Right {
                    let mx = app::event_x_root();
                    let my = app::event_y_root();
                    let _popwin = popup_window::popup_window::PopupWindow::new(
                        &iden_type,
                        song_clone.clone(),
                        index_in_list,
                    )
                    .with_pos(mx, my);
                }
                true
            }
            _ => false,
        });
        group.end();
        SongIdentifier {
            group,
            song_link,
            index_in_list,
        }
    }
}

widget_extends!(SongIdentifier, group::Flex, group);
