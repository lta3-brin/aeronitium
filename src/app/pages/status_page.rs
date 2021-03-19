use cursive::views::{TextView, LinearLayout, NamedView};
use crate::app::components::container::aeronitium_container;


pub fn build_status_scaner() -> NamedView<LinearLayout> {
    let widget = aeronitium_container(
        TextView::new("Halaman Status Pemindai"),
        |_| {}
    );

    widget
}
