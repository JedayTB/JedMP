pub mod gui_controller {
    use crate::music_cache_handler::music_file_handler;
    use crate::music_play_queue_handler::play_queue_handler::{
        PLAY_QUEUE, PLAY_QUEUE_INDEX, decrement_play_queue_index, increment_play_queue_index,
    };
    use crate::song_identifier::{SongIdentifier, SongIdentifierType};
    use fltk::dialog;
    use fltk::group::Flex;
    use fltk::{app, button::Button, enums::*, group::Pack, prelude::*, window::Window};

    use fltk_theme::{ColorTheme, color_themes};
    use rodio::{OutputStream, Sink};
    use std::cell::RefCell;

    use std::rc::Rc;

    // Functions
    pub fn open_window() {
        // GUI Stuff
        //
        // GUI Element creation and positioning
        let app = app::App::default().with_scheme(app::Scheme::Oxy);
        let theme = ColorTheme::new(color_themes::TAN_THEME);
        theme.apply();
        let base_window_width = 896;
        let base_window_height = 504;

        let general_y_pad = 10;
        //let general_x_pad = 15;

        let mut wind = Window::new(0, 0, base_window_width, base_window_height, "JedMP");

        let top_bar_height = 25;

        // Top Bar
        let mut top_bar_group = Flex::default()
            .with_size(base_window_width, top_bar_height)
            .with_pos(0, 0);

        top_bar_group.set_frame(FrameType::GtkDownFrame);

        let mut add_music_directory_button = Button::default()
            .with_size(base_window_width / 12, top_bar_height)
            .with_label("Choose Music directory");

        top_bar_group.end();

        let library_list_width = 500;
        let library_list_height = 300;

        let library_list_pos_x = 0;
        let library_list_pos_y = 0;
        let mut library_list = Flex::default()
            .column()
            .with_size(library_list_width, library_list_height)
            .with_pos(
                library_list_pos_x,
                library_list_pos_y + general_y_pad + top_bar_height,
            );

        library_list.set_frame(FrameType::GtkDownFrame);

        let shared_library_list = Rc::new(RefCell::new(library_list.clone()));
        library_list.end();

        let button_box_height = base_window_height / 8;
        let button_box_width = base_window_width;
        let button_box_pos_y = wind.h();
        let button_box_pos_x = base_window_width / 2;

        let button_box = Flex::default()
            .with_size(button_box_width, button_box_height)
            .with_pos(
                button_box_pos_x - button_box_width / 2,
                button_box_pos_y - button_box_height,
            )
            .row();

        let mut last_song_button = Button::default().with_label("<");
        let mut pause_song_button = Button::default().with_label("Pause");
        let mut next_song_button = Button::default().with_label(">");

        button_box.end();
        let x_pad_from_lib = 25;
        let play_queue_box_width = 250;
        let play_queue_box_height = 300;

        let mut play_queue_box = Flex::default()
            .column()
            .with_size(play_queue_box_width, play_queue_box_height)
            .right_of(&library_list, x_pad_from_lib);
        play_queue_box.set_frame(FrameType::GtkDownBox);
        // GUI state variables creation

        // Because Im bad at coding, this must be called before anything to do with
        // play queue is done.
        music_file_handler::try_load_cached_music();

        make_library_list_frames(&mut library_list);
        make_queue_list_frames(&mut play_queue_box);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Rc::new(RefCell::new(Sink::try_new(&stream_handle).unwrap()));
        let sink_pause = Rc::clone(&sink);
        let sink_next = Rc::clone(&sink);
        let sink_last = Rc::clone(&sink);

        last_song_button.set_callback(move |_| {
            // Goes back a song. Replays song if already at 0th index
            let play_ind = decrement_play_queue_index().unwrap_or(0);

            let next_song_path = PLAY_QUEUE.read().unwrap()[play_ind].clone();
            let new_source = music_file_handler::load_path(&next_song_path.song_path);
            sink_last.borrow().stop();
            sink_last.borrow().append(new_source);
            sink_last.borrow().play();
        });

        next_song_button.set_callback(move |_| {
            let play_ind = increment_play_queue_index();

            if play_ind == None {
                // Other logic here, check if replay playlist is on for example.
                // (Future feature)

                // We've reached end of play queue.
                sink_next.borrow().stop();
            } else {
                let next_song_path = PLAY_QUEUE.read().unwrap()[play_ind.unwrap()].clone();
                let next_source = music_file_handler::load_path(&next_song_path.song_path);

                sink_next.borrow().stop();
                sink_next.borrow().append(next_source);
                sink_next.borrow().play();
            }
        });

        pause_song_button.set_callback(move |btn| {
            if sink.borrow().empty() {
                let ind = PLAY_QUEUE_INDEX.read().unwrap();
                let path = PLAY_QUEUE.read().unwrap()[*ind].clone();
                let source = music_file_handler::load_path(&path.song_path);
                // Stops playback and clears all appened files
                sink.borrow().stop();
                sink.borrow().append(source);
                sink.borrow().play();
            }

            if sink_pause.borrow().is_paused() {
                sink.borrow().play();
                btn.set_label("Pause");
            } else {
                sink_pause.borrow().pause();
                btn.set_label("Play");
            }
        });

        add_music_directory_button.set_callback(move |_| {
            let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseDir);
            nfc.set_option(dialog::NativeFileChooserOptions::SaveAsConfirm);
            match nfc.try_show() {
                Err(e) => {
                    eprintln!("{}", e);
                    //None
                }

                Ok(a) => match a {
                    dialog::NativeFileChooserAction::Success => {
                        let directory = nfc.filename();
                        let strname = directory
                            .to_str()
                            .expect("Directory doesn't have a string name?..");

                        music_file_handler::process_chosen_song_directory(strname);
                        music_file_handler::load_cached_songs();

                        make_library_list_frames(&mut *shared_library_list.borrow_mut());
                    }
                    dialog::NativeFileChooserAction::Cancelled => {
                        println!("Directory Pick cancelled");
                    }
                },
            }
        });
        wind.end();
        wind.make_resizable(true);
        wind.show();
        app.run().unwrap();
    }

    fn make_library_list_frames(library_list_box: &mut Flex) {
        for song in PLAY_QUEUE.read().unwrap().iter() {
            let si = SongIdentifier::new(
                100,
                30,
                &song.song_title,
                fltk::enums::Align::Right,
                SongIdentifierType::LIBRARY,
                song.to_owned(),
                None,
            );
            library_list_box.add(&*si);
        }
    }

    fn make_queue_list_frames(play_queue_box: &mut Flex) {
        let inner_pad = 2;
        let pq_box_width = play_queue_box.w() - inner_pad;
        let pq_box_height = play_queue_box.h() - inner_pad;

        let mut pack = Pack::default().with_size(pq_box_width, pq_box_height);
        pack.set_spacing(inner_pad);
        play_queue_box.add(&pack);
        let mut i: i32 = 0;
        for queued_song in PLAY_QUEUE.read().unwrap().iter() {
            let song_iden = SongIdentifier::new(
                pq_box_width,
                pq_box_height,
                &queued_song.song_title,
                fltk::enums::Align::Right,
                SongIdentifierType::PLAYQUEUE,
                queued_song.to_owned(),
                Some(i as usize),
            );
            pack.add(&*song_iden);
            i += 1;
        }
        play_queue_box.recalc();
        pack.auto_layout();
    }
}
