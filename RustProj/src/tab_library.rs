pub mod Tab_Library {

    use fltk::widget_extends;

    use fltk::group::{Pack, Scroll};
    use fltk::prelude::WidgetExt;
    pub struct TabLibrary {
        pub lib_scroll: Scroll,
        pub scroll_pack: Pack,
        pub pq_idx_belongs_to: usize,
    }
    widget_extends!(TabLibrary, Scroll, lib_scroll);
    impl TabLibrary {
        pub fn new(pq_idx: usize) -> Self {
            let lib_scroll = Scroll::default();
            let pq_idx_belongs_to = pq_idx;
            let scroll_pack = Pack::default();

            TabLibrary {
                lib_scroll,
                scroll_pack,
                pq_idx_belongs_to,
            }
        }
    }
}
