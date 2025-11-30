pub mod gui_controller {
    use crate::audio_media_player::AudioMediaPlayer;
    use crate::audio_media_player::AudioMediaPlayer::MpMessage;
    use crate::colors_handler::color_handler::COLOR_DICTIONARY;
    use crate::colors_handler::color_handler::JedMP_Colors;
    use crate::colors_handler::color_handler::get_jedmp_color;
    use crate::discord_presence_handler::discord_presence::DRPC_SENDER;
    use crate::get_jedmp_musiccache_path;
    use crate::get_jedmp_playlist_dir;
    use crate::gui_resources::gui_resources::ArtistFrame;
    use crate::gui_resources::gui_resources::JButton;

    use crate::gui_resources::gui_resources::PlaylistTab;
    use crate::gui_resources::gui_resources::SongIdentifier;
    use crate::gui_resources::gui_resources::SongIdentifierType;
    use crate::gui_resources::gui_resources::TabLibrary;
    use crate::music_cache_handler::music_file_handler;
    use crate::music_play_queue_handler::play_queue_handler::MUSIC_LIBRARIES;
    use crate::music_play_queue_handler::play_queue_handler::{
        PLAY_QUEUE_INDEX, PLAY_QUEUES, decrement_play_queue_index, increment_play_queue_index,
    };
    use crate::play_queue_song::PlayQueueSong;
    use crate::playlist_handler::playlist_handler::get_playlists_names;
    use fltk::dialog;
    use fltk::frame::Frame;
    use fltk::widget::Widget;
    use fltk::{app, enums::*, group::*, prelude::*, window::Window};

    use fltk_theme::{SchemeType, WidgetScheme};
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc;
    use std::rc::Rc;
    use std::sync::RwLock;

    // Shit way of making a global (Because can't do runtime functions for Pack)
    // Yes, we're always accesing index [0]. Please tell me if theres a better way. Please.
    static SHARED_PLAY_QUEUE_GUI: RwLock<Vec<Pack>> = RwLock::new(Vec::new());
    // Embrace the shit code. Another Global
    static ARTIST_VIEW_SCROLL: RwLock<Vec<Scroll>> = RwLock::new(Vec::new());
    // Man..
    static CURRENT_PLAYING_SONG: RwLock<Vec<Frame>> = RwLock::new(Vec::new());

    // How this is MEANT to be used.
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

        let mut menu_button = JButton::new()
            .with_size(menu_button_width, menu_button_height)
            .with_label("Menu")
            .with_pos(0, 0);

        let button_box_height = BASE_WINDOW_HEIGHT / 18;
        let button_box_width = BASE_WINDOW_WIDTH;
        let button_box_pos_y = wind.h() - button_box_height - 5;
        let button_box_pos_x =
            (BASE_WINDOW_WIDTH / 2) - (button_box_width / 2) + MENU_ARTISTVIEW_PAD;

        let button_box = Flex::default()
            .with_size(button_box_width - MENU_ARTISTVIEW_PAD, button_box_height)
            .with_pos(button_box_pos_x, button_box_pos_y)
            .row();

        let mut last_song_button = JButton::new().with_label("<");
        let mut pause_song_button = JButton::new().with_label("Pause");
        let mut next_song_button = JButton::new().with_label(">");

        button_box.end();

        let mut song_info_box = Flex::default()
            .with_size(button_box_width, button_box_height)
            .with_pos(button_box_pos_x, button_box_pos_y - button_box_height - 5);

        let current_playing_song_frame = Frame::default().with_label("Current song here\t\t00:00");

        CURRENT_PLAYING_SONG
            .write()
            .unwrap()
            .push(current_playing_song_frame);

        song_info_box.set_frame(FrameType::FlatBox);
        song_info_box.end();
        let row_width = BASE_WINDOW_WIDTH - menu_button_width - 350;
        let row_height = BASE_WINDOW_HEIGHT - (BASE_WINDOW_HEIGHT / 6) - 10;

        let mut row = Flex::default()
            .with_size(row_width, row_height)
            .row()
            .with_pos(MENU_ARTISTVIEW_PAD, 0);

        row.set_color(get_jedmp_color(JedMP_Colors::Tabs_bg_color));

        let mut tabs = Tabs::default();
        tabs.handle_overflow(TabsOverflow::Compress);
        tabs.set_color(get_jedmp_color(JedMP_Colors::Background_color));

        let shared_tabs = rc::Rc::new(RefCell::new(tabs.clone()));

        let main_tab = PlaylistTab::new(get_jedmp_musiccache_path(), "All".to_owned(), 0, false);
        let mtab_lib = rc::Rc::new(RefCell::new(main_tab.library));

        tabs.end();
        tabs.auto_layout();

        row.end();

        let artist_view_scroll = Scroll::default()
            .with_size(MENU_ARTISTVIEW_PAD, BASE_WINDOW_HEIGHT - menu_button_height)
            .with_pos(0, 15);

        artist_view_scroll.end();

        ARTIST_VIEW_SCROLL.write().unwrap().push(artist_view_scroll);

        make_artist_view_frames(mtab_lib.clone(), 0 as usize);

        let play_queue_box_width = 250;
        let play_queue_box_height = 300;

        let mut main_playqueue = Scroll::default()
            .with_size(play_queue_box_width, play_queue_box_height)
            .with_pos(
                BASE_WINDOW_WIDTH - play_queue_box_width - GENERAL_X_PAD,
                GENERAL_Y_PAD * 2,
            );
        main_playqueue.set_color(get_jedmp_color(JedMP_Colors::Background_color));
        main_playqueue.set_frame(FrameType::UpBox);

        let mpq_pack = Pack::default_fill();

        mpq_pack.end();
        main_playqueue.add(&mpq_pack);
        main_playqueue.end();

        SHARED_PLAY_QUEUE_GUI.write().unwrap().push(mpq_pack);
        wind.end();
        wind.make_resizable(true);
        wind.show();

        //
        //  Create callbacks
        //
        //FIXME:
        // This will error later, when user closes a tab. IE
        // tab1  tab2 tab3
        // * closes tab 2 *
        // tab1 tab3
        // this logic will asign idx = 1 (element 2) when it finds tab3.
        // Possible fixes is removing tab2's library from MUSIC_LIBRARIES and adjusting indexes
        // accordingly. The other option is a custom tab implementation That returns PlaylistTab
        // instead of {impl GroupExt}
        // Will also need to resize and restructure the MUSIC_LIBRARIES vec. Though this is easily
        // doe with .remove(idx)
        tabs.handle(move |tb, e: Event| match e {
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
                    let art_view = &ARTIST_VIEW_SCROLL.write().unwrap()[0];
                    let mut k = 0;
                    while k < art_view.children() {
                        let mut c = art_view
                            .child(k)
                            .expect("No Children / Child doesn't exist");

                        if k != i {
                            c.hide();
                        } else {
                            c.show();
                        }
                        k += 1;
                    }

                    *CURRENT_PLAYLIST_INDEX.write().unwrap() = i as usize;
                }
                app::redraw();
                true
            }
            _ => true,
        });

        last_song_button.set_callback(move |_| {
            // Goes back a song. Replays song if already at 0th index
            let play_ind = decrement_play_queue_index().unwrap_or(0);
            let next_song = PLAY_QUEUES.read().unwrap()[play_ind].clone();
            update_song(next_song);
        });
        next_song_button.set_callback(move |_| {
            let play_ind = increment_play_queue_index();

            if play_ind == None {
                // Other logic here, check if replay playlist is on for example.
                // (Future feature)

                // We've reached end of play queue.
                AudioMediaPlayer::MessagePlayerThread(MpMessage::Pause, "".to_owned())
                    .expect("Couldn't send message");
            } else {
                let next_song = PLAY_QUEUES.read().unwrap()[play_ind.unwrap()].clone();
                update_song(next_song);
            }
        });

        pause_song_button.set_callback(move |btn| {
            //FIXME:
            //Breaks when playqueue is empty.
            if *RELOAD_SINK.read().unwrap() == true {
                let ind = PLAY_QUEUE_INDEX.read().unwrap();
                println!("pq idx: {ind}");
                let song = PLAY_QUEUES.read().unwrap()[*ind].clone();
                update_song(song);
                *RELOAD_SINK.write().unwrap() = false;
            }

            // Don't really like comparing strings, but I can't get the MusicPlayer paused value
            // across threads easy.
            if btn.label() == "Play" {
                AudioMediaPlayer::MessagePlayerThread(MpMessage::Play, "".to_owned())
                    .expect("Couldn't send message");
                btn.set_label("Pause");
            } else if btn.label() == "Pause" {
                // Else it's  paused.
                AudioMediaPlayer::MessagePlayerThread(MpMessage::Pause, "".to_owned())
                    .expect("Couldn't send message");
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
            let mut add_mus_dir_but = JButton::new().with_label("Add Music Directory");

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

            let mut add_playlist_tab = JButton::new().with_label("Open Playlist");

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
                    let mut temp_b = JButton::new().with_label(&playlist_name).with_size(450, 30);

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
                        let mut newPlTab = PlaylistTab::new(plPath, pl_name, pl_idx, true);
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

    fn update_song(pqs: PlayQueueSong) {
        let s_name = pqs.song_title.clone();
        let pq_idx = *PLAY_QUEUE_INDEX.read().unwrap();

        println!("[Debug/Gui_state_controller] Updated Sink song: {s_name}\tpq idx: {pq_idx}");

        // Stops playback and clears all appened files
        let data = pqs.song_path.clone();
        AudioMediaPlayer::MessagePlayerThread(MpMessage::SetAndPlay, data)
            .expect("Couldn't send message");

        let pqs_title = pqs.song_title.clone();
        let pqs_artist = pqs._song_artists.clone();
        let drpc_send = format!("{pqs_title} - {pqs_artist}");

        update_current_playing_song(pqs.song_title, "00:00".to_string());
        println!("[Debug] attempting to send drpc thread message");
        // Quick little match because I am NOT aborting the app just because drpc BS
        match DRPC_SENDER.write().unwrap()[0].send(drpc_send) {
            Ok(_) => {} // set properly
            Err(_) => {
                println!("[ERROR]\tCouldn't send to DRPC thread. Not running?")
            }
        }
    }
    fn tab_close_cb(g: &mut impl GroupExt) {
        if app::callback_reason() == CallbackReason::Closed {
            let mut parent = g.parent().unwrap();
            parent.remove(g);
            app::redraw();
        }
    }
    fn update_current_playing_song(s_name: String, s_time: String) {
        let set_s = format!("{s_name}\t\t{s_time}");
        CURRENT_PLAYING_SONG.write().unwrap()[0].set_label(&set_s);
    }

    pub fn append_song_to_queue(pq_song: PlayQueueSong) {
        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUES.read().unwrap().len() - 1),
        );
        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].add(&*song_iden);
        app::redraw();
    }
    pub fn insert_song_to_queue(pq_song: PlayQueueSong) {
        let current_index = *PLAY_QUEUE_INDEX.read().unwrap();

        let song_iden = SongIdentifier::new(
            IN_PLAY_QUEUE_BOX_WIDTH,
            IN_PLAY_QUEUE_BOX_HEIGHT,
            &pq_song.song_title,
            fltk::enums::Align::Center,
            SongIdentifierType::PLAYQUEUE,
            pq_song.to_owned(),
            Some(PLAY_QUEUES.read().unwrap().len() - 1),
        );

        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].insert(&*song_iden, current_index as i32);
        app::redraw();
    }

    pub fn sink_play_instant(pq_song: PlayQueueSong) {
        *PLAY_QUEUE_INDEX.write().unwrap() = pq_song
            .index_in_play_queue
            .clone()
            .expect("Possible expect of a musiclib PlayQueueSong");

        update_song(pq_song);
    }
    pub fn remove_song_from_playqueue(rm_index: usize) {
        SHARED_PLAY_QUEUE_GUI.write().unwrap()[0].remove_by_index(rm_index as i32);
    }
    pub fn make_library_list_frames(tablib: &mut TabLibrary, which_pq: usize) {
        tablib.scroll_pack.clear();
        let w = tablib.lib_scroll.w();
        let h = tablib.lib_scroll.h();
        let pl_ind = which_pq;
        tablib.scroll_pack.resize(0, 0, w, h);

        for song in MUSIC_LIBRARIES.read().unwrap()[pl_ind].iter() {
            let si = SongIdentifier::new(
                100,
                30,
                &song.song_title,
                fltk::enums::Align::Center,
                SongIdentifierType::LIBRARY,
                song.to_owned(),
                None,
            );
            tablib.scroll_pack.add(&*si);
        }

        app::redraw();
    }
    pub fn make_artist_view_frames(tablib_link: Rc<RefCell<TabLibrary>>, mus_lib_idx: usize) {
        let mut art_hash: HashMap<String, ArtistFrame> = HashMap::new();

        let artist_frame_width = MENU_ARTISTVIEW_PAD;
        let artist_frame_height = 50;

        let p_width = ARTIST_VIEW_SCROLL.read().unwrap()[0].w();
        let p_height = ARTIST_VIEW_SCROLL.read().unwrap()[0].h();

        let mut art_pack = Pack::default().with_size(p_width, p_height);
        art_pack.set_align(Align::Left);

        let misc_artist_frame = ArtistFrame::new("All".to_owned(), tablib_link.clone())
            .with_size(artist_frame_width, artist_frame_height)
            .with_label("All");

        art_pack.add(&*misc_artist_frame);
        art_hash.insert("".to_owned(), misc_artist_frame);

        for s in MUSIC_LIBRARIES.read().unwrap()[mus_lib_idx].clone() {
            let artist_name = s._song_artists;

            if art_hash.contains_key(&artist_name) == false {
                // For song's without artist metadata
                if artist_name == "".to_owned() {
                    let mut artist_frame =
                        ArtistFrame::new("Unnamed Artist".to_owned(), tablib_link.clone())
                            .with_size(artist_frame_width, artist_frame_height)
                            .with_label(&"Unnamed Artist");
                    artist_frame.set_align(Align::Left);
                    art_pack.add(&*artist_frame);
                    art_hash.insert("Unnamed Artist".to_owned(), artist_frame);
                } else {
                    let mut artist_frame =
                        ArtistFrame::new(artist_name.clone(), tablib_link.clone())
                            .with_size(artist_frame_width, artist_frame_height)
                            .with_label(&artist_name);

                    artist_frame.set_align(Align::Center);
                    art_pack.add(&*artist_frame);
                    art_hash.insert(artist_name, artist_frame);
                }
            }
        }
        art_pack.end();
        ARTIST_VIEW_SCROLL.write().unwrap()[0].add(&art_pack);
    }
}
