// Tell rust to stop annoying me mid dev
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod popup_window {
    use fltk::{button::Button, enums::*, prelude::*, widget::Widget, window::DoubleWindow, *};
    use std::cell::RefCell;
    use std::rc::Rc;

    widget_extends!(PopupWindow, window::Window, win);
    pub struct PopupWindow {
        win: window::Window,
    }
    impl PopupWindow {
        pub fn new(width: i32, choices: &[&str]) -> Self {
            let mut win = window::Window::default().with_size(width, choices.len() as i32 * 25);
            win.set_color(Color::White);
            win.set_frame(FrameType::BorderBox);

            let mut pack = group::Pack::new(1, 1, win.w() - 2, win.h() - 2, None);
            win.set_border(false);
            win.handle(move |win, event| match event {
                Event::Push => {
                    win.trigger();
                    true
                }
                Event::Unfocus => {
                    win.hide();
                    true
                }
                _ => false,
            });
            // More thorough handling when a click is detected
            // For some reason doesn't get called.. ever
            // whatever
            win.set_callback(move |win| {
                // 1 left, 2 middle, 3 right.
                println!("{:?},hello? ", app::event_button());
                if app::event_button() == 1 {
                    println!("Mouse button was pressed");
                    let m_x = app::event_x_root();
                    let m_y = app::event_y_root();

                    let win_x = win.x();
                    let win_y = win.y();
                    let win_w = win.width();
                    let win_h = win.height();

                    let within_x = m_x > win_x && m_x < win_x + win_w;
                    let within_y = m_y > win_y && m_y < win_y + win_h;

                    if within_x == false && within_y == false {
                        win.hide();
                    }

                    win.hide();
                }
            });
            win.show();
            win.end();

            for (i, choice) in choices.iter().enumerate() {
                let mut but = Button::default().with_size(width, 25).with_label(choice);
                but.clear_visible_focus();
                pack.add(&but);
            }
            pack.auto_layout();
            Self { win }
        }
    }
}
