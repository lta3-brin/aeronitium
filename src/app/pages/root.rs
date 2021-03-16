use cursive::views::{TextView, StackView, NamedView, Dialog};
use cursive::traits::{Resizable, Nameable};
use cursive::align::Align;


pub fn root_page() -> NamedView<StackView> {
    let widget = StackView::new()
        .layer(
            Dialog::around(TextView::new(
                "Pilih prosedur yang telah disediakan disamping. \
                Lakukanlah secara berurutan. Tekan Enter atau klik kanan mouse untuk memilih."
            ).align(Align::center())).full_screen()
        )
        .with_name("pilih_page");

    widget
}
