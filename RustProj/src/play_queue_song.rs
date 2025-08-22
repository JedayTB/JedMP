// Refactor to 'Song' to avoid confusion.

#[derive(Clone, Debug)]
pub struct PlayQueueSong {
    pub song_path: String,
    pub song_title: String,
    pub _song_artists: String,
    pub _song_image_path: String,
    pub index_in_play_queue: usize,
}

impl PlayQueueSong {
    pub fn new(song_path: String, song_title: String, index_in_pq: usize) -> PlayQueueSong {
        let song_path = song_path;
        let song_title = song_title;
        let _song_artists = "Not implemented yet".to_owned();
        let _song_image_path = "Not implemented yet.".to_owned();
        let index_in_play_queue: usize = index_in_pq;

        PlayQueueSong {
            song_path,
            song_title,
            _song_artists,
            _song_image_path,
            index_in_play_queue,
        }
    }
}
