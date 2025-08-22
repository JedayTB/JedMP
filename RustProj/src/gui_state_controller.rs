pub mod gui_controller {
    use crate::music_cache_handler::music_file_handler;
    use crate::music_play_queue_handler::play_queue_handler::{
        PLAY_QUEUE, PLAY_QUEUE_INDEX, decrement_play_queue_index, increment_play_queue_index,
    };
    use crate::play_queue_song::PlayQueueSong;
    use crate::song_identifier::{SongIdentifier, SongIdentifierType};
    use fltk::dialog;
    use fltk::group::Flex;
    use fltk::{app, button::Button, enums::*, group::*, prelude::*, window::Window};

    use fltk_theme::{ColorTheme, color_themes};
    use rodio::{OutputStream, Sink};
    use std::cell::RefCell;
    use std::rc::Rc;

    use std::sync::RwLock;
    static SHARED_PLAY_QUEUE_GUI: RwLock<Vec<Pack>> = RwLock::new(Vec::new());
    // Embrace the shit code. Another Global
    static SHARED_SINK: RwLock<Vec<Sink>> = RwLock::new(Vec::new());
    static IN_PLAY_QUEUE_BOX_HEIGHT: i32 = 40;
    static IN_PLAY_QUEUE_BOX_WIDTH: i32 = 100;

    // Functions
    //
    //
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
        let play_queue_box_width = 350;
        let play_queue_box_height = 300;

        let mut play_queue_box = Scroll::default()
            .with_size(play_queue_box_width, play_queue_box_height)
            .right_of(&library_list, x_pad_from_lib);

        play_queue_box.set_type(fltk::group::ScrollType::Vertical);
        play_queue_box.set_frame(FrameType::PlasticDownBox);
        play_queue_box.end();

        // GUI state variables creation

        make_library_list_frames(&mut library_list);
        make_queue_list_frames(&mut play_queue_box);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let s = Sink::try_new(&stream_handle).unwrap();
        SHARED_SINK.write().unwrap().push(s);

        last_song_button.set_callback(move |_| {
            // Goes back a song. Replays song if already at 0th index
            let play_ind = decrement_play_queue_index().unwrap_or(0);

            let next_song_path = PLAY_QUEUE.read().unwrap()[play_ind].clone();
            let new_source = music_file_handler::load_path(&next_song_path.song_path);
            SHARED_SINK.write().unwrap()[0].stop();
            SHARED_SINK.write().unwrap()[0].append(new_source);
            SHARED_SINK.write().unwrap()[0].play();
        });

        next_song_button.set_callback(move |_| {
            let play_ind = increment_play_queue_index();

            if play_ind == None {
                // Other logic here, check if replay playlist is on for example.
                // (Future feature)

                // We've reached end of play queue.

                SHARED_SINK.write().unwrap()[0].stop();
            } else {
                let next_song_path = PLAY_QUEUE.read().unwrap()[play_ind.unwrap()].clone();
                let next_source = music_file_handler::load_path(&next_song_path.song_path);

                SHARED_SINK.write().unwrap()[0].stop();
                SHARED_SINK.write().unwrap()[0].append(next_source);
                SHARED_SINK.write().unwrap()[0].play();
            }
        });

        pause_song_button.set_callback(move |btn| {
            if SHARED_SINK.read().unwrap()[0].empty() {
                let ind = PLAY_QUEUE_INDEX.read().unwrap();
                let path = PLAY_QUEUE.read().unwrap()[*ind].clone();
                let source = music_file_handler::load_path(&path.song_path);
                // Stops playback and clears all appened files

                SHARED_SINK.write().unwrap()[0].stop();
                SHARED_SINK.write().unwrap()[0].append(source);
                SHARED_SINK.write().unwrap()[0].play();
            }

            if SHARED_SINK.read().unwrap()[0].is_paused() {
                SHARED_SINK.write().unwrap()[0].play();
                btn.set_label("Pause");
            } else {
                SHARED_SINK.write().unwrap()[0].pause();
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
                fltk::enums::Align::Center,
                SongIdentifierType::LIBRARY,
                song.to_owned(),
                None,
            );
            library_list_box.add(&*si);
        }
    }

    fn make_queue_list_frames(play_queue_box: &mut Scroll) {
        // yes this is jank as fuck. No I don't care.
        SHARED_PLAY_QUEUE_GUI.write().unwrap().clear();

        let mut pack = Pack::default_fill();
        pack.make_resizable(true);
        play_queue_box.add(&pack);

        let mut i: i32 = 0;
        for queued_song in PLAY_QUEUE.read().unwrap().iter() {
            let song_iden = SongIdentifier::new(
                IN_PLAY_QUEUE_BOX_WIDTH,
                IN_PLAY_QUEUE_BOX_HEIGHT,
                &queued_song.song_title,
                fltk::enums::Align::Center,
                SongIdentifierType::PLAYQUEUE,
                queued_song.to_owned(),
                Some(i as usize),
            );
            pack.add(&*song_iden);
            i += 1;
        }
        pack.end();

        play_queue_box.scroll_to(-527, -40);
        SHARED_PLAY_QUEUE_GUI.write().unwrap().push(pack);
    }

    pub fn append_song_to_queue(pq_song: PlayQueueSong) {
        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUE.read().unwrap().len() - 1),
        );
        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].add(&*song_iden);
        //SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].auto_layout();
        app::redraw();
    }
    pub fn insert_song_to_queue(pq_song: PlayQueueSong, current_index: usize) {
        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUE.read().unwrap().len() - 1),
        );

        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].insert(&*song_iden, current_index as i32);
        app::redraw();
    }

    pub fn sink_play_instant(pq_song: PlayQueueSong) {
        let source = music_file_handler::load_path(&pq_song.song_path);
        // Stops playback and clears all appened files

        SHARED_SINK.write().unwrap()[0].stop();
        SHARED_SINK.write().unwrap()[0].append(source);
        SHARED_SINK.write().unwrap()[0].play();
    }
    pub fn remove_song_from_playqueue(rm_index: usize) {
        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].remove_by_index(rm_index as i32);
    }
}
