use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type EngUnitPage = ResizedView<Panel<TextView>>;

pub fn build_unit() -> EngUnitPage {
    let widget = Panel::new(
        TextView::new("Halaman Satuan Teknik").align(Align::center())
    ).full_screen();

    widget
}
