pub mod popup_window {

    use std::cell::RefCell;
    use std::rc::Rc;

    use fltk::{button::Button, enums::*, prelude::*, *};

    use crate::{
        music_play_queue_handler::play_queue_handler, play_queue_song::PlayQueueSong,
        song_identifier::SongIdentifierType,
    };

    pub const LIBRARY_OPTIONS: &'static str = "Add To Queue,Insert Next";
    pub const PLAYQUEUE_OPTIONS: &'static str = "Remove This,Play Now,Stop after";

    widget_extends!(PopupWindow, window::Window, win);
    pub struct PopupWindow {
        win: window::Window,
    }

    impl PopupWindow {
        pub fn new(pwin_type: &SongIdentifierType, song: PlayQueueSong, index: usize) -> Self {
            let mut win = window::Window::default();
            win.set_color(Color::White);
            win.set_frame(FrameType::BorderBox);

            let mut pack = group::Pack::new(1, 1, win.w() - 2, win.h() - 2, None);
            win.set_border(false);

            let mut _choices: Vec<&str> = Vec::new();

            // Kind of ugly. but whatever.
            match pwin_type {
                SongIdentifierType::LIBRARY => {
                    _choices = crate::popup_window::popup_window::LIBRARY_OPTIONS
                        .split(",")
                        .collect();
                    let mut add_queue_but = Button::default()
                        .with_label(_choices[0])
                        .with_size(_choices[0].len() as i32 * 10, 25);
                    let mut insert_next_but = Button::default()
                        .with_label(_choices[1])
                        .with_size(_choices[1].len() as i32 * 10, 25);

                    let song_: Rc<RefCell<PlayQueueSong>> = Rc::new(RefCell::new(song));
                    let song__ = Rc::clone(&song_);

                    add_queue_but.set_callback(move |_| {
                        play_queue_handler::append_to_playqueue(song_.borrow().clone())
                    });
                    insert_next_but.set_callback(move |_| {
                        play_queue_handler::insert_song_into_playqueue(
                            song__.borrow().clone(),
                            index,
                        )
                    });
                }
                SongIdentifierType::PLAYQUEUE => {
                    _choices = crate::popup_window::popup_window::PLAYQUEUE_OPTIONS
                        .split(",")
                        .collect();
                    let mut remove_this_but = Button::default()
                        .with_label(_choices[0])
                        .with_size(_choices[0].len() as i32 * 10, 25);

                    let mut play_now_but = Button::default()
                        .with_label(_choices[1])
                        .with_size(_choices[1].len() as i32 * 10, 25);
                    let mut stop_after_but = Button::default()
                        .with_label(_choices[2])
                        .with_size(_choices[2].len() as i32 * 10, 25);

                    remove_this_but.set_callback(|_| println!("Not implemented yet"));
                    play_now_but.set_callback(|_| println!("Not implemented yet"));
                    stop_after_but.set_callback(|_| println!("Not implemented yet"));
                }
            }

            win.handle(move |win, event| match event {
                Event::Unfocus => {
                    win.hide();
                    true
                }
                _ => false,
            });

            win.set_size(100, _choices.len() as i32 * 25);
            pack.set_size(100, _choices.len() as i32 * 25);

            pack.auto_layout();
            win.show();
            win.end();
            Self { win }
        }
    }
}
