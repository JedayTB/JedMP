pub mod AudioMediaPlayer {
    use std::thread;
    use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx};

    use std::sync::RwLock;
    use std::sync::mpsc::{SendError, channel};
    use std::sync::mpsc::{Sender, TryRecvError};

    use crate::THREAD_POLL_RATE;

    static MUSIC_PLAYER_SENDER: RwLock<Vec<Sender<String>>> = RwLock::new(Vec::new());

    pub enum MpMessage {
        SetAndPlay,
        Play,
        Pause,
        Seek,
        VolumeAdjust,
    }
    pub static STARTUPPERSISTENTVOLUME: RwLock<i32> = RwLock::new(0);
    pub static CURRENTSONGTIME: RwLock<i64> = RwLock::new(0);

    pub fn Start_music_player() {
        let (tx, rx) = channel::<String>();

        MUSIC_PLAYER_SENDER.write().unwrap().push(tx.to_owned());

        thread::spawn(move || {
            let instance = Instance::new().unwrap();
            instance.set_user_agent("JedMP - Using LibVLC", "What do I put here");
            //instance.set_app_id(id, version, icon);
            //TODO: Set icon when we have art.
            let player = MediaPlayer::new(&instance).unwrap();
            let spv = player.get_volume();
            *STARTUPPERSISTENTVOLUME.write().unwrap() = spv;

            loop {
                //print!("\033[2K\r awaiting signal");

                let s_time = player.get_time();

                match s_time {
                    Some(t) => {
                        *CURRENTSONGTIME.write().unwrap() = t;
                    }

                    None => {
                        let st = *CURRENTSONGTIME.read().unwrap();
                        // Just so we don't set it every update tick.
                        // idk if this really matters. Oh well.
                        if st != 0 {
                            *CURRENTSONGTIME.write().unwrap() = 0;
                        }
                    }
                }

                // Rust thread events builtins
                match rx.try_recv() {
                    Ok(val) => {
                        let vc = val.clone();
                        println!("[Debug/AudioMediaPlayer] recieved message {vc}");
                        let msg_args: Vec<&str> = vc.split("\x00").collect();
                        //dbg!(&msg_args);

                        let msg = msg_args[0];
                        let data = msg_args[1];

                        if msg == "SetAndPlay" {
                            let media =
                                Media::new_path(&instance, data).expect("Path does not exist");

                            player.stop();
                            player.set_media(&media);
                            player.play().unwrap();
                        } else if msg == "Seek" {
                            let s_time: i64 = data.parse::<i64>().expect("Couldn't parse.");
                            player.set_time(s_time);
                        } else if msg == "VolumeAdjust" {
                            let vol: i32 = data.parse::<i32>().expect("Couldn't parse");

                            player.set_volume(vol).expect("Couldn't set volume.");
                            //println!("[Debug/AudioMediaPlayer] Volume set");
                        } else if msg == "Play" {
                            player.play().expect("couldn't play");
                        } else if msg == "Pause" {
                            player.pause();
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        //print!("IN THREAD LOOP Terminating\n");
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        //println!("D_rpc thread recieved empty");
                    }
                }

                thread::sleep(THREAD_POLL_RATE)
            }
        });
    }

    ///Send's a message to the MusicPlayer thread. Data is optional unless used with
    ///MpMessage::Play, MpMessage::Seek and MpMessage::VolumeAdjust
    ///This is so shit. But it's the best way I could think of
    pub fn MessagePlayerThread(msg: MpMessage, data: String) -> Result<(), SendError<String>> {
        let tx = &MUSIC_PLAYER_SENDER.write().unwrap()[0];
        let send_string: String;
        match msg {
            MpMessage::SetAndPlay => {
                send_string = format!("SetAndPlay\x00{data}");
            }
            MpMessage::Seek => {
                send_string = format!("Seek\x00{data}");
            }
            MpMessage::VolumeAdjust => {
                send_string = format!("VolumeAdjust\x00{data}");
            }
            MpMessage::Play => {
                send_string = String::from("Play\x00 ");
            }
            MpMessage::Pause => {
                send_string = String::from("Pause\x00 ");
            }
        }
        tx.send(send_string)
    }
}
