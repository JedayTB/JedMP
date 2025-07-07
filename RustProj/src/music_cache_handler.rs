pub mod music_file_handler {
    // Use statements
    use std::fs::File;
    use std::fs::{self, OpenOptions};

    use rodio::Decoder;
    use std::cell::RefCell;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;

    use std::rc::Rc;

    use crate::get_jedmp_dir;
    use crate::get_jedmp_musiccache_path;
    use crate::song_file_metadata_handler::*;
    pub fn load_path(path_to_song: &String) -> Decoder<BufReader<File>> {
        let f = File::open(path_to_song);
        let file = BufReader::new(f.unwrap());
        let music_source = Decoder::new(file).expect("FILE WAS NOT: MP3, WAV, VORBIS OR FLAC.");
        return music_source;
    }
    pub fn process_chosen_song_directory(dir_path: &str) {
        let cached_songs_path = &get_jedmp_musiccache_path();

        let pathsindir = fs::read_dir(dir_path).unwrap();
        let mut music_cache_file = OpenOptions::new()
            .append(true)
            .open(cached_songs_path)
            .expect("Couldn't open music_cache");

        for path in pathsindir {
            let mut pathb = PathBuf::new();
            let pathstr = path.unwrap().path().display().to_string();
            pathb.push(&pathstr);

            if pathb.is_dir() {
                println!(
                    "Encountered secondary directory {:?}: Scanning and caching",
                    pathstr
                );
                scan_directory_to_cached_songs(&pathstr, cached_songs_path);
            } else if pathb.is_file() {
                println!("Writing {:?}", pathstr);
                //let song_path_str = format!("{}\n", pathstr);
                writeln!(music_cache_file, "{}", pathstr).expect("Write failed.");
            }
        }
    }
    fn scan_directory_to_cached_songs(dir_path: &str, cached_songs_path: &str) {
        let pathsindir = fs::read_dir(dir_path).unwrap();
        for path in pathsindir {
            let song_path = path.unwrap().path().display().to_string();

            println!("Writing {:?}", song_path);
            fs::write(cached_songs_path, song_path).expect("Couldn't write.");
        }
    }
    pub fn try_load_cached_music() -> Result<Rc<RefCell<Vec<String>>>, &'static str> {
        let jedmp_directory = get_jedmp_dir();
        let pathb = PathBuf::from(&jedmp_directory);
        let mut _cachedfiles: File;
        let cachedfiles_path_str = format!("{jedmp_directory}/music_cache");

        let m = pathb.try_exists();
        let r = m.expect("Path Exists");
        let mut loadedcachedsongs: bool = false;
        let play_queue: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::default());
        if r == false {
            println!("Jed MP Folder does not exist. Creating and populating...");
            fs::create_dir(&jedmp_directory).expect("Jed MP Dir Created");

            // Do my logic here.
            _cachedfiles = File::create(&cachedfiles_path_str).unwrap();
            print!("Created cachedfiles.. file");
        } else {
            println!("Cached Music Found, Loading values...");
            // Load Cached values..
            println!("Loading music..");

            *play_queue.borrow_mut() = load_cached_songs();
            loadedcachedsongs = true;
        }
        if loadedcachedsongs {
            Ok(play_queue)
        } else {
            Err("No CachedMusicFile found, not returning play queue")
        }
    }

    // TODO:
    // Change this function to return a Result with String Vec
    pub fn load_cached_songs() -> Vec<String> {
        let cached_songs_path = &get_jedmp_musiccache_path();
        let mut queue_list: Vec<String> = Vec::new();
        let cached_music_file =
            File::open(cached_songs_path).expect("Couldn't read cached_songs file.");
        let c_metadata = cached_music_file.metadata().expect("File has no metadata?");
        let cached_music_file_length = c_metadata.len();

        if cached_music_file_length == 0 {
            println!("There's no cached music! Choose a directory to load.");
        }
        let buf_reader = BufReader::new(cached_music_file);
        let string_it = buf_reader.lines();

        for lines in string_it {
            let song_path = lines.expect("Couldn't read song paths.");
            let song_name = song_file_metadata_handler::get_song_title(&song_path);
            println!("Song found: name {:?}", song_name);
            queue_list.push(song_name);
        }

        // Rust why?
        return queue_list;
    }
}
