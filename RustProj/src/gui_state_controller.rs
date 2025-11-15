pub mod gui_controller {
    use crate::JButton::JButton::J_Button;
    use crate::Playlist_Tab::Playlist_Tab::PlaylistTab;
    use crate::colors_handler::color_handler::COLOR_DICTIONARY;
    use crate::colors_handler::color_handler::JedMP_Colors;
    use crate::colors_handler::color_handler::get_jedmp_color;
    use crate::get_jedmp_playlist_dir;
    use crate::music_cache_handler::music_file_handler;
    use crate::music_play_queue_handler::play_queue_handler::{
        PLAY_QUEUE_INDEX, PLAY_QUEUES, decrement_play_queue_index, increment_play_queue_index,
    };
    use crate::play_queue_song::PlayQueueSong;
    use crate::playlist_handler::playlist_handler::get_playlists_names;
    use crate::song_identifier::{SongIdentifier, SongIdentifierType};
    use fltk::dialog;
    use fltk::{app, enums::*, group::*, prelude::*, window::Window};

    use fltk_theme::{SchemeType, WidgetScheme};
    use rodio::{OutputStream, Sink};
    use std::cell::RefCell;
    use std::rc;
    use std::sync::RwLock;
    // Shit way of making a global
    static SHARED_PLAY_QUEUE_GUI: RwLock<Vec<Pack>> = RwLock::new(Vec::new());
    // Embrace the shit code. Another Global
    static SHARED_SINK: RwLock<Vec<Sink>> = RwLock::new(Vec::new());

    static CURRENT_PLAYLIST_INDEX: RwLock<usize> = RwLock::new(0);
    static PLAYLIST_COUNT: RwLock<usize> = RwLock::new(0);

    static IN_PLAY_QUEUE_BOX_HEIGHT: i32 = 30;
    static IN_PLAY_QUEUE_BOX_WIDTH: i32 = 100;

    pub static BASE_WINDOW_WIDTH: i32 = 896;
    pub static BASE_WINDOW_HEIGHT: i32 = 504;
    pub static GENERAL_X_PAD: i32 = 10;
    pub static GENERAL_Y_PAD: i32 = 10;
    pub static MENU_ARTISTVIEW_PAD: i32 = 61;

    //TODO:
    //Create custom frame rendering for Tabs using Tabs bg color.
    //In fact, likely use this all over the place.

    pub fn open_window() {
        // GUI Stuff
        //
        // GUI Element creation and positioning
        let app = app::App::default();

        let widgetscheme = WidgetScheme::new(SchemeType::Aqua);
        widgetscheme.apply();

        let mut wind = Window::default()
            .with_size(BASE_WINDOW_WIDTH, BASE_WINDOW_HEIGHT)
            .with_label("JedMP");
        wind.set_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Background_color as usize]);

        //  Add below for closable tabs
        //  col1.set_trigger(CallbackTrigger::Closed);
        //  col1.set_callback(tab_close_cb);

        let menu_button_width = 30;
        let menu_button_height = 10;

        let mut menu_button = J_Button::new()
            .with_size(menu_button_width, menu_button_height)
            .with_label("Menu")
            .with_pos(0, 0);
        let menu_artistview_pad = menu_button_width + 31;
        let button_box_height = BASE_WINDOW_HEIGHT / 12;
        let button_box_width = BASE_WINDOW_WIDTH;
        let button_box_pos_y = wind.h();
        let button_box_pos_x = BASE_WINDOW_WIDTH / 2;

        let button_box = Flex::default()
            .with_size(button_box_width - menu_artistview_pad, button_box_height)
            .with_pos(
                (button_box_pos_x - button_box_width / 2) + menu_artistview_pad,
                button_box_pos_y - button_box_height - 5,
            )
            .row();

        let mut last_song_button = J_Button::new().with_label("<");
        let mut pause_song_button = J_Button::new().with_label("Pause");
        let mut next_song_button = J_Button::new().with_label(">");

        button_box.end();

        let mut row = Flex::default()
            .with_size(
                BASE_WINDOW_WIDTH - menu_button_width,
                BASE_WINDOW_HEIGHT - button_box_height - 10,
            )
            .row()
            .with_pos(menu_artistview_pad, 0);
        row.set_color(get_jedmp_color(JedMP_Colors::Tabs_bg_color));
        let mut tabs = Tabs::default();
        tabs.handle_overflow(TabsOverflow::Compress);
        tabs.set_color(get_jedmp_color(JedMP_Colors::Background_color));
        tabs.set_callback(|t| {
            let current_playlist_index = t.push().expect("No tab was pushed?");
            //let t = current_playlist_index.get_type;
            //println!("{t}");
        });
        let shared_tabs = rc::Rc::new(RefCell::new(tabs.clone()));

        //===================================================================================
        //              Put everything wanted inside main tab under here
        //===================================================================================

        let mut main_lib_tab = Group::default()
            .with_label("Full Library")
            .with_size(BASE_WINDOW_WIDTH, BASE_WINDOW_HEIGHT);
        main_lib_tab.set_color(get_jedmp_color(JedMP_Colors::Background_color));
        let library_list_width = 500;
        let library_list_height = 300;

        let library_list_pos_x = GENERAL_X_PAD;
        let library_list_pos_y = 0;

        let mut library_list = Scroll::default()
            .with_size(library_list_width, library_list_height)
            .with_pos(library_list_pos_x, library_list_pos_y + GENERAL_Y_PAD);

        //library_list.set_type(fltk::group::ScrollType::Vertical);

        library_list.set_frame(FrameType::GtkDownFrame);
        library_list
            .set_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Background_color as usize]);
        library_list.end();
        let shared_library_list = rc::Rc::new(RefCell::new(library_list.clone()));

        let play_queue_box_width = 250;
        let play_queue_box_height = 300;

        let mut play_queue_box = Scroll::default()
            .with_size(play_queue_box_width, play_queue_box_height)
            .with_pos(
                (BASE_WINDOW_WIDTH - menu_artistview_pad) - play_queue_box_width,
                library_list_pos_y + GENERAL_Y_PAD,
            );

        //play_queue_box.set_type(fltk::group::ScrollType::Vertical);
        //play_queue_box.set_frame(FrameType::PlasticDownBox);

        play_queue_box.set_color(get_jedmp_color(JedMP_Colors::Background_color));
        play_queue_box.end();

        // GUI state variables creation

        make_library_list_frames(&mut library_list, 0);
        make_queue_list_frames(&mut play_queue_box, 0);

        main_lib_tab.end();
        tabs.end();
        tabs.auto_layout();

        row.end();

        wind.end();
        wind.make_resizable(true);
        wind.show();
        // Increment because 0th (1st) has been made
        *PLAYLIST_COUNT.write().unwrap() += 1;
        //
        //  Create callbacks
        //
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let s = Sink::try_new(&stream_handle).unwrap();
        SHARED_SINK.write().unwrap().push(s);
        last_song_button.set_callback(move |_| {
            let cur_pl = *CURRENT_PLAYLIST_INDEX.read().unwrap();
            // Goes back a song. Replays song if already at 0th index
            let play_ind = decrement_play_queue_index().unwrap_or(0);
            let next_song_path = PLAY_QUEUES.read().unwrap()[cur_pl][play_ind].clone();
            let new_source = music_file_handler::load_path(&next_song_path.song_path);
            SHARED_SINK.write().unwrap()[0].stop();
            SHARED_SINK.write().unwrap()[0].append(new_source);
            SHARED_SINK.write().unwrap()[0].play();
        });
        next_song_button.set_callback(move |_| {
            let cur_pl = *CURRENT_PLAYLIST_INDEX.read().unwrap();
            let play_ind = increment_play_queue_index(cur_pl);

            if play_ind == None {
                // Other logic here, check if replay playlist is on for example.
                // (Future feature)

                // We've reached end of play queue.

                SHARED_SINK.write().unwrap()[0].stop();
            } else {
                let cur_pl = *CURRENT_PLAYLIST_INDEX.read().unwrap();
                let next_song_path = PLAY_QUEUES.read().unwrap()[cur_pl][play_ind.unwrap()].clone();
                let next_source = music_file_handler::load_path(&next_song_path.song_path);

                SHARED_SINK.write().unwrap()[0].stop();
                SHARED_SINK.write().unwrap()[0].append(next_source);
                SHARED_SINK.write().unwrap()[0].play();
            }
        });

        pause_song_button.set_callback(move |btn| {
            if SHARED_SINK.read().unwrap()[0].empty() {
                let ind = PLAY_QUEUE_INDEX.read().unwrap();
                let cur_pl = CURRENT_PLAYLIST_INDEX.read().unwrap();
                let path = PLAY_QUEUES.read().unwrap()[*cur_pl][*ind].clone();
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

        // Quick and dirty custom choices
        let sh_lib_list = shared_library_list.clone();
        let sh_tab = shared_tabs.clone();
        menu_button.set_callback(move |mbut| {
            let mut cwind = Window::default()
                .with_pos(
                    mbut.x() + mbut.w() + mbut.label_size(),
                    mbut.y() + mbut.h() * 2,
                )
                .with_size(150, 100);

            cwind.set_border(false);
            let flex = Flex::default_fill().column();
            let mut add_mus_dir_but = J_Button::new().with_label("Add Music Directory");

            let sh_lib_inner = sh_lib_list.clone();
            add_mus_dir_but.set_callback(move |_| {
                let sh_inner_inner = sh_lib_inner.clone();
                let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseDir);
                nfc.set_option(dialog::NativeFileChooserOptions::SaveAsConfirm);
                match nfc.try_show() {
                    Err(e) => {
                        eprintln!("{}", e);
                        //None
                    }

                    Ok(a) => match a {
                        dialog::NativeFileChooserAction::Success => {
                            println!("Valid Directory Chosen, processing for music..");
                            let directory = nfc.filename();
                            let strname = directory
                                .to_str()
                                .expect("Directory doesn't have a string name?..");

                            music_file_handler::process_chosen_song_directory(strname);
                            music_file_handler::load_cached_songs();

                            make_library_list_frames(&mut *sh_inner_inner.borrow_mut(), 0);
                        }
                        dialog::NativeFileChooserAction::Cancelled => {
                            println!("Directory Pick cancelled");
                        }
                    },
                }
            });

            let mut add_playlist_tab = J_Button::new().with_label("Open Playlist");

            let sh_tab_inner = sh_tab.clone();

            add_playlist_tab.set_callback(move |_| {
                let mut wind = Window::default().with_size(500, 250);
                wind.set_border(false);
                wind.set_color(Color::from_rgb(100, 100, 100));

                let playlists = get_playlists_names();
                let mut playlist_picker: Scroll = Scroll::default().with_size(500, 200);

                let mut pl_pack = Pack::default_fill();
                pl_pack.make_resizable(true);
                playlist_picker.add(&pl_pack);

                let sh_tab_inner_inner = sh_tab_inner.clone();
                for playlist_name in playlists {
                    let mut temp_b = J_Button::new()
                        .with_label(&playlist_name)
                        .with_size(450, 30);
                    //let tsp = song.to_owned();
                    let sht = sh_tab_inner_inner.clone();
                    temp_b.set_callback(move |_| {
                        let pl_name = playlist_name.clone();
                        let sh_tab_for = sht.clone();
                        let jedmpPLDir = get_jedmp_playlist_dir();
                        let plPath = format!("{jedmpPLDir}/{playlist_name}");
                        let tab = &mut *sh_tab_for.borrow_mut();

                        *PLAYLIST_COUNT.write().unwrap() += 1;
                        let c = *PLAYLIST_COUNT.read().unwrap();
                        let mut newPlTab = PlaylistTab::new(plPath, pl_name, c);
                        newPlTab.set_trigger(CallbackTrigger::Closed);
                        newPlTab.set_callback(tab_close_cb);
                        tab.add(&*newPlTab);
                        tab.auto_layout();
                        // add to shared_tabs somehow. Borrow checker gonna be a bitch
                    });
                    pl_pack.add(&*temp_b);
                }
                pl_pack.end();

                playlist_picker.end();
                wind.show();
                wind.end();
            });

            flex.end();
            cwind.end();
            cwind.show();
        });

        /*
        wind.handle(|win, e: Event| match e {
            Event::Push => true,

            Event::Drag => true,

            Event::Released => true,
            _ => true,
        });
        */

        app.run().unwrap();
    }

    pub fn make_library_list_frames(library_list_box: &mut Scroll, which_pq: usize) {
        library_list_box.clear();

        let pl_ind = CURRENT_PLAYLIST_INDEX.read().unwrap().clone();
        let mut pack =
            Pack::default().with_size(library_list_box.width(), library_list_box.height()); //_fill();
        pack.make_resizable(true);
        library_list_box.add(&pack);

        for song in PLAY_QUEUES.read().unwrap()[pl_ind].iter() {
            let si = SongIdentifier::new(
                100,
                30,
                &song.song_title,
                fltk::enums::Align::Center,
                SongIdentifierType::LIBRARY,
                song.to_owned(),
                None,
                which_pq,
            );
            pack.add(&*si);
        }

        app::redraw();
    }

    pub fn make_queue_list_frames(play_queue_box: &mut Scroll, which_pq: usize) {
        // yes this is jank as fuck. No I don't care.
        SHARED_PLAY_QUEUE_GUI.write().unwrap().clear();

        let pl_ind = CURRENT_PLAYLIST_INDEX.read().unwrap().clone();
        let mut pack = Pack::default().with_size(500, 400); //_fill();
        pack.make_resizable(true);
        play_queue_box.add(&pack);

        let mut i: i32 = 0;
        for queued_song in PLAY_QUEUES.read().unwrap()[pl_ind].iter() {
            let song_iden = SongIdentifier::new(
                IN_PLAY_QUEUE_BOX_WIDTH,
                IN_PLAY_QUEUE_BOX_HEIGHT,
                &queued_song.song_title,
                fltk::enums::Align::Center,
                SongIdentifierType::PLAYQUEUE,
                queued_song.to_owned(),
                Some(i as usize),
                which_pq,
            );
            pack.add(&*song_iden);
            i += 1;
        }
        pack.end();
        play_queue_box.auto_layout();
        play_queue_box.scroll_to(-637, -40);
        SHARED_PLAY_QUEUE_GUI.write().unwrap().push(pack);
    }

    fn tab_close_cb(g: &mut impl GroupExt) {
        if app::callback_reason() == CallbackReason::Closed {
            let mut parent = g.parent().unwrap();
            parent.remove(g);
            app::redraw();
        }
    }

    pub fn append_song_to_queue(pq_song: PlayQueueSong, which_pq: usize) {
        let pl_ind = CURRENT_PLAYLIST_INDEX.read().unwrap().clone();

        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUES.read().unwrap()[pl_ind].len() - 1),
            which_pq,
        );
        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].add(&*song_iden);
        app::redraw();
    }
    pub fn insert_song_to_queue(pq_song: PlayQueueSong, current_index: usize, which_pq: usize) {
        let pl_ind = CURRENT_PLAYLIST_INDEX.read().unwrap().clone();
        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUES.read().unwrap()[pl_ind].len() - 1),
            which_pq,
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
