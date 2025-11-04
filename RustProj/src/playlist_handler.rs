pub mod playlist_handler {

    use std::{
        fs::{self, File},
        path::PathBuf,
    };

    use crate::{get_jedmp_musiccache_path, get_jedmp_playlist_dir};
    pub struct Playlist {}

    pub fn try_create_playlist_dir() {
        let jedmp_playlist_dir_path = get_jedmp_playlist_dir();
        let PathB = PathBuf::from(jedmp_playlist_dir_path);

        if PathB.exists() == false {
            fs::create_dir(PathB).expect("Couldn't create directory");
        }
    }
    pub fn create_playlist(playlist_name: String) {
        File::create(playlist_name).expect("Couldn't create playlist");
    }
    pub fn add_to_playlist(playlist_name: String, pq_songs_to_add: Vec<String>) {
        let mf = File::open(get_jedmp_musiccache_path()).unwrap();
    }
}
