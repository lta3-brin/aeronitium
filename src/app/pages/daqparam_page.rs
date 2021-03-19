use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type DaqParamPage = ResizedView<Panel<TextView>>;

pub fn build_daqparam() -> DaqParamPage {
    let widget = Panel::new(
        TextView::new("Halaman Akuisisi Parameter").align(Align::center())
    ).full_screen();

    widget
}
