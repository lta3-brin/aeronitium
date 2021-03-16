use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type FirmwarePage = ResizedView<Panel<TextView>>;

pub fn build_firmware() -> FirmwarePage {
    let widget = Panel::new(
        TextView::new("Halaman Versi DTC").align(Align::center())
    ).full_screen();

    widget
}
