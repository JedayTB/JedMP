pub mod play_queue_handler {
    use crate::play_queue_song::PlayQueueSong;

    use std::fs::File;
    use std::{io::BufReader, io::Lines, sync::RwLock};

    //TODO:
    // Will likely have to do major refactoring. Currently Library list variables are based on playqueu
    // Which makes many things a headache when adding multiple tabs / playlists
    // Do i reset playqueue everytime I load a new tab? Reset everytime tabs switch?
    // Better to make a separate vec for Libary and Playqueue.
    // (Store library vec inside corresponding Tab (playlist))

    pub static PLAY_QUEUES: RwLock<Vec<Vec<PlayQueueSong>>> = RwLock::new(Vec::new());
    pub static PLAY_QUEUE_INDEX: RwLock<usize> = RwLock::new(0usize);

    pub fn create_playqueue(cached_file_lines: Lines<BufReader<File>>, playqueues_index: usize) {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        *pqi = 0;

        let cfl_vec: Vec<String> = cached_file_lines.collect::<Result<_, _>>().unwrap();
        println!("----\t[Debug] Playqueue Vec creation benchmarking\t----");

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
            let len = tempPQ.len();
            println!("Tpq len {len}");
        }
        PLAY_QUEUES.write().unwrap().push(tempPQ.clone());
    }
    // Probably going to be temporary

    // perhaps not best to take in a copy of the struct.
    // But im not sure if the PLAY_QUEUE variable would be satisfied with a reference
    // design speaking as well, it's probably best the contents of PLAY_QUEUE aren't references
    // as well.

    // NOTE::
    // Must adjust the songs within the play_queue to match their index
    // This must be done for each song after an insert and removal.

    fn adjust_playqueue(adjust_after_index: i32, which_pq_to_adjust: usize) {
        let pq = &mut PLAY_QUEUES.write().unwrap()[which_pq_to_adjust];

        let play_queue_length = pq.len() as i32;

        let mut i = adjust_after_index;
        while i < play_queue_length {
            pq[i as usize].index_in_play_queue += 1;
            i += 1;
        }
    }

    pub fn insert_song_into_playqueue(
        pq_song: PlayQueueSong,
        index: usize,
        which_pq_adjust: usize,
    ) {
        PLAY_QUEUES.write().unwrap()[which_pq_adjust].insert(index, pq_song);

        adjust_playqueue(index as i32, which_pq_adjust);
    }
    pub fn append_to_playqueue(pq_song: PlayQueueSong, which_pq_adjust: usize) {
        PLAY_QUEUES.write().unwrap()[which_pq_adjust].push(pq_song);
    }
    pub fn remove_from_playqueue(index: usize, which_pq_adjust: usize) {
        PLAY_QUEUES.write().unwrap()[which_pq_adjust].remove(index);
    }
    pub fn increment_play_queue_index(which_pq_adjust: usize) -> Option<usize> {
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        let pq_len = PLAY_QUEUES.read().unwrap()[which_pq_adjust].len();
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

    pub fn remove_song_at_index(rm_ind: usize, which_pq_adjust: usize) {
        PLAY_QUEUES.write().unwrap()[which_pq_adjust].remove(rm_ind);
    }
}
