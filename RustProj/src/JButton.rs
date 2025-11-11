#![allow(non_camel_case_types)]
pub mod JButton {

    use crate::colors_handler::color_handler::*;

    use fltk::{button::Button, enums::Color, enums::Event, prelude::*, *};
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

            but.handle(move |_button, ev: Event| match ev {
                Event::Enter => {
                    // Functioning weird.
                    //button.set_color(get_jedmp_color(JedMP_Colors::Important_text_color));
                    true
                }

                Event::Leave => {
                    //button.set_color(get_jedmp_color(JedMP_Colors::Button_bg_color));
                    true
                }
                _ => false,
            });
            Self { but }
        }

        pub fn ColorOverrides(&mut self, bg: Color, text: Color, pressed: Color, hover: Color) {
            self.set_color(bg);
            self.set_label_color(text);
            self.set_selection_color(pressed);

            self.handle(move |button, ev: Event| match ev {
                Event::Enter => {
                    button.set_color(hover.to_owned());
                    true
                }
                Event::Leave => {
                    button.set_color(bg.to_owned());
                    true
                }
                _ => false,
            });
        }
    }
}
