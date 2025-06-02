use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
use fltk_theme::{ColorTheme, color_themes, widget_schemes};
use rodio::{Decoder, OutputStream, Sink};
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::BLACK_THEME);
    theme.apply();

    let mut wind = Window::new(0, 0, 680, 360, "JedMP");
    let queue_list = Flex::default()
        .column()
        .with_size(65, 300)
        .left_of(&wind, -70);
    let _frame = Frame::default().with_label("Test");

    queue_list.end();

    let mut button_box = Flex::default()
        .with_size(620, 45)
        .row()
        .below_of(&wind, -50);

    let mut last_song_button = Button::default().with_label("<");
    let mut pause_song_button = Button::default().with_label("Pause");
    let mut next_song_button = Button::default().with_label(">");

    button_box.end();

    let button_box_padding_from_queue_list = 40;
    let new_button_box_x = queue_list.x() + button_box_padding_from_queue_list;
    let new_button_box_y = button_box.y();

    button_box.set_pos(new_button_box_x, new_button_box_y);

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
    wind.end();
    wind.show();

    app.run().unwrap();
}

fn make_queue_list_frames(mut queue_list_box: Flex, play_queue: &Vec<String>) {
    for path in play_queue {
        let new_frame = Frame::default().with_label(path);
        queue_list_box.add(&new_frame);
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
    println!("{}", print_string);
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
