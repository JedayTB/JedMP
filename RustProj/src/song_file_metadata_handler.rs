pub mod song_file_metadata_handler {
    use audiotags::*;

    pub fn get_song_title(song_path: &String) -> String {
        let binding = Tag::new().read_from_path(song_path).unwrap();
        let result = binding.title();
        match result {
            Some(_) => {
                return result.unwrap().to_owned();
            }
            None => {
                println!("File title invalid, File path: {:?}", song_path.clone());
                return song_path.to_owned();
            }
        }
    }
}
