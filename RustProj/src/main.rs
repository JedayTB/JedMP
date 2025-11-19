// Modules
#![allow(non_snake_case)]

pub mod JButton;
pub mod Playlist_Tab;
pub mod artist_frame;
pub mod colors_handler;
pub mod gui_state_controller;
pub mod music_cache_handler;
pub mod music_play_queue_handler;
pub mod play_queue_song;
pub mod playlist_handler;
pub mod playlist_window;
pub mod popup_window;
pub mod song_identifier;
pub mod tab_library;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::abort;

use crate::gui_state_controller::gui_controller;

fn main() {
    // Mostly for debugging purposes
    handle_cmd_args();
    // Handle if JedMPDir exists
    handle_jedmp_directory();

    // Load music_cache file if exists, create it
    music_cache_handler::music_file_handler::try_load_cached_music();
    // Same pattern as above
    colors_handler::color_handler::try_load_mastercolorrc();

    playlist_handler::playlist_handler::try_create_playlist_dir();

    // Most things happen in gui_controller.
    // Just to keep GUI Events and logic close, otherwise it's a pain.
    gui_controller::open_window();
}
fn handle_cmd_args() {
    // CMD Args handling
    let args: Vec<String> = env::args().collect();
    for cmd_args in args {
        if cmd_args == "h" {
            println!("Usage: jedmp -[arg]\nPlay music sensibly.\nWith no argument specified, jedmp launches regularly.\n
                -h\tGet this menu
                -ra\tRemove all jedmp dot files (Usually at ~.jedmp)
                -rp\tRemove all playlists
                -rm\tRemove jedmp's music_cache
                -rc\tRemove jedmp's colorrc");
            abort();
        } else if cmd_args == "ra" {
            // Redo first init logic here
            println!("Argument ra found, removing ~/.jedmp for testing.");
            match fs::remove_dir_all(get_jedmp_dir()) {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error occured trying to remove ~/.jedmp! {e}");
                }
            };
        } else if cmd_args == "rp" {
            println!("Argument 'rp' found. Removing ~.jedmp/playlists for testing");
            match fs::remove_dir_all(get_jedmp_playlist_dir()) {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error occured trying to remove ~/.jedmp/Playlists! {e}");
                }
            }
        } else if cmd_args == "rm" {
            println!("Argument 'rm' found. Removing ~.jedmp/music_cache for testing");
            match fs::remove_file(get_jedmp_musiccache_path()) {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error occured trying to remove ~/.jedmp/music_cache {e}");
                }
            }
        } else if cmd_args == "rc" {
            println!("Argumennt 'rc' found. Remoing ~/.jedmp/master_colorrc");

            match fs::remove_file(get_jedmp_master_color_file_path()) {
                Ok(_r) => {}
                Err(e) => {
                    eprintln!("Error occured trying to remove ~/.jedmp/master_colorrc {e}");
                }
            }
        }
    }
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
fn get_jedmp_playlist_dir() -> String {
    let jedmp_pl_dir = get_jedmp_dir();
    return format!("{jedmp_pl_dir}/playlists");
}
fn get_jedmp_musiccache_path() -> String {
    let jedmpdir = get_jedmp_dir();
    return format!("{jedmpdir}/music_cache");
}
fn get_jedmp_master_color_file_path() -> String {
    let jedmpdir = get_jedmp_dir();

    return format!("{jedmpdir}/master_colorrc");
}
