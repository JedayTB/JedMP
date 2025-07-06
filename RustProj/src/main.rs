// Modules
pub mod gui_state_controller;
pub mod music_cache_handler;
pub mod song_identifier;

use std::env;
use std::fs::File;
use std::fs::{self, OpenOptions};

use rodio::{Decoder, Sink};
use std::io::{BufReader, Write};
use std::path::PathBuf;

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
/// Scans A Directory and appends the path of the files into the music_cache file.
fn proccess_chosen_directory(dir_path: &str, cached_songs_path: &str) {
    let pathsindir = fs::read_dir(dir_path).unwrap();
    let mut music_cache_file = OpenOptions::new()
        .append(true)
        .open(cached_songs_path)
        .expect("Couldn't open music_cache");

    for path in pathsindir {
        let mut pathb = PathBuf::new();
        let pathstr = path.unwrap().path().display().to_string();
        pathb.push(&pathstr);

        if pathb.is_dir() {
            println!(
                "Encountered secondary directory {:?}: Scanning and caching",
                pathstr
            );
            scan_directory_to_cached_songs(&pathstr, cached_songs_path);
        } else if pathb.is_file() {
            println!("Writing {:?}", pathstr);
            //let song_path_str = format!("{}\n", pathstr);
            writeln!(music_cache_file, "{}", pathstr).expect("Write failed.");
        }
    }
}
fn scan_directory_to_cached_songs(dir_path: &str, cached_songs_path: &str) {
    let pathsindir = fs::read_dir(dir_path).unwrap();
    for path in pathsindir {
        let song_path = path.unwrap().path().display().to_string();

        println!("Writing {:?}", song_path);
        fs::write(cached_songs_path, song_path).expect("Couldn't write.");
    }
}

fn _list_queue(play_queue: &Vec<String>, current_song_index: usize) {
    let mut i: i32 = 0;
    let mut print_string: String = String::from("");
    for song_name in play_queue {
        print_string.push_str(song_name);
        i += 1;
        if i == current_song_index as i32 {
            print_string.push_str("\t<--- Listening");
        }
        print_string.push_str("\n");
    }
}
fn back_a_song(sink: &Sink, play_queue: &Vec<String>, current_song_index: usize) -> usize {
    let mut new_ind: usize = current_song_index;
    if new_ind != 0 {
        new_ind -= 1;
    }
    let next_song = BufReader::new(File::open(play_queue[new_ind].clone()).unwrap());
    let source = Decoder::new(next_song).unwrap();
    // Stops playback and clears all appened files
    sink.stop();
    sink.append(source);
    sink.play();

    return new_ind;
}
fn next_song_in_queue(sink: &Sink, play_queue: &Vec<String>, current_song_index: usize) -> usize {
    let mut new_ind: usize = current_song_index;
    new_ind += 1;

    if new_ind > play_queue.len() - 1 {
        new_ind -= 1;
    }
    let next_song = BufReader::new(File::open(play_queue[new_ind].clone()).unwrap());
    let source = Decoder::new(next_song).unwrap();
    // Stops playback and clears all appened files
    sink.stop();
    sink.append(source);
    sink.play();

    return new_ind;
}
