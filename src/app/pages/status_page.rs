use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type StatusScannerPage = ResizedView<Panel<TextView>>;

pub fn build_status_scaner() -> StatusScannerPage {
    let widget = Panel::new(
        TextView::new("Halaman Status Pemindai").align(Align::center())
    ).full_screen();

    widget
}
