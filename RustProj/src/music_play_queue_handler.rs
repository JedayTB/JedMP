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

        let mut i: i32 = 0;
        // Read necessary information
        for line in cached_file_lines {
            let song_path = line.expect("Couldn't read path?");
            let song_title =
                song_file_metadata_handler::song_file_metadata_handler::get_song_title(&song_path);
            let plq_song = PlayQueueSong::new(song_path, song_title, i as usize);

            i += 1;
            PLAY_QUEUE.write().unwrap().push(plq_song);
        }
    }
    // perhaps not best to take in a copy of the struct.
    // But im not sure if the PLAY_QUEUE variable would be satisfied with a reference
    // design speaking as well, it's probably best the contents of PLAY_QUEUE aren't references
    // as well.

    // NOTE::
    // Must adjust the songs within the play_queue to match their index
    // This must be done for each song after an insert and removal.
    //

    fn adjust_playqueue(adjust_after_index: i32) {
        let mut pq = PLAY_QUEUE.write().unwrap();

        let play_queue_length = pq.len() as i32;

        let i = adjust_after_index;
        while i < play_queue_length {
            pq[i as usize].index_in_play_queue += 1;
        }
    }

    pub fn insert_song_into_playqueue(pq_song: PlayQueueSong, index: usize) {
        PLAY_QUEUE.write().unwrap().insert(index, pq_song);

        adjust_playqueue(index as i32);
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
    pub fn play_song_instant(new_pq_index: usize) {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        *pqi = new_pq_index;
    }

    pub fn remove_song_at_index(rm_ind: usize) {
        PLAY_QUEUE.write().unwrap().remove(rm_ind);
        adjust_playqueue(rm_ind as i32);
    }
}
