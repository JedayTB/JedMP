pub mod music_file_handler {
    use std::fs::File;
    use std::fs::{self, OpenOptions};
    use std::time::SystemTime;

    use crate::play_queue_song::PlayQueueSong;
    use crate::{get_jedmp_musiccache_path, music_play_queue_handler};
    use rodio::Decoder;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;

    pub fn load_path(path_to_song: &String) -> Decoder<BufReader<File>> {
        let f = File::open(path_to_song);
        let file = BufReader::new(f.unwrap());
        let music_source = Decoder::new(file).expect("FILE WAS NOT: MP3, WAV, VORBIS OR FLAC.");
        return music_source;
    }

    pub fn process_chosen_song_directory(dir_path: &str) {
        let cached_songs_path = &get_jedmp_musiccache_path();

        let mut music_cache_file = OpenOptions::new()
            .append(true)
            .open(cached_songs_path)
            .expect("Couldn't open music_cache");

        println!("----\t[Master] Starting processing benchmark\t----");
        let startNanoTime = SystemTime::now();

        let mut pathb = PathBuf::new();
        pathb.push(dir_path);

        let mut music_cache: Vec<String> = Vec::new();
        let paths_in_master = pathb
            .read_dir()
            .expect("Couldn't unwrap chosen directory {dir_path}");

        for path in paths_in_master {
            pathb.clear();

            let pathstr = path
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
                .to_owned();

            pathb.push(&pathstr);
            // get the first sub dir
            // read sub dir, while pathb.is_dir is true, keep going until find non directory
            // add all directories to dir_to_search vec,
            if pathb.is_dir() {
                println!(
                    "[Master] [Encountered secondary directory {:?}: Scanning and caching]",
                    pathstr
                );

                scan_directory_to_cached_songs(&pathstr, &mut music_cache);
            } else if pathb.is_file() {
                //println!("[Master Dir] Writing {:?}", pathstr);
                // Check it's one of our supported song types
                process_song_to_vec(pathstr, &mut music_cache);
            }
        }

        let s = music_cache.concat();
        let sb = s.into_bytes();
        music_cache_file
            .write(&sb[..])
            .expect("Could't write into music_cache");

        music_cache_file
            .flush()
            .expect("Byte's couldn't reach music_cache");

        let elapsedTime = SystemTime::now()
            .duration_since(startNanoTime)
            .unwrap()
            .as_millis();
        println!("[Master] Finished Scanning for music.");
        println!("----\t[Debug] Benchmark for Music Directories Processing. Time:\t{elapsedTime}");
    }

    fn process_song_to_vec(pathstr: String, music_cache_vec: &mut Vec<String>) {
        let extension = pathstr.split(".").last().unwrap_or("").to_owned();
        //println!("(Found extension) {:?}", extension);

        if extension == "mp3" || extension == "flac" || extension == "wav" || extension == "opus" {
            let tF = taglib::File::new(&pathstr).expect("Coudln't open song file as taglib file");

            let album = tF
                .tag()
                .unwrap()
                .album()
                .unwrap_or("".to_owned())
                .to_owned();
            let artist = tF
                .tag()
                .unwrap()
                .artist()
                .unwrap_or("".to_owned())
                .to_owned();
            let mut title: String = tF
                .tag()
                .unwrap()
                .title()
                .unwrap_or("".to_owned())
                .to_owned();
            if title == "" {
                title = pathstr.split("/").last().unwrap().to_owned();
            }
            let s = format!("{pathstr}\x00{title}\x00{album}\x00{artist}\n");
            music_cache_vec.push(s);
        }
    }
    pub fn process_existing_song_to_string(song: PlayQueueSong) -> String {
        let path = song.song_path;
        let title = song.song_title;
        let album = song._song_album;
        let artist = song._song_artists;
        format!("{path}\x00{title}\x00{album}\x00{artist}\n")
    }

    fn scan_directory_to_cached_songs(dir_path: &str, music_cache: &mut Vec<String>) {
        let pathsindir = fs::read_dir(dir_path).unwrap();
        let mut pathBuf = PathBuf::new();
        for path in pathsindir {
            let song_path = path.unwrap().path().display().to_string();
            pathBuf.push(&song_path);

            if pathBuf.is_dir() {
                scan_directory_to_cached_songs(&song_path, music_cache);
            } else {
                process_song_to_vec(song_path, music_cache);
            }
        }
    }

    pub fn try_load_cached_music() {
        let cachedfiles_path_str = get_jedmp_musiccache_path();

        let cf_pathb = PathBuf::from(&cachedfiles_path_str);

        if cf_pathb
            .try_exists()
            .expect("Smth went wrong checking if path exist")
            == false
        {
            println!("Jed MP Folder does not exist. Creating and populating...");
            File::create(&cachedfiles_path_str).unwrap();
            println!("Created cachedmusic file");
        } else {
            println!("Cached Music Found, Loading library...");
            load_cached_songs();
        }
    }

    pub fn load_cached_songs() {
        let cached_songs_path = &get_jedmp_musiccache_path();
        let cached_music_file =
            File::open(cached_songs_path).expect("Couldn't read cached_songs file.");
        let c_metadata = cached_music_file.metadata().expect("File has no metadata?");
        let cached_music_file_length = c_metadata.len();

        if cached_music_file_length == 0 {
            println!("There's no cached music! Choose a directory to load.");
        }

        let buf_reader = BufReader::new(cached_music_file);
        let string_it = buf_reader.lines();
        // Only ever creates Full Libary tab playqueue
        music_play_queue_handler::play_queue_handler::create_playqueue(string_it, 0);
    }
}
