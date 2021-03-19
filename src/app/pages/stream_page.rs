use cursive::views::{Panel, TextView, ResizedView};
use cursive::traits::Resizable;
use cursive::align::Align;


type StreamPage = ResizedView<Panel<TextView>>;

pub fn build_stream() -> StreamPage {
    let widget = Panel::new(
        TextView::new("Halaman Simulasi Tekanan").align(Align::center())
    ).full_screen();

    widget
}
