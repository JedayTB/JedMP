pub mod artist_frame {
    use fltk::frame::Frame;
    use fltk::widget_extends;
    use fltk::{enums::*, prelude::*};

    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::music_play_queue_handler::play_queue_handler::PLAY_QUEUES;
    use crate::tab_library::Tab_Library::TabLibrary;

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
}
