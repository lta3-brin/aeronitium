use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type ScannerListPage = ResizedView<Panel<TextView>>;

pub fn build_scannerlist() -> ScannerListPage {
    let widget = Panel::new(
        TextView::new("Halaman Daftar Pemindai").align(Align::center())
    ).full_screen();

    widget
}
