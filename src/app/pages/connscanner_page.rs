use cursive::views::{TextView, LinearLayout, NamedView};
use crate::app::components::container::aeronitium_container;


pub fn build_connscanner() -> NamedView<LinearLayout> {
    let widget = aeronitium_container(
        TextView::new("Halaman Koneksi Pemindai"),
        |_| {}
    );

    widget
}
