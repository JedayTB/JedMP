pub mod playlist_handler {

    use std::{
        fs::{self, File, OpenOptions},
        io::Write,
        path::PathBuf,
    };

    use crate::get_jedmp_playlist_dir;
    pub struct Playlist {}

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

    pub fn add_song_to_playlst(playlist_name: &String, song: String) {
        let pdir = get_jedmp_playlist_dir();
        let sel_pl_path = format!("{pdir}/{playlist_name}");
        let mut mf = OpenOptions::new()
            .append(true)
            .open(sel_pl_path)
            .expect("Couldn't open playlist");

        let writestr = process_song_to_string(song);

        write!(mf, "{}", writestr).expect("Failed to write to file");
    }

    pub fn add_multiple_songs_to_playlist(playlist_name: &String, pq_songs_to_add: Vec<String>) {
        let pdir = get_jedmp_playlist_dir();
        let sel_pl_path = format!("{pdir}/{playlist_name}");
        let mut mf = OpenOptions::new()
            .append(true)
            .open(sel_pl_path)
            .expect("Couldn't open playlist");

        let writestr = process_multiple_songs_to_string(pq_songs_to_add);

        write!(mf, "{}", writestr).expect("Failed to write to file");
    }
    fn process_multiple_songs_to_string(songs: Vec<String>) -> String {
        let mut ret_string: String = "".to_owned();
        for s in songs {
            let tF = taglib::File::new(&s).expect("Couldn't open song file as taglib file");

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
                title = s.split("/").last().unwrap().to_owned();
            }
            ret_string.push_str(&format!("{s}\x00{title}\x00{album}\x00{artist}\n"));
        }
        return ret_string;
    }
    fn process_song_to_string(pathstr: String) -> String {
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
        return s;
    }
}
