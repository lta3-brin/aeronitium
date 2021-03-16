use cursive::Cursive;
use cursive::views::{Dialog, SelectView, StackView, LayerPosition};
use cursive::align::HAlign;
use cursive::traits::{Resizable, Scrollable, Nameable};


pub fn build_sidebar() -> Dialog {
    let widget = Dialog::around(
        SelectView::new().h_align(HAlign::Center)
            .item("Periksa Versi DTC", 0)
            .item("Koneksi Pemindai", 1)
            .item("Parameter Akuisisi", 2)
            .item("Daftar Pemindai", 3)
            .item("Koefisien EEPROM", 4)
            .item("Satuan Teknik", 5)
            .item("Status Pemindai", 6)
            .item("Cetak Koefisien", 7)
            .item("Simulasi Tekanan", 8)
            .on_submit(on_submit)
            .min_width(20)
            .with_name("pilih_prosedur")
            .scrollable()
    ).title("AERONITIUM BBTA3");

    widget
}

fn on_submit(s: &mut Cursive, index: &i32) {
    s.call_on_name("pilih_page", |view: &mut StackView| {
        view.move_to_front(LayerPosition::FromBack(*index as usize))
    });
}
