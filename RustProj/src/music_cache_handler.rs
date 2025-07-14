pub mod music_file_handler {
    // Use statements
    use std::fs::File;
    use std::fs::{self, OpenOptions};

    use rodio::Decoder;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;

    use crate::get_jedmp_musiccache_path;
    use crate::{get_jedmp_dir, music_play_queue_handler};
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
    pub fn try_load_cached_music() {
        let jedmp_directory = get_jedmp_dir();
        let pathb = PathBuf::from(&jedmp_directory);
        let mut _cachedfiles: File;
        let cachedfiles_path_str = format!("{jedmp_directory}/music_cache");

        let m = pathb.try_exists();
        let r = m.expect("Path Exists");
        if r == false {
            println!("Jed MP Folder does not exist. Creating and populating...");
            fs::create_dir(&jedmp_directory).expect("Jed MP Dir Created");

            // Do my logic here.
            _cachedfiles = File::create(&cachedfiles_path_str).unwrap();
            print!("Created cachedfiles.. file");
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
        music_play_queue_handler::play_queue_handler::create_playqueue(string_it);
    }
}
