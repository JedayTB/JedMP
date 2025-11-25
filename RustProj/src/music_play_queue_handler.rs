pub mod play_queue_handler {
    use crate::play_queue_song::PlayQueueSong;

    use std::fs::File;
    use std::{io::BufReader, io::Lines, sync::RwLock};

    pub static PLAY_QUEUES: RwLock<Vec<PlayQueueSong>> = RwLock::new(Vec::new());
    pub static PLAY_QUEUE_INDEX: RwLock<usize> = RwLock::new(0usize);
    pub static MUSIC_LIBRARIES: RwLock<Vec<Vec<PlayQueueSong>>> = RwLock::new(Vec::new());

    pub fn create_playqueue(cached_file_lines: Lines<BufReader<File>>) {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        *pqi = 0;

        let cfl_vec: Vec<String> = cached_file_lines.collect::<Result<_, _>>().unwrap();

        let mut i: i32 = 0;

        let mut tempPQ: Vec<PlayQueueSong> = Vec::new();
        // Read necessary information

        for line in cfl_vec {
            let entry = line;

            let entries: Vec<&str> = entry.split("\x00").collect();

            let plq_song = PlayQueueSong::new(
                entries[0].to_owned(),
                entries[1].to_owned(),
                entries[2].to_owned(),
                entries[3].to_owned(),
                i as usize,
            );

            i += 1;
            tempPQ.push(plq_song);
        }
        PLAY_QUEUES.write().unwrap().clone_from(&tempPQ.clone());
    }
    pub fn open_music_lib(music_lib_lines: Lines<BufReader<File>>) {
        let cfl_vec: Vec<String> = music_lib_lines.collect::<Result<_, _>>().unwrap();

        let mut i: i32 = 0;

        let mut tempPQ: Vec<PlayQueueSong> = Vec::new();
        // Read necessary information

        for line in cfl_vec {
            let entry = line;

            let entries: Vec<&str> = entry.split("\x00").collect();

            let plq_song = PlayQueueSong::new(
                entries[0].to_owned(),
                entries[1].to_owned(),
                entries[2].to_owned(),
                entries[3].to_owned(),
                i as usize,
            );

            i += 1;
            tempPQ.push(plq_song);
        }
        MUSIC_LIBRARIES.write().unwrap().push(tempPQ.clone());
    }
    // Probably going to be temporary

    // perhaps not best to take in a copy of the struct.
    // But im not sure if the PLAY_QUEUE variable would be satisfied with a reference
    // design speaking as well, it's probably best the contents of PLAY_QUEUE aren't references
    // as well.

    // Must adjust the songs within the play_queue to match their index
    // This must be done for each song after an insert and removal.

    fn adjust_playqueue(adjust_after_index: i32) {
        let pq = &mut PLAY_QUEUES.write().unwrap();

        let play_queue_length = pq.len() as i32;

        let mut i = adjust_after_index;
        while i < play_queue_length {
            pq[i as usize].index_in_play_queue += 1;
            i += 1;
        }
    }

    pub fn insert_song_into_playqueue(pq_song: PlayQueueSong, index: usize) {
        PLAY_QUEUES.write().unwrap().insert(index, pq_song);

        adjust_playqueue(index as i32);
    }
    pub fn append_to_playqueue(pq_song: PlayQueueSong) {
        PLAY_QUEUES.write().unwrap().push(pq_song);
    }
    pub fn remove_from_playqueue(index: usize) {
        PLAY_QUEUES.write().unwrap().remove(index);
    }
    pub fn increment_play_queue_index() -> Option<usize> {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        let pq_len = PLAY_QUEUES.read().unwrap().len();
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
        PLAY_QUEUES.write().unwrap().remove(rm_ind);
    }
}
