pub mod music_file_handler {
    use std::collections::HashMap;
    use std::fs::File;
    use std::fs::{self, OpenOptions};
    use std::time::SystemTime;

    use vlc::{Instance, Media};

    use crate::get_jedmp_musiccache_path;
    use crate::play_queue_song::PlayQueueSong;
    use std::io::Write;
    use std::path::PathBuf;

    //TODO:
    //Implement a way to decrease music cache file size.
    //If the user's music is scattered across multiple directories, it's difficult
    //But if it's mainly in ~/Music, we can insert /home/username/Music to save 17 bytes every song
    //entry(If
    //username is 5 characters long. Like mine is)
    //usernname is "
    pub fn process_chosen_song_directory(dir_path: &str) {
        let cached_songs_path = &get_jedmp_musiccache_path();
        let libvlc_instance = Instance::new().expect("Couldn't start instance.");
        let mut music_cache_file = OpenOptions::new()
            .append(true)
            .open(cached_songs_path)
            .expect("Couldn't open music_cache");
        let SupportedFilesDict: HashMap<&str, bool> = HashMap::from([
            ("mp1", true),
            ("mp2", true),
            ("mp3", true),
            ("aac", true),
            ("m4a", true),
            ("m3u", true),
            ("wav", true),
            ("ogg", true),
            ("opus", true),
            ("ac3", true),
            ("eac3", true),
            ("mlp", true),
            ("thd", true),
            ("dts", true),
            ("wma", true),
            ("flac", true),
            ("alac", true),
            ("spx", true),
            ("mpc", true),
            ("aa3", true),
            ("oma", true),
            ("wv", true),
            ("mod", true),
            ("tta", true),
            ("ape", true),
            ("ra", true),
            ("alaw", true),
            ("ulaw", true),
            ("amr", true),
            ("mid", true),
            ("midi", true),
            ("lpcm", true),
            ("adpcm", true),
            ("qcp", true),
            ("dv", true),
            ("qdm", true),
            ("mace", true),
        ]);

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

                scan_directory_to_cached_songs(
                    &pathstr,
                    &mut music_cache,
                    &SupportedFilesDict,
                    &libvlc_instance,
                );
            } else if pathb.is_file() {
                //println!("[Master Dir] Writing {:?}", pathstr);
                // Check it's one of our supported song types
                process_song_to_vec(
                    pathstr,
                    &mut music_cache,
                    &SupportedFilesDict,
                    &libvlc_instance,
                );
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
        // Just to make sure.
        drop(libvlc_instance);
        let elapsedTime = SystemTime::now()
            .duration_since(startNanoTime)
            .unwrap()
            .as_millis();
        println!("[Master] Finished Scanning for music.");
        println!("----\t[Debug] Benchmark for Music Directories Processing. Time:\t{elapsedTime}");
    }
    //TODO:
    //Find out how to add compatibility for opus and m4a codecs
    fn process_song_to_vec(
        pathstr: String,
        music_cache_vec: &mut Vec<String>,
        supported_files: &HashMap<&str, bool>,
        libvlcIst: &Instance,
    ) {
        let extension = pathstr.split(".").last().unwrap_or("");
        //println!("(Found extension) {:?}", extension);

        match supported_files.get(extension) {
            Some(_) => {
                let media = Media::new_path(libvlcIst, &pathstr)
                    .expect("Couldn't open song file as taglib file");
                let album: String;
                let artist: String;
                let mut title: String;

                media.parse();
                album = media.get_meta(vlc::Meta::Album).unwrap_or("".to_string());
                artist = media.get_meta(vlc::Meta::Artist).unwrap_or("".to_string());
                title = media.get_meta(vlc::Meta::Title).unwrap_or("".to_string());

                if title == "" {
                    title = pathstr.split("/").last().unwrap().to_owned();
                }

                let s = format!("{pathstr}\x00{title}\x00{album}\x00{artist}\n");
                music_cache_vec.push(s);
            }
            None => {
                println!(
                    "[Debug] skipping {pathstr} - non acceptable file codec. Extension: {extension}"
                );
            }
        }
    }
    pub fn process_existing_song_to_string(song: PlayQueueSong) -> String {
        let path = song.song_path;
        let title = song.song_title;
        let album = song._song_album;
        let artist = song._song_artists;
        format!("{path}\x00{title}\x00{album}\x00{artist}\n")
    }

    fn scan_directory_to_cached_songs(
        dir_path: &str,
        music_cache: &mut Vec<String>,
        supported_files: &HashMap<&str, bool>,
        libvlcIst: &Instance,
    ) {
        let pathsindir = fs::read_dir(dir_path).unwrap();
        let mut pathBuf = PathBuf::new();
        for path in pathsindir {
            let song_path = path.unwrap().path().display().to_string();
            pathBuf.push(&song_path);

            if pathBuf.is_dir() {
                scan_directory_to_cached_songs(&song_path, music_cache, supported_files, libvlcIst);
            } else {
                process_song_to_vec(song_path, music_cache, supported_files, libvlcIst);
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
            println!("[Debug/music_cache_handler] JedMP music_cache does not exist. Creating...");
            File::create(&cachedfiles_path_str).unwrap();
            println!("[Debug/music_cache_handler] Created cachedmusic file");
        } else {
            println!("[Debug/music_cache_handler] Cached Music Found, Loading library...");
            load_cached_songs();
        }
    }
    ///This function is redundant
    pub fn load_cached_songs() {
        let cached_songs_path = &get_jedmp_musiccache_path();
        let cached_music_file =
            File::open(cached_songs_path).expect("Couldn't read cached_songs file.");
        let c_metadata = cached_music_file.metadata().expect("File has no metadata?");
        let cached_music_file_length = c_metadata.len();

        if cached_music_file_length == 0 {
            println!("There's no cached music! Choose a directory to load.");
        }

        // Only ever creates Full Libary tab playqueue
        // Also don't run this. No need to have playqueues on startup
        // let buf_reader = BufReader::new(cached_music_file);
        // let string_it = buf_reader.lines();
        // music_play_queue_handler::play_queue_handler::create_playqueue(string_it);
    }
}
