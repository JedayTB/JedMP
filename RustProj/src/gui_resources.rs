pub mod gui_resources {

    use crate::{
        gui_state_controller, music_play_queue_handler::play_queue_handler::PLAY_QUEUE_INDEX,
    };
    use fltk::{button::Button, enums::Color, enums::Event, prelude::*, *};
    use fltk::{enums::Align, text::*};

    use fltk::group::{Flex, Pack};
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    use crate::play_queue_song::PlayQueueSong;
    use crate::playlist_handler;
    use crate::playlist_handler::playlist_handler::{add_song_to_playlst, get_playlists_names};
    use fltk::frame::Frame;
    use fltk::group::{Group, Scroll};
    use fltk::widget_extends;

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::colors_handler::color_handler::COLOR_DICTIONARY;
    use crate::colors_handler::color_handler::JedMP_Colors;
    use crate::colors_handler::color_handler::get_jedmp_color;
    use crate::gui_state_controller::gui_controller::{
        BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH, GENERAL_X_PAD, GENERAL_Y_PAD, MENU_ARTISTVIEW_PAD,
        make_library_list_frames, make_queue_list_frames,
    };
    use crate::music_play_queue_handler::play_queue_handler;
    use crate::music_play_queue_handler::play_queue_handler::PLAY_QUEUES;

    use fltk::enums::FrameType;

    pub struct PlaylistTab {
        tab_group: Group,
        pub library: TabLibrary,
        play_queue: Scroll,
        path_to_playlist: String,
        playlist_index: usize,
    }

    widget_extends!(PlaylistTab, group::Group, tab_group);
    impl PlaylistTab {
        pub fn new(path_to_playlist: String, playlist_name: String, pl_index: usize) -> Self {
            let playlist_file =
                File::open(&path_to_playlist).expect("Couldn't read cached_songs file.");

            let bufR = BufReader::new(playlist_file);
            let lines = bufR.lines();

            play_queue_handler::create_playqueue(lines);

            let mut tab_group = Group::default()
                .with_label(&playlist_name)
                .with_size(BASE_WINDOW_WIDTH, BASE_WINDOW_HEIGHT);

            tab_group.set_color(get_jedmp_color(JedMP_Colors::Background_color));
            let library_list_width = 500;
            let library_list_height = 300;

            let library_list_pos_x = GENERAL_X_PAD;
            let library_list_pos_y = 0;

            let mut library = TabLibrary::new(pl_index)
                .with_size(library_list_width, library_list_height)
                .with_pos(library_list_pos_x, library_list_pos_y + GENERAL_Y_PAD);

            //library_list.set_type(fltk::group::ScrollType::Vertical);

            library.set_frame(FrameType::GtkDownFrame);
            library.set_color(
                COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Background_color as usize],
            );
            library.end();
            //let shared_library_list = rc::Rc::new(RefCell::new(library_list.clone()));

            let play_queue_box_width = 250;
            let play_queue_box_height = 300;

            let mut play_queue = Scroll::default()
                .with_size(play_queue_box_width, play_queue_box_height)
                .with_pos(
                    (BASE_WINDOW_WIDTH - MENU_ARTISTVIEW_PAD) - play_queue_box_width,
                    library_list_pos_y + GENERAL_Y_PAD,
                );

            //play_queue_box.set_type(fltk::group::ScrollType::Vertical);
            //play_queue_box.set_frame(FrameType::PlasticDownBox);

            play_queue.set_color(get_jedmp_color(JedMP_Colors::Background_color));
            play_queue.end();
            // GUI state variables creation
            make_library_list_frames(&mut library, pl_index);
            make_queue_list_frames(&mut play_queue, pl_index);

            let playlist_index = pl_index;
            Self {
                tab_group,
                library,
                play_queue,
                path_to_playlist,
                playlist_index,
            }
        }
    }

    //
    //
    //
    pub struct ArtistFrame {
        artist_frame: Frame,
        tab_belong_idx: usize,
    }

    widget_extends!(ArtistFrame, Frame, artist_frame);

    impl ArtistFrame {
        pub fn new(
            artist_frame_name: String,
            tab_belong_idx: usize,
            library_in_tab_frame_rfc: Rc<RefCell<TabLibrary>>,
        ) -> Self {
            let mut artist_frame = Frame::default();

            artist_frame.handle(move |f, e| match e {
                Event::Push => {
                    let n = f.label();
                    //println!("[Debug] {n} pushed");
                    let lbf = library_in_tab_frame_rfc.borrow_mut();
                    let lb_pack = &lbf.scroll_pack;
                    let pq_idx = lbf.pq_idx_belongs_to;
                    let mut i = 0;
                    //FIXME:
                    // Need library lists instead of play_queues. For now, this'll do
                    // just don't change the playqueue from default!

                    // Don't process for misc frame

                    //if artist_frame_name != "Miscellaneous".to_owned() {
                    for s in &PLAY_QUEUES.read().unwrap()[pq_idx] {
                        let s_frame_artist = s._song_artists.clone();

                        if artist_frame_name != s_frame_artist {
                            let mut c = lb_pack.child(i).expect("Expected child");
                            c.hide();
                        } else if artist_frame_name == s_frame_artist {
                            let mut c = lb_pack.child(i).expect("Expected child");
                            c.show();
                        }
                        i += 1;
                    }
                    //}

                    true
                }

                _ => true,
            });
            ArtistFrame {
                artist_frame,
                tab_belong_idx,
            }
        }
    }

    pub struct J_Button {
        pub but: Button,
    }
    widget_extends!(J_Button, Button, but);
    impl J_Button {
        pub fn new() -> Self {
            let mut but = Button::default();
            but.set_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Button_bg_color as usize]);
            but.set_label_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Text_color as usize]);
            but.set_selection_color(
                COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Button_hover_color as usize],
            );

            but.handle(move |_button, ev: Event| match ev {
                Event::Enter => {
                    // Functioning weird.
                    //button.set_color(get_jedmp_color(JedMP_Colors::Important_text_color));
                    true
                }

                Event::Leave => {
                    //button.set_color(get_jedmp_color(JedMP_Colors::Button_bg_color));
                    true
                }
                _ => false,
            });
            Self { but }
        }

        pub fn ColorOverrides(&mut self, bg: Color, text: Color, pressed: Color, hover: Color) {
            self.set_color(bg);
            self.set_label_color(text);
            self.set_selection_color(pressed);

            self.handle(move |button, ev: Event| match ev {
                Event::Enter => {
                    button.set_color(hover.to_owned());
                    true
                }
                Event::Leave => {
                    button.set_color(bg.to_owned());
                    true
                }
                _ => false,
            });
        }
    }

    pub struct PlaylistWindow {
        wind: window::Window,
    }
    widget_extends!(PlaylistWindow, window::Window, wind);

    impl PlaylistWindow {
        pub fn new(song: PlayQueueSong) -> Self {
            let mut wind = window::Window::default().with_size(500, 250);
            wind.set_border(false);
            wind.set_color(Color::from_rgb(100, 100, 100));

            let playlists = get_playlists_names();
            let mut playlist_picker: Scroll = Scroll::default().with_size(500, 200);

            let mut pl_pack = Pack::default_fill();
            pl_pack.make_resizable(true);
            playlist_picker.add(&pl_pack);

            for pls in playlists {
                let mut temp_b = J_Button::new().with_label(&pls).with_size(450, 30);
                let tsp = song.to_owned();
                temp_b.set_callback(move |_| {
                    add_song_to_playlst(&pls, tsp.to_owned());
                });
                pl_pack.add(&*temp_b);
            }
            pl_pack.end();

            playlist_picker.end();

            let but_pack = Flex::default()
                .with_size(500, 50)
                .below_of(&playlist_picker, 3)
                .row();

            let mut create_pl_but = J_Button::new().with_label("Create new");

            let mut cancel_but = J_Button::new().with_label("Close");

            create_pl_but.set_callback(|_b| {
                let _cpd = CreatePlaylistDialog::new();
            });

            cancel_but.set_callback({
                let mut win = wind.clone();
                move |_| {
                    win.hide();
                }
            });

            but_pack.layout();
            but_pack.end();
            wind.make_modal(true);
            wind.end();
            wind.show();
            Self { wind }
        }
    }

    //TODO:
    //Create new colors for this

    struct CreatePlaylistDialog {}
    impl CreatePlaylistDialog {
        pub fn new() -> Self {
            let mx = app::event_x_root();
            let my = app::event_y_root();

            let mut win = window::Window::default()
                .with_size(400, 100)
                .with_pos(mx, my);

            win.set_border(false);
            win.set_color(Color::from_rgb(240, 240, 240));

            let mut pack = group::Pack::default()
                .with_size(300, 30)
                .center_of_parent()
                .with_type(group::PackType::Horizontal);

            pack.set_spacing(20);

            frame::Frame::default()
                .with_size(80, 0)
                .with_label("Enter Playlist Name:");

            let mut inp = input::Input::default().with_size(100, 0);
            inp.set_frame(FrameType::FlatBox);

            let mut ok = J_Button::new().with_size(80, 0).with_label("Ok");
            pack.end();
            win.end();
            win.make_modal(true);
            win.show();
            ok.set_callback({
                let mut win = win.clone();
                move |_| {
                    playlist_handler::playlist_handler::create_playlist(inp.value());
                    win.hide();
                }
            });
            Self {}
        }
    }

    pub const LIBRARY_OPTIONS: &'static str = "Add To Queue,Insert Next,Add To Playlist";
    pub const PLAYQUEUE_OPTIONS: &'static str = "Remove This,Play Now,Stop after";

    pub enum SongIdentifierType {
        LIBRARY,
        PLAYQUEUE,
    }

    widget_extends!(PopupWindow, window::Window, win);
    pub struct PopupWindow {
        win: window::Window,
    }
    impl PopupWindow {
        pub fn new(
            pwin_type: &SongIdentifierType,
            song: PlayQueueSong,
            _index: Option<usize>,
            pq_belongs_to: usize,
        ) -> Self {
            let mut win = window::Window::default();

            let mut pack = group::Pack::new(1, 1, win.w() - 2, win.h() - 2, None);
            win.set_border(false);

            let mut _choices: Vec<&str> = Vec::new();

            // Kind of ugly. but whatever.
            match pwin_type {
                SongIdentifierType::LIBRARY => {
                    _choices = LIBRARY_OPTIONS.split(",").collect();
                    let mut add_queue_but = J_Button::new()
                        .with_label(_choices[0])
                        .with_size(_choices[0].len() as i32 * 10, 25);
                    let mut insert_next_but = J_Button::new()
                        .with_label(_choices[1])
                        .with_size(_choices[1].len() as i32 * 10, 25);
                    let mut add_to_playlist = J_Button::new()
                        .with_label(_choices[2])
                        .with_size(_choices.len() as i32 * 10, 25);

                    let song_: Rc<RefCell<PlayQueueSong>> = Rc::new(RefCell::new(song));
                    let song__ = Rc::clone(&song_);
                    let song___ = Rc::clone(&song_);

                    add_queue_but.set_callback(move |_| {
                        println!("Appended to pq");

                        play_queue_handler::append_to_playqueue(
                            song_.borrow().clone(),
                            pq_belongs_to,
                        );

                        gui_state_controller::gui_controller::append_song_to_queue(
                            song_.borrow().clone(),
                            pq_belongs_to,
                        );
                    });
                    insert_next_but.set_callback(move |_| {
                        println!("Inserted in pq");
                        let current_index = PLAY_QUEUE_INDEX.read().unwrap();
                        play_queue_handler::insert_song_into_playqueue(
                            song__.borrow().clone(),
                            *current_index,
                            pq_belongs_to,
                        );
                        gui_state_controller::gui_controller::insert_song_to_queue(
                            song__.borrow().clone(),
                            current_index.clone(),
                            pq_belongs_to,
                        );
                    });
                    add_to_playlist.set_callback(move |_| {
                        // Check if any playlists exist
                        // If none prompt
                        let mx = app::event_x_root();
                        let my = app::event_y_root();
                        let _playlist_popup =
                            PlaylistWindow::new(song___.borrow().clone()).with_pos(mx, my);
                    });
                }
                SongIdentifierType::PLAYQUEUE => {
                    let song_: Rc<RefCell<PlayQueueSong>> = Rc::new(RefCell::new(song));
                    let song__ = Rc::clone(&song_);

                    _choices = PLAYQUEUE_OPTIONS.split(",").collect();

                    let mut remove_this_but = J_Button::new()
                        .with_label(_choices[0])
                        .with_size(_choices[0].len() as i32 * 10, 25);

                    let mut play_now_but = J_Button::new()
                        .with_label(_choices[1])
                        .with_size(_choices[1].len() as i32 * 10, 25);

                    let mut stop_after_but = J_Button::new()
                        .with_label(_choices[2])
                        .with_size(_choices[2].len() as i32 * 10, 25);

                    remove_this_but.set_callback(move |_| {
                        play_queue_handler::remove_song_at_index(
                            song_.borrow().clone().index_in_play_queue,
                            pq_belongs_to,
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

    // Song link IS used.. Stupid.

    // Refactor to SongDisplayer

    // Having PlayQueueSong is bad. But because Library views haven't had much work, its somewhat
    // necessary.
    // Add to group by using group.add(&*SongIdentifier), because it needs to be dereferenced
    #[derive(Debug)]
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
            pq_belongs_to: usize,
        ) -> SongIdentifier {
            let mut group = Flex::default().with_size(w, h);

            let mut _song_name_text = text::TextDisplay::default().center_of(&group);
            let mut txt_buffer = TextBuffer::default();
            txt_buffer.set_text(song_name);

            _song_name_text.super_handle_first(false);
            _song_name_text.set_buffer(txt_buffer);
            _song_name_text.set_align(alignment);
            _song_name_text.set_frame(enums::FrameType::FlatBox);

            _song_name_text.set_color(get_jedmp_color(JedMP_Colors::Song_iden_bg_color));
            // Set song text colour based on which type (library or playqueu)
            match iden_type {
                SongIdentifierType::LIBRARY => {
                    _song_name_text
                        .set_text_color(get_jedmp_color(JedMP_Colors::Libary_Song_text_color));
                }
                SongIdentifierType::PLAYQUEUE => {
                    _song_name_text
                        .set_text_color(get_jedmp_color(JedMP_Colors::Playqueue_Song_text_color));
                }
            }
            group.set_align(alignment);

            let song_clone = song_link.clone();
            group.super_handle_first(false);

            group.handle(move |_widg, event| match event {
                Event::Push => {
                    if app::event_mouse_button() == app::MouseButton::Right {
                        let mx = app::event_x_root();
                        let my = app::event_y_root();
                        let _popwin = PopupWindow::new(
                            &iden_type,
                            song_clone.clone(),
                            index_in_list,
                            pq_belongs_to,
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

    pub struct TabLibrary {
        pub lib_scroll: Scroll,
        pub scroll_pack: Pack,
        pub pq_idx_belongs_to: usize,
    }
    widget_extends!(TabLibrary, Scroll, lib_scroll);
    impl TabLibrary {
        pub fn new(pq_idx: usize) -> Self {
            let lib_scroll = Scroll::default();
            let pq_idx_belongs_to = pq_idx;
            let scroll_pack = Pack::default();

            TabLibrary {
                lib_scroll,
                scroll_pack,
                pq_idx_belongs_to,
            }
        }
    }
}
