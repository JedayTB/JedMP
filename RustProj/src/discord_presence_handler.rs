pub mod discord_presence {
    use discord_presence::{Client, Event};

    use std::fmt;
    use std::sync::RwLock;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::{Sender, TryRecvError};
    use std::thread;
    use std::time::Duration;

    pub enum DrpcEvents {
        UpdateActivity,
    }

    impl fmt::Display for DrpcEvents {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::UpdateActivity => write!(f, "{}", self),
            }
        }
    }

    pub static DRPC_SENDER: RwLock<Vec<Sender<String>>> = RwLock::new(Vec::new());
    pub fn start_discord_rpc() {
        let (tx, rx) = channel::<String>();

        DRPC_SENDER.write().unwrap().push(tx.to_owned());

        thread::spawn(move || {
            println!("[Debug/DiscordPresence] Discord RPC test");
            // Get our main status message
            let state_message = "Idling.";
            let status_display = "JedMP";
            // Create the client
            let mut drpc = Client::new(1003450375732482138);
            // Register event handlers with the corresponding methods
            let _ = drpc
                .on_ready(|_ctx| {
                    println!("[DiscordPresence_client]\tready?");
                })
                .persist();

            // or

            let _ = drpc
                .on_event(Event::Ready, |_ctx| {
                    println!("[DiscordPresence_client]\tREADY!");
                })
                .persist();
            let _ = drpc
                .on_event(Event::Connected, |_ctx| {
                    println!("[DiscordPresence_client]\tConnected to Discord");
                    dbg!(_ctx.event);
                })
                .persist();

            drpc.start();
            drpc.block_until_event(Event::Ready).unwrap();

            assert!(Client::is_ready());

            // Set the activity
            let _ = drpc
                .set_activity(|act| {
                    act.state(status_display)
                        .details(state_message)
                        //.assets(|Assets| {})
                        .activity_type(discord_presence::models::ActivityType::Listening)
                        .status_display(discord_presence::models::DisplayType::State)
                })
                .expect("Failed to set activity");

            println!("[Debug/DiscordPresence]\tDrpc Activity set");

            // Loop until recieves kill
            loop {
                //print!("\033[2K\r awaiting signal");

                // Rust thread events builtins
                match rx.try_recv() {
                    Ok(val) => {
                        let vc = val.clone();
                        println!("[Debug/DRPC/DRPC_Thread]\tRecieved update. details set to {vc} ");
                        drpc.set_activity(|act| {
                            act.state(status_display)
                                .details(val)
                                //.assets(|Assets| {})
                                .activity_type(discord_presence::models::ActivityType::Listening)
                                .status_display(discord_presence::models::DisplayType::State)
                        })
                        .expect("Failed to set activity");
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        //print!("IN THREAD LOOP Terminating\n");
                        //drpc.shutdown().unwrap();
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        //println!("D_rpc thread recieved empty");
                    }
                }

                thread::sleep(Duration::from_millis(16))
            }
        });

        //tx.send(TryRecvError::Disconnected).unwrap();
    }
}
