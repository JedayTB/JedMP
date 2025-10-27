pub mod play_queue_handler {
    use crate::play_queue_song::PlayQueueSong;
    use crate::song_file_metadata_handler;

    use std::fs::File;
    use std::thread::JoinHandle;
    use std::time::SystemTime;
    use std::{io::BufReader, io::Lines, sync::Arc, sync::RwLock, thread};

    pub static PLAY_QUEUE: RwLock<Vec<PlayQueueSong>> = RwLock::new(Vec::new());
    pub static PLAY_QUEUE_INDEX: RwLock<usize> = RwLock::new(0usize);

    const MINIMUM_SIZE_TO_MULTITHREAD: usize = 500;

    pub fn create_playqueue(cached_file_lines: Lines<BufReader<File>>) {
        // Clear queue first
        PLAY_QUEUE.write().unwrap().clear();

        // Reset play queue index
        let mut pqi = PLAY_QUEUE_INDEX.write().unwrap();
        *pqi = 0;

        let cfl_vec: Vec<String> = cached_file_lines.collect::<Result<_, _>>().unwrap();
        println!("----\t[Debug] Playqueue Vec creation benchmarking\t----");
        let startTime = SystemTime::now();
        let cfl_vec_len = cfl_vec.len();

        //let threads_to_spawn = cfl_vec_len;

        // No need to multithread relatively small Play queues

        //  Single thread version
        if cfl_vec_len < MINIMUM_SIZE_TO_MULTITHREAD {
            let mut i: i32 = 0;

            let mut tempPQ: Vec<PlayQueueSong> = Vec::new();
            // Read necessary information

            for line in cfl_vec {
                let song_path = line;
                let song_title =
                    song_file_metadata_handler::song_file_metadata_handler::get_song_title(
                        &song_path,
                    );
                let plq_song = PlayQueueSong::new(song_path, song_title, i as usize);

                i += 1;
                tempPQ.push(plq_song);
            }
            PLAY_QUEUE.write().unwrap().clone_from(&tempPQ);
            let elapsed = SystemTime::now()
                .duration_since(startTime)
                .unwrap()
                .as_millis();
            println!("----\t[Debug] Playqueue vec created with main core: Time:    {elapsed}");
        } else {
            // Multithreading!

            // Setup
            let arc_cfl: Arc<Vec<String>> = Arc::from(cfl_vec);
            let arc_cfl2: Arc<Vec<String>> = arc_cfl.clone();
            let half_size = cfl_vec_len / 2;

            let t1j = pqTFunc(arc_cfl, 0usize, half_size);
            let t2j = pqTFunc(arc_cfl2, half_size, cfl_vec_len);

            t1j.join().expect("Couldn't rejoin to main thread");
            t2j.join().expect("Coudln't rejoin to main thread");

            let elapsed = SystemTime::now()
                .duration_since(startTime)
                .unwrap()
                .as_millis();
            println!(
                "----\t[Debug] Playqeueu created with 2 threads\n\t\tTime Taken:\t{elapsed}\n\t\tSongs processed:\t{cfl_vec_len}"
            );
        }
    }
    // Probably going to be temporary
    fn pqTFunc(aPQ: Arc<Vec<String>>, start_ind: usize, end_ind: usize) -> JoinHandle<()> {
        let joinHandle: JoinHandle<_> = thread::spawn(move || {
            let mut i: usize = start_ind;
            let mut tempPQ: Vec<PlayQueueSong> = Vec::new();
            let cfl = aPQ.clone();
            while i < end_ind {
                let song_path = cfl[i].clone();
                let song_title =
                    song_file_metadata_handler::song_file_metadata_handler::get_song_title(
                        &song_path,
                    );

                //println!("[t2] pushed song {i} - {song_title}");
                let plq_song = PlayQueueSong::new(song_path.to_owned(), song_title, i as usize);

                i += 1;
                tempPQ.push(plq_song);
            }

            PLAY_QUEUE.write().unwrap().append(&mut tempPQ);
        });
        return joinHandle;
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
