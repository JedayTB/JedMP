pub mod playlist_handler {

    use std::{
        fs::{self, File, OpenOptions},
        io::Write,
        path::PathBuf,
    };

    use crate::{
        get_jedmp_playlist_dir,
        music_cache_handler::music_file_handler::process_existing_song_to_string,
        play_queue_song::PlayQueueSong,
    };

    pub fn try_create_playlist_dir() {
        let jedmp_playlist_dir_path = get_jedmp_playlist_dir();
        let PathB = PathBuf::from(jedmp_playlist_dir_path);

        if PathB.exists() == false {
            fs::create_dir(PathB).expect("Couldn't create directory");
        }
    }
    pub fn get_playlists_names() -> Vec<String> {
        let jpl_dir = get_jedmp_playlist_dir();
        let pathb = PathBuf::from(jpl_dir);

        let mut dirscan: Vec<String> = Vec::new();
        let filesindir = pathb.read_dir().expect("Could't read {jpl_dir}");

        for path in filesindir {
            // jedmpdirs will always be found in ~/.jedmp So there's no reason to return the path.
            // Just filename
            let filename = path.unwrap().file_name().into_string().unwrap();
            dirscan.push(filename);
        }

        return dirscan;
    }
    pub fn create_playlist(playlist_name: String) {
        let pldir = get_jedmp_playlist_dir();
        let pl_path = format!("{pldir}/{playlist_name}");
        File::create(pl_path).expect("Couldn't create playlist");
    }

    pub fn add_song_to_playlst(playlist_name: &String, song: PlayQueueSong) {
        let pdir = get_jedmp_playlist_dir();
        let sel_pl_path = format!("{pdir}/{playlist_name}");
        let mut mf = OpenOptions::new()
            .append(true)
            .open(sel_pl_path)
            .expect("Couldn't open playlist");

        write!(mf, "{}", process_existing_song_to_string(song)).expect("Failed to write to file");
    }

    pub fn add_multiple_songs_to_playlist(
        playlist_name: &String,
        pq_songs_to_add: Vec<PlayQueueSong>,
    ) {
        let pdir = get_jedmp_playlist_dir();
        let sel_pl_path = format!("{pdir}/{playlist_name}");
        let mut mf = OpenOptions::new()
            .append(true)
            .open(sel_pl_path)
            .expect("Couldn't open playlist");

        let writestr = process_multiple_songs_to_string(pq_songs_to_add);

        write!(mf, "{}", writestr).expect("Failed to write to file");
    }
    fn process_multiple_songs_to_string(songs: Vec<PlayQueueSong>) -> String {
        let mut ret_string: String = "".to_owned();
        for s in songs {
            ret_string.push_str(&process_existing_song_to_string(s));
        }
        return ret_string;
    }
}
