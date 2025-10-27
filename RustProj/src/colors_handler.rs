#![allow(non_camel_case_types)]
pub mod color_handler {
    use std::fs::File;

    use crate::get_jedmp_master_color_file_path;
    use fltk::enums::Color;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    pub static COLOR_DICTIONARY: OnceLock<Vec<Color>> = OnceLock::new();
    #[derive(Eq, PartialEq)]
    pub enum JedMP_Colors {
        Background_color = 0,
        Text_color = 1,
        Important_text_color = 2,
        Song_text_color = 3,
        Song_iden_bg_color = 4,
        Song_hover_color = 5,
        Button_bg_color = 6,
        Button_hover_color = 7,
        Scroll_bar_color = 8,
    }

    pub fn try_load_mastercolorrc() {
        let color_rc_file_path = get_jedmp_master_color_file_path();
        let pathb = PathBuf::from(&color_rc_file_path);

        let exists = pathb.try_exists().expect("Path doesn't exist");

        if exists {
            println!("[Colors Handler] colorrc found. Loading...");
            load_colorrc(&color_rc_file_path);
        } else {
            println!("[Colors Handler] colorrc not found. Creating with defaults.");
            //"Unable to create colorrc. Aborting."
            let mut crc = File::create(&color_rc_file_path).unwrap();
            println!("[Colors Handler] colorrc created. Populating with defaults.");
            write_colorrc_defaults(&mut crc);
            load_colorrc(&color_rc_file_path);
        }
    }

    fn load_colorrc(colorrc_path: &str) {
        let mut colors: Vec<Color> = Vec::new();
        let colorrc_file = File::open(colorrc_path).expect("Couldn't open path");

        let buf_read = BufReader::new(colorrc_file);
        let file_lines = buf_read.lines();
        // There should be a better way to do this but oh well

        for line in file_lines {
            //TODO:
            //Do error checking, if a field is missing or errors,
            //Make system use defaults
            //Also, this is forcing that All text fields be in order..
            let strl = line
                .unwrap()
                .split("#")
                .last()
                .expect("Expected a string")
                .to_owned();
            //println!("{strl}");
            let color_as_hex =
                u32::from_str_radix(&strl, 16).expect("Couldn't parse from hex string");
            let col = Color::from_hex(color_as_hex);

            colors.push(col);
        }

        COLOR_DICTIONARY
            .set(colors)
            .expect("Couldn't set COLOR_DICTIONARY");
    }

    fn write_colorrc_defaults(colorrc_file: &mut File) {
        let colorrc_default_fields = "Background color:#0D001A
Text_color:#6CB9C9
Important_text_color:#9ED198
song_text_color:#B0D199
Song_iden_bg_color:#2E0A30
Song_Hover_color:#4F1446
Button_BG_color:#2E0A30
Button_hover_color:#4F1446
Scroll_bar_color:#6E5181";

        write!(colorrc_file, "{}", colorrc_default_fields)
            .expect("[Colors Handler] Could not write to colorrc");
    }
}
