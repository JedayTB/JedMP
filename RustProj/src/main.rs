// Modules
pub mod gui_state_controller;
pub mod music_cache_handler;
pub mod music_play_queue_handler;
pub mod play_queue_song;
pub mod popup_window;
pub mod song_file_metadata_handler;
pub mod song_identifier;

use std::env;
use std::fs;

use crate::gui_state_controller::gui_controller;
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
    // Because Im bad at coding, this must be called before anything to do with
    // play queue is done.
    music_cache_handler::music_file_handler::try_load_cached_music();

    // Everything happens in gui_controller - because separating logic that far out
    // Is a pain in the ass with this language.
    gui_controller::open_window();
}

fn get_jedmp_dir() -> String {
    let username_string = whoami::username();

    return format!("/home/{username_string}/.jedmp");
}
fn get_jedmp_musiccache_path() -> String {
    let jedmpdir = get_jedmp_dir();
    return format!("{jedmpdir}/music_cache");
}
