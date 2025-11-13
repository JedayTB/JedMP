pub mod playlist_window {

    use fltk::enums::Color;
    use fltk::group::{Flex, Pack, Scroll};

    use fltk::{enums::FrameType, prelude::*, *};

    use crate::JButton::JButton::J_Button;
    use crate::play_queue_song::PlayQueueSong;
    use crate::playlist_handler;
    use crate::playlist_handler::playlist_handler::{add_song_to_playlst, get_playlists_names};

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
}
