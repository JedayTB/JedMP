pub mod popup_window {

    use std::cell::RefCell;
    use std::rc::Rc;

    use fltk::{button::Button, enums::*, prelude::*, *};

    use crate::{
        gui_state_controller,
        music_play_queue_handler::play_queue_handler::{self, PLAY_QUEUE_INDEX},
        play_queue_song::PlayQueueSong,
        song_identifier::SongIdentifierType,
    };

    pub const LIBRARY_OPTIONS: &'static str = "Add To Queue,Insert Next";
    pub const PLAYQUEUE_OPTIONS: &'static str = "Remove This,Play Now,Stop after";

    widget_extends!(PopupWindow, window::Window, win);
    pub struct PopupWindow {
        win: window::Window,
    }
    impl PopupWindow {
        pub fn new(
            pwin_type: &SongIdentifierType,
            song: PlayQueueSong,
            _index: Option<usize>,
        ) -> Self {
            let mut win = window::Window::default();
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

                    // Ultra ugly but lol
                    // This is window, so goes
                    // window, song Ident, Play_queue_box

                    //let mut pq_box: Flex = win.parent().unwrap(); //.parent().unwrap().parent().unwrap();

                    add_queue_but.set_callback(move |_| {
                        println!("Appended to pq");

                        play_queue_handler::append_to_playqueue(song_.borrow().clone());

                        gui_state_controller::gui_controller::append_song_to_queue(
                            song_.borrow().clone(),
                        );
                    });
                    insert_next_but.set_callback(move |_| {
                        println!("Inserted in pq");
                        let current_index = PLAY_QUEUE_INDEX.read().unwrap();
                        play_queue_handler::insert_song_into_playqueue(
                            song__.borrow().clone(),
                            *current_index,
                        );
                        gui_state_controller::gui_controller::insert_song_to_queue(
                            song__.borrow().clone(),
                            current_index.clone(),
                        );
                    });
                }
                SongIdentifierType::PLAYQUEUE => {
                    let song_: Rc<RefCell<PlayQueueSong>> = Rc::new(RefCell::new(song));
                    let song__ = Rc::clone(&song_);
                    let song___ = Rc::clone(&song_);

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

                    remove_this_but.set_callback(move |_| {
                        play_queue_handler::remove_song_at_index(
                            song_.borrow().clone().index_in_play_queue,
                        );
                        gui_state_controller::gui_controller::remove_song_from_playqueue(
                            song_.borrow().clone().index_in_play_queue,
                        );
                    });

                    play_now_but.set_callback(move |_| {
                        play_queue_handler::play_song_instant(
                            song__.borrow().clone().index_in_play_queue,
                        );
                        gui_state_controller::gui_controller::sink_play_instant(
                            song__.borrow().clone(),
                        );
                    });

                    stop_after_but.set_callback(move |_| println!("Not Implemented yet"));
                }
            }

            win.handle(move |win, event| match event {
                Event::Leave => {
                    win.hide();
                    true
                }

                _ => false,
            });

            pack.set_size(100, _choices.len() as i32 * 25);
            win.set_size(100, _choices.len() as i32 * 25);

            pack.auto_layout();
            win.show();
            win.end();
            Self { win }
        }
    }
}
