use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type PrintCoefPage = ResizedView<Panel<TextView>>;

pub fn build_printcoef() -> PrintCoefPage {
    let widget = Panel::new(
        TextView::new("Halaman Cetak Koefisien").align(Align::center())
    ).full_screen();

    widget
}
