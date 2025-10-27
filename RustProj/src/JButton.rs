#![allow(non_camel_case_types)]
pub mod JButton {

    use crate::colors_handler::color_handler::*;
    use fltk::{button::Button, prelude::*, *};
    pub struct J_Button {
        pub but: Button,
    }
    widget_extends!(J_Button, Button, but);
    impl J_Button {
        pub fn new() -> Self {
            let mut but = Button::default();
            but.set_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Button_bg_color as usize]);
            but.set_label_color(COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Text_color as usize]);
            but.set_selection_color(
                COLOR_DICTIONARY.get().unwrap()[JedMP_Colors::Button_hover_color as usize],
            );
            Self { but }
        }
    }
}
