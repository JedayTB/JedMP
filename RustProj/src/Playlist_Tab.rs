#![allow(dead_code)]
pub mod Playlist_Tab {

    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    use fltk::group::{Group, Scroll};

    use crate::colors_handler::color_handler::COLOR_DICTIONARY;
    use crate::colors_handler::color_handler::JedMP_Colors;
    use crate::colors_handler::color_handler::get_jedmp_color;
    use crate::gui_state_controller::gui_controller::{
        BASE_WINDOW_HEIGHT, BASE_WINDOW_WIDTH, GENERAL_X_PAD, GENERAL_Y_PAD, MENU_ARTISTVIEW_PAD,
        make_library_list_frames, make_queue_list_frames,
    };
    use crate::music_play_queue_handler::play_queue_handler;
    use crate::tab_library::Tab_Library::TabLibrary;

    use fltk::{enums::FrameType, prelude::*, *};

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
}
