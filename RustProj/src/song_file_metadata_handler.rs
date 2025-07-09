pub mod song_file_metadata_handler {
    pub fn get_song_title(song_path: &String) -> String {
        let aud_file = taglib::File::new(song_path).expect("Not a valid audio file.");
        let title = aud_file.tag().expect("Has no title").title();
        match title {
            Some(_) => title.unwrap(),
            None => {
                let parts = song_path.split("/").last().unwrap();
                let split_at_dot: Vec<&str> = parts.split(".").collect();
                let make_shift_title = split_at_dot[0].to_owned();
                return make_shift_title;
            }
        }
    }
}
