use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type ConnScannPage = ResizedView<Panel<TextView>>;

pub fn build_connscanner() -> ConnScannPage {
    let widget = Panel::new(
        TextView::new("Halaman Koneksi Pemindai").align(Align::center())
    ).full_screen();

    widget
}
