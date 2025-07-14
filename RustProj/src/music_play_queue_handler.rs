pub mod play_queue_handler {
    use crate::play_queue_song::PlayQueueSong;
    use crate::song_file_metadata_handler;

    use std::fs::File;
    use std::{io::BufReader, io::Lines, sync::RwLock};
    pub static PLAY_QUEUE: RwLock<Vec<PlayQueueSong>> = RwLock::new(Vec::new());
    pub fn create_playqueue(cached_file_lines: Lines<BufReader<File>>) {
        // Clear queue first
        PLAY_QUEUE.write().unwrap().clear();
        // Read necessary information
        for line in cached_file_lines {
            let song_path = line.expect("Couldn't read path?");
            let song_title =
                song_file_metadata_handler::song_file_metadata_handler::get_song_title(&song_path);
            let plq_song = PlayQueueSong::new(song_path, song_title);

            PLAY_QUEUE.write().unwrap().push(plq_song);
        }
    }
    // perhaps not best to take in a copy of the struct.
    // But im not sure if the PLAY_QUEUE variable would be satisfied with a reference
    // design speaking as well, it's probably best the contents of PLAY_QUEUE aren't references
    // as well.
    pub fn insert_song_into_playqueue(pq_song: PlayQueueSong, index: usize) {
        PLAY_QUEUE.write().unwrap().insert(index, pq_song);
    }
    pub fn append_to_playqueue(pq_song: PlayQueueSong) {
        PLAY_QUEUE.write().unwrap().push(pq_song);
    }
    pub fn remove_from_playqueue(index: usize) {
        PLAY_QUEUE.write().unwrap().remove(index);
    }
}
