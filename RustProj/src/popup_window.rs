// Tell rust to stop annoying me mid dev
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub mod popup_window {
    use fltk::app::delete_widget;
    use fltk::{button::Button, enums::*, prelude::*, window::DoubleWindow, *};
    use std::cell::RefCell;
    use std::ops::{Deref, DerefMut};
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
                Event::Unfocus => {
                    win.hide();
                    true
                }

                _ => false,
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
