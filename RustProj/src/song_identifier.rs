use crate::popup_window;
use fltk::{
    enums::{Align, Event},
    group::Flex,
    prelude::*,
    text::*,
    *,
};
// Add to group by using group.add(&*SongIdentifier), because it needs to be dereferenced
pub struct SongIdentifier {
    group: Flex,
    _song_name_text: TextDisplay,
}

impl SongIdentifier {
    pub fn new(w: i32, h: i32, song_name: &str, alignment: Align) -> SongIdentifier {
        let mut group = Flex::default().with_size(w, h);
        let mut _song_name_text = text::TextDisplay::default().center_of(&group);
        let mut txt_buffer = TextBuffer::default();
        txt_buffer.set_text(song_name);
        _song_name_text.set_buffer(txt_buffer);
        _song_name_text.set_align(alignment);
        _song_name_text.set_frame(enums::FrameType::NoBox);
        group.set_align(alignment);
        group.set_frame(enums::FrameType::GtkUpBox);

        group.handle(|_widg, event| match event {
            Event::Push => {
                if app::event_mouse_button() == app::MouseButton::Right {
                    let mx = app::event_x_root();
                    let my = app::event_y_root();
                    let lol = "Add To Queue,Insert Next,Filler";
                    let parts: Vec<&str> = lol.split(",").collect();
                    let _popwin =
                        popup_window::popup_window::PopupWindow::new(50, &parts).with_pos(mx, my);
                }
                true
            }
            _ => false,
        });
        group.end();
        //let col = Color::from_rgb(100, 0, 100);
        SongIdentifier {
            group,
            _song_name_text,
        }
    }
}
widget_extends!(SongIdentifier, group::Flex, group);
