use fltk::{enums::Align, group::Flex, prelude::*, text::*, *};

// Add to group by using group.add(&*SongIdentifier), because it needs to be dereferenced
pub struct SongIdentifier {
    group: Flex,
    song_name_text: TextDisplay,
}

impl SongIdentifier {
    pub fn new(w: i32, h: i32, song_name: &str, alignment: Align) -> SongIdentifier {
        let mut group = Flex::default().with_size(w, h);
        let mut song_name_text = text::TextDisplay::default().center_of(&group);
        let mut txt_buffer = TextBuffer::default();
        txt_buffer.set_text(song_name);
        song_name_text.set_buffer(txt_buffer);
        song_name_text.set_align(alignment);
        song_name_text.set_frame(enums::FrameType::NoBox);
        group.set_align(alignment);
        group.set_frame(enums::FrameType::GtkUpBox);
        //let col = Color::from_rgb(100, 0, 100);
        SongIdentifier {
            group,
            song_name_text,
        }
    }
}
widget_extends!(SongIdentifier, group::Flex, group);
