#![allow(unused_must_use)]
pub mod play_queue_handler {
    use crate::play_queue_song::PlayQueueSong;
    use crate::song_file_metadata_handler;

    use std::fs::File;
    use std::{io::BufReader, io::Lines, sync::RwLock};

    pub static PLAY_QUEUE: RwLock<Vec<PlayQueueSong>> = RwLock::new(Vec::new());
    pub static PLAY_QUEUE_INDEX: RwLock<usize> = RwLock::new(0usize);

    pub fn create_playqueue(cached_file_lines: Lines<BufReader<File>>) {
        // Clear queue first
        PLAY_QUEUE.write().unwrap().clear();

        // Reset play queue index
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        *pqi = 0;

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

        dbg!(PLAY_QUEUE.read().unwrap());
    }
    pub fn append_to_playqueue(pq_song: PlayQueueSong) {
        PLAY_QUEUE.write().unwrap().push(pq_song);
    }
    pub fn remove_from_playqueue(index: usize) {
        PLAY_QUEUE.write().unwrap().remove(index);
    }
    pub fn increment_play_queue_index() -> Option<usize> {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        let pq_len = PLAY_QUEUE.read().unwrap().len();
        let inc_ind = pqi.checked_add(1).unwrap_or_default();

        if inc_ind > pq_len {
            return None;
        } else {
            *pqi = inc_ind;
            return Some(inc_ind);
        }
    }
    pub fn decrement_play_queue_index() -> Option<usize> {
        let pqi = PLAY_QUEUE_INDEX.write().unwrap();
        let dec_ind = pqi.checked_sub(1);
        if dec_ind != None {
            return Some(dec_ind.unwrap());
        } else {
            return None;
        }
    }
}
