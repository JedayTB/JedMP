pub mod gui_controller {
    use crate::JButton::JButton::J_Button;
    use crate::Playlist_Tab;
    use crate::Playlist_Tab::Playlist_Tab::PlaylistTab;
    use crate::artist_frame::artist_frame::ArtistFrame;
    use crate::colors_handler::color_handler::COLOR_DICTIONARY;
    use crate::colors_handler::color_handler::JedMP_Colors;
    use crate::colors_handler::color_handler::get_jedmp_color;
    use crate::get_jedmp_musiccache_path;
    use crate::get_jedmp_playlist_dir;
    use crate::music_cache_handler::music_file_handler;
    use crate::music_play_queue_handler::play_queue_handler::{
        PLAY_QUEUE_INDEX, PLAY_QUEUES, decrement_play_queue_index, increment_play_queue_index,
    };
    use crate::play_queue_song::PlayQueueSong;
    use crate::playlist_handler::playlist_handler::get_playlists_names;
    use crate::song_identifier::{SongIdentifier, SongIdentifierType};
    use crate::tab_library::Tab_Library::TabLibrary;
    use fltk::dialog;
    use fltk::widget::Widget;
    use fltk::{app, enums::*, group::*, prelude::*, window::Window};

    use fltk_theme::{SchemeType, WidgetScheme};
    use rodio::{OutputStream, Sink};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc;
    use std::sync::RwLock;

    // Shit way of making a global (Because can't do runtime functions for Pack)
    static SHARED_PLAY_QUEUE_GUI: RwLock<Vec<Pack>> = RwLock::new(Vec::new());
    // Embrace the shit code. Another Global
    static SHARED_SINK: RwLock<Vec<Sink>> = RwLock::new(Vec::new());

    // These globals arent too bad.
    static CURRENT_PLAYLIST_INDEX: RwLock<usize> = RwLock::new(0);
    static PLAYLIST_COUNT: RwLock<usize> = RwLock::new(0);
    static RELOAD_SINK: RwLock<bool> = RwLock::new(true);

    static IN_PLAY_QUEUE_BOX_HEIGHT: i32 = 30;
    static IN_PLAY_QUEUE_BOX_WIDTH: i32 = 100;

    pub static BASE_WINDOW_WIDTH: i32 = 896;
    pub static BASE_WINDOW_HEIGHT: i32 = 504;
    pub static GENERAL_X_PAD: i32 = 10;
    pub static GENERAL_Y_PAD: i32 = 10;
    pub static MENU_ARTISTVIEW_PAD: i32 = 100;

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

        //FIXME:
        //Do something to make this user cofigurable
        let font = Font::load_font("/home/jeday/.fonts/JetBrainsMono-Medium.ttf").unwrap();
        Font::set_font(Font::Helvetica, &font);

        let menu_button_width = 30;
        let menu_button_height = 10;

        let mut menu_button = J_Button::new()
            .with_size(menu_button_width, menu_button_height)
            .with_label("Menu")
            .with_pos(0, 0);
        let button_box_height = BASE_WINDOW_HEIGHT / 12;
        let button_box_width = BASE_WINDOW_WIDTH;
        let button_box_pos_y = wind.h();
        let button_box_pos_x = BASE_WINDOW_WIDTH / 2;

        let button_box = Flex::default()
            .with_size(button_box_width - MENU_ARTISTVIEW_PAD, button_box_height)
            .with_pos(
                (button_box_pos_x - button_box_width / 2) + MENU_ARTISTVIEW_PAD,
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
            .with_pos(MENU_ARTISTVIEW_PAD, 0);

        row.set_color(get_jedmp_color(JedMP_Colors::Tabs_bg_color));
        let mut tabs = Tabs::default();
        tabs.handle_overflow(TabsOverflow::Compress);
        tabs.set_color(get_jedmp_color(JedMP_Colors::Background_color));

        //FIXME:
        // This will error later, when user closes a tab. IE
        // tab1  tab2 tab3
        // * closes tab 2 *
        // tab1 tab3
        // this logic will asign idx = 1 (element 2) when it finds tab3.
        // Possible fixes is removing tab2's playlists from PLAY_QUEUES and adjustinng indexes
        // accordingly. The other option is a custom tab implementation That returns PlaylistTab
        // instead of {impl GroupExt}
        tabs.handle(|tb, e: Event| match e {
            Event::Push => {
                let tabs_children = tb.children();
                let clicked_playlist = tb.push();
                let sel_pl: Widget;
                if clicked_playlist.is_none() {
                    //println!("No playlist pushed down on");
                } else {
                    sel_pl = clicked_playlist.unwrap().as_base_widget();
                    let mut i = 0;

                    while i < tabs_children {
                        let c = tb.child(i).expect("No widgets?");
                        let p = c.label();
                        println!("{p}");
                        if sel_pl.is_same(&c) {
                            //println!("clicked playlist was {p}, idx is {i}");
                            *RELOAD_SINK.write().unwrap() = true;
                            break;
                        }

                        i += 1;
                    }

                    *CURRENT_PLAYLIST_INDEX.write().unwrap() = i as usize;
                }
                true
            }
            _ => true,
        });

        let shared_tabs = rc::Rc::new(RefCell::new(tabs.clone()));

        let main_tab = PlaylistTab::new(get_jedmp_musiccache_path(), "All".to_owned(), 0);
        let mtab_lib = rc::Rc::new(RefCell::new(main_tab.library));

        tabs.end();
        tabs.auto_layout();

        row.end();

        let artist_view_box = Scroll::default()
            .with_size(MENU_ARTISTVIEW_PAD, BASE_WINDOW_HEIGHT - menu_button_height)
            .with_pos(0, 15);

        let mut art_pack = Pack::default_fill();

        let artist_frame_width = MENU_ARTISTVIEW_PAD;
        let artist_frame_height = 50;

        let mut art_hash: HashMap<String, ArtistFrame> = HashMap::new();

        let misc_artist_frame =
            ArtistFrame::new("Miscellaneous".to_owned(), 0 as usize, mtab_lib.clone())
                .with_size(artist_frame_width, artist_frame_height)
                .with_label("Miscellaneous");

        art_pack.add(&*misc_artist_frame);
        art_hash.insert("".to_owned(), misc_artist_frame);

        for s in PLAY_QUEUES.read().unwrap()[0].clone() {
            let artist_name = s._song_artists;

            if art_hash.contains_key(&artist_name) == false {
                let artist_frame =
                    ArtistFrame::new(artist_name.clone(), 0 as usize, mtab_lib.clone())
                        .with_size(artist_frame_width, artist_frame_height)
                        .with_label(&artist_name);

                art_pack.add(&*artist_frame);
                art_hash.insert(artist_name, artist_frame);
            }
        }

        art_pack.end();
        artist_view_box.end();
        wind.end();
        wind.make_resizable(true);
        wind.show();

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
            if SHARED_SINK.read().unwrap()[0].empty() || *RELOAD_SINK.read().unwrap() == true {
                let ind = PLAY_QUEUE_INDEX.read().unwrap();
                let cur_pl = CURRENT_PLAYLIST_INDEX.read().unwrap();
                let idx = *cur_pl;
                println!("Load song from {idx} playlist");
                let path = PLAY_QUEUES.read().unwrap()[*cur_pl][*ind].clone();
                let source = music_file_handler::load_path(&path.song_path);
                // Stops playback and clears all appened files

                SHARED_SINK.write().unwrap()[0].stop();
                SHARED_SINK.write().unwrap()[0].append(source);
                SHARED_SINK.write().unwrap()[0].play();
                *RELOAD_SINK.write().unwrap() = false;
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
        // sh_tab_list exists in attempt to populate the Scroll element after picking a music
        // directory
        let sh_lib_list = mtab_lib.clone();
        let sh_tab = shared_tabs.clone();
        menu_button.set_callback(move |mbut| {
            let mut cwind = Window::default()
                .with_pos(
                    mbut.x() + mbut.w() + mbut.label_size(),
                    mbut.y() + mbut.h() * 2,
                )
                .with_size(150, 100);
            let rcwin = rc::Rc::new(RefCell::new(cwind.clone()));
            cwind.set_border(false);
            let flex = Flex::default_fill().column();
            let mut add_mus_dir_but = J_Button::new().with_label("Add Music Directory");

            let sh_lib_inner = sh_lib_list.clone();

            let adm_rcwin = rcwin.clone();
            add_mus_dir_but.set_callback(move |_| {
                adm_rcwin.borrow_mut().hide();
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

            let apt_rc_win = rcwin.clone();
            add_playlist_tab.set_callback(move |_| {
                apt_rc_win.borrow_mut().hide();
                let mut wind = Window::default().with_size(500, 250);
                let r_wind = rc::Rc::new(RefCell::new(wind.clone()));
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

                    let sht = sh_tab_inner_inner.clone();
                    let rw = r_wind.clone();
                    temp_b.set_callback(move |_| {
                        let pl_name = playlist_name.clone();
                        let sh_tab_for = sht.clone();
                        let jedmpPLDir = get_jedmp_playlist_dir();
                        let plPath = format!("{jedmpPLDir}/{playlist_name}");
                        let tab = &mut *sh_tab_for.borrow_mut();

                        *PLAYLIST_COUNT.write().unwrap() += 1;
                        let pl_idx = *PLAYLIST_COUNT.read().unwrap();
                        let mut newPlTab = PlaylistTab::new(plPath, pl_name, pl_idx);
                        newPlTab.set_trigger(CallbackTrigger::Closed);
                        newPlTab.set_callback(tab_close_cb);
                        tab.add(&*newPlTab);
                        tab.auto_layout();

                        rw.borrow_mut().hide();
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

    pub fn make_library_list_frames(tablib: &mut TabLibrary, which_pq: usize) {
        tablib.scroll_pack.clear();
        let w = tablib.lib_scroll.w();
        let h = tablib.lib_scroll.h();
        let pl_ind = which_pq;
        tablib.scroll_pack.resize(0, 0, w, h);

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
            tablib.scroll_pack.add(&*si);
        }

        app::redraw();
    }

    pub fn make_queue_list_frames(play_queue_box: &mut Scroll, which_pq: usize) {
        // yes this is jank as fuck. No I don't care.
        SHARED_PLAY_QUEUE_GUI.write().unwrap().clear();

        let pl_ind = which_pq;
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
