#![allow(non_camel_case_types)]
pub mod color_handler {
    use std::fs::File;

    use crate::get_jedmp_master_color_file_path;
    use fltk::enums::Color;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    pub static COLOR_DICTIONARY: OnceLock<Vec<Color>> = OnceLock::new();

    #[derive(Eq, PartialEq, Debug)]
    pub enum JedMP_Colors {
        Background_color,
        Text_color,
        Important_text_color,
        Libary_Song_text_color,
        Playqueue_Song_text_color,
        Song_iden_bg_color,
        Song_hover_color,
        Button_bg_color,
        Button_Text_color,
        Button_hover_color,
        Scroll_bar_color,
        Tabs_bg_color,
    }

    pub fn get_jedmp_color(Color: JedMP_Colors) -> Color {
        return COLOR_DICTIONARY.get().unwrap()[Color as usize];
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
    /*
    #f7768e #f7768e 	This keyword, HTML elements, Regex group symbol, CSS units, Terminal Red
    #ff9e64 #ff9e64 	Number and Boolean constants, Language support constants
    #e0af68 #e0af68 	Function parameters, Regex character sets, Terminal Yellow
    #cfc9c2 #cfc9c2 	Parameters inside functions (semantic highlighting only)
    #9ece6a #9ece6a 	Strings, CSS class names
    #73daca #73daca 	Object literal keys, Markdown links, Terminal Green
    #b4f9f8 #b4f9f8 	Regex literal strings
    #2ac3de #2ac3de 	Language support functions, CSS HTML elements
    #7dcfff #7dcfff 	Object properties, Regex quantifiers and flags, Markdown headings, Terminal Cyan, Markdown code, Import/export keywords
    #7aa2f7 #7aa2f7 	Function names, CSS property names, Terminal Blue
    #bb9af7 #bb9af7 	Control Keywords, Storage Types, Regex symbols and operators, HTML Attributes, Terminal Magenta
    #c0caf5 #c0caf5 	Variables, Class names, Terminal White
    #a9b1d6 #a9b1d6 	Editor Foreground
    #9aa5ce #9aa5ce 	Markdown Text, HTML Text
    #565f89 #565f89 	Comments
    #414868 #414868 	Terminal Black
    #24283b #24283b 	Editor Background (Storm)
    #1a1b26 #1a1b26 	Editor Background (Night)
    */
    fn write_colorrc_defaults(colorrc_file: &mut File) {
        let colorrc_default_fields = "Background color:#1A1B26
Text_color:#CFC9C2
Important_text_color:#F7768E
library_song_text_color:#B0D199
playqueue_song_text_color:#E0AF68
Song_iden_bg_color:#24283B
Song_Hover_color:#4F1446
Button_BG_color:#565F89
Button_Text_color:#FF9E64
Button_hover_color:#4F1446
Scroll_bar_color:#565F89
Tabs_bg_color:#414868";

        write!(colorrc_file, "{}", colorrc_default_fields)
            .expect("[Colors Handler] Could not write to colorrc");
    }
}
