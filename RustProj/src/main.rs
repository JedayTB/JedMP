use fltk::dialog;
use fltk::{app, button::Button, enums::*, group::Flex, prelude::*, window::Window};
use fltk_theme::{ColorTheme, color_themes};
use rodio::{Decoder, OutputStream, Sink};
use song_identifier::SongIdentifier;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

// Modules
mod init_app_gui;
mod song_identifier;
// Rename to app_gui
use crate::init_app_gui::gui_init;
fn main() {
    gui_init::test();
    // CMD Args handling
    let args: Vec<String> = env::args().collect();
    for cmd_args in args {
        if cmd_args == "r" {
            // Redo first init logic here
        }
    }
    let username_string = whoami::username();

    let jedmp_directory = format!("/home/{username_string}/.jedmp");

    let pathb = PathBuf::from(&jedmp_directory);
    match pathb.try_exists() {
        Ok(t) => {
            if t == false {
                println!("Jed MP Folder does not exist. Creating and populating...");
                match fs::create_dir(jedmp_directory) {
                    Ok(_o) => {
                        // Yay..
                    }
                    Err(e) => {
                        eprintln!("Error Occured trying to create directory. {e}");
                    }
                }

                // Do my logic here.
            } else {
                // Load Cached values..
                println!("Loading cached values...");
            }
        }
        Err(e) => {
            eprintln!("{e} The hell happened?")
        }
    }
    // GUI Stuff
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    let theme = ColorTheme::new(color_themes::TAN_THEME);
    theme.apply();
    let base_window_width = 896;
    let base_window_height = 504;

    let general_y_pad = 10;
    //let general_x_pad = 15;

    let mut wind = Window::new(0, 0, base_window_width, base_window_height, "JedMP");

    let top_bar_height = 25;

    // Top Bar
    let mut top_bar_group = Flex::default()
        .with_size(base_window_width, top_bar_height)
        .with_pos(0, 0);

    top_bar_group.set_frame(FrameType::GtkDownFrame);

    let mut add_music_directory_button = Button::default()
        .with_size(base_window_width / 12, top_bar_height)
        .with_label("Choose Music directory");

    top_bar_group.end();

    let queue_list_width = 500;
    let queue_list_height = 300;

    let queue_list_pos_x = 0;
    let queue_list_pos_y = 0;
    let mut queue_list = Flex::default()
        .column()
        .with_size(queue_list_width, queue_list_height)
        .with_pos(
            queue_list_pos_x,
            queue_list_pos_y + general_y_pad + top_bar_height,
        );

    queue_list.set_frame(FrameType::GtkDownFrame);
    queue_list.end();

    let button_box_height = base_window_height / 8;
    let button_box_width = base_window_width;
    let button_box_pos_y = wind.h();
    let button_box_pos_x = base_window_width / 2;

    let button_box = Flex::default()
        .with_size(button_box_width, button_box_height)
        .with_pos(
            button_box_pos_x - button_box_width / 2,
            button_box_pos_y - button_box_height,
        )
        .row();

    let mut last_song_button = Button::default().with_label("<");
    let mut pause_song_button = Button::default().with_label("Pause");
    let mut next_song_button = Button::default().with_label(">");

    button_box.end();

    let play_queue = Rc::new(RefCell::new(scan_directory("TestMusicFiles")));

    let play_queue_last = Rc::clone(&play_queue);
    let play_queue_next = Rc::clone(&play_queue);

    let current_song_index = Rc::new(RefCell::new(0usize));
    let index_next_pointer = Rc::clone(&current_song_index);
    let index_last_pointer = Rc::clone(&current_song_index);

    make_queue_list_frames(queue_list, &play_queue.borrow().clone());
    //const TESTMP3PATH: &str = "TestMusicFiles/07 Alright.mp3";
    // Get an output steam handle to the default physical sound device
    // Note that no sound will be played if _stream is droppped;
    // Stream must live as long as sink

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Rc::new(RefCell::new(Sink::try_new(&stream_handle).unwrap()));
    let sink_pause = Rc::clone(&sink);
    let sink_next = Rc::clone(&sink);
    let sink_last = Rc::clone(&sink);
    // load a sound from a file, using a path relative to cargo.toml
    let file = BufReader::new(File::open(play_queue.borrow()[0].clone()).unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Sink has weird behavior where it will play
    // song after having it be appended
    sink.borrow().append(source);
    sink.borrow().pause();

    last_song_button.set_callback(move |_| {
        let curr_ind = *index_last_pointer.borrow();
        let new_ind = back_a_song(&sink_last.borrow(), &play_queue_last.borrow(), curr_ind);
        *index_last_pointer.borrow_mut() = new_ind;
    });

    next_song_button.set_callback(move |_| {
        let curr_ind = *index_next_pointer.borrow();
        let new_ind = next_song_in_queue(&sink_next.borrow(), &play_queue_next.borrow(), curr_ind);
        *index_next_pointer.borrow_mut() = new_ind;
    });

    pause_song_button.set_callback(move |btn| {
        if sink_pause.borrow().is_paused() {
            sink.borrow().play();
            btn.set_label("Pause");
        } else {
            sink_pause.borrow().pause();
            btn.set_label("Play");
        }
    });

    add_music_directory_button.set_callback(move |_| {
        let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseDir);
        nfc.set_option(dialog::NativeFileChooserOptions::SaveAsConfirm);
        match nfc.try_show() {
            Err(e) => {
                eprintln!("{}", e);
                //None
            }
            Ok(a) => match a {
                dialog::NativeFileChooserAction::Success => {
                    let directory = nfc.filename();
                    println!("{:?}", directory);
                }
                dialog::NativeFileChooserAction::Cancelled => {
                    println!("Directory Pick cancelled");
                }
            },
        }
    });
    wind.end();
    //wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

fn proccess_chosen_directory(dir_path: &str) {
    // Scans for files in the given directory.
    let pathsindir = fs::read_dir(dir_path).unwrap();
    //let mut plqueue: Vec<String> = Vec::new();
    for path in pathsindir {
        let mut pathb = PathBuf::new();
        let pathstr = path.unwrap().path().display().to_string();
        pathb.push(&pathstr);

        if pathb.is_dir() {
            scan_directory(&pathstr);
        } else if pathb.is_file() {
        }
        //fs::metadata(path).map_errO;
    }
}
fn scan_directory(dir_path: &str) -> Vec<String> {
    // Scans for files in the given directory.
    let pathsindir = fs::read_dir(dir_path).unwrap();
    let mut plqueue: Vec<String> = Vec::new();
    for path in pathsindir {
        //println!("Name: {}", path.unwrap().path().display());
        plqueue.push(path.unwrap().path().display().to_string());
    }
    return plqueue;
}
fn make_queue_list_frames(mut queue_list_box: Flex, play_queue: &Vec<String>) {
    for path in play_queue {
        let _path = path.split("/");
        let songname = _path.collect::<Vec<&str>>();
        let si = SongIdentifier::new(100, 30, songname[1], Align::Right);

        queue_list_box.add(&*si);
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
    //println!("{}", print_string);
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
