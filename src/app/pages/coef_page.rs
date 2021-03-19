use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type CoefficientPage = ResizedView<Panel<TextView>>;

pub fn build_coefficient() -> CoefficientPage {
    let widget = Panel::new(
        TextView::new("Halaman Koefisien EEPROM").align(Align::center())
    ).full_screen();

    widget
}
