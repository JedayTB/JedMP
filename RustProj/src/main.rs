// Modules
#![allow(non_snake_case)]

pub mod JButton;
pub mod colors_handler;
pub mod gui_state_controller;
pub mod music_cache_handler;
pub mod music_play_queue_handler;
pub mod play_queue_song;
pub mod popup_window;
pub mod song_identifier;
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::gui_state_controller::gui_controller;

//TODO:
//Rewrite jedmpdir handling from music_cache_handler
//to here and let music_cache_handler / colors_handler
//Handle the creation of their indiviual files.
fn main() {
    // For mostly debugging.

    let jedmpdir = get_jedmp_dir();
    // CMD Args handling
    let args: Vec<String> = env::args().collect();
    for cmd_args in args {
        if cmd_args == "r" {
            // Redo first init logic here
            println!("Argument r found, removing jedmp_directory for testing.");
            match fs::remove_dir_all(&jedmpdir) {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error occured! {e}");
                }
            };
        }
    }

    // Handle if JedMPDir exists
    handle_jedmp_directory();

    music_cache_handler::music_file_handler::try_load_cached_music();

    colors_handler::color_handler::try_load_mastercolorrc();

    // Most things happen in gui_controller.
    // Just to keep GUI Events and logic close, otherwise it's a pain.
    gui_controller::open_window();
}

fn handle_jedmp_directory() {
    let jed_dir_path_buf = PathBuf::from(get_jedmp_dir());

    if jed_dir_path_buf
        .try_exists()
        .expect("Couldn't check validation of jedmpdir")
        == false
    {
        fs::create_dir(jed_dir_path_buf).expect("Couldn't create Jed MP Directory");
    }
}

fn get_jedmp_dir() -> String {
    let username_string = whoami::username();

    return format!("/home/{username_string}/.jedmp");
}
fn get_jedmp_musiccache_path() -> String {
    let jedmpdir = get_jedmp_dir();
    return format!("{jedmpdir}/music_cache");
}
fn get_jedmp_master_color_file_path() -> String {
    let jedmpdir = get_jedmp_dir();

    return format!("{jedmpdir}/master_colorrc");
}
