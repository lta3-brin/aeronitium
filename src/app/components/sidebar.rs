use cursive::Cursive;
use cursive::views::{Dialog, SelectView, StackView, PaddedView};
use cursive::align::HAlign;
use cursive::traits::{Resizable, Scrollable, Nameable};
use crate::app::pages::{
    firmware_page::build_firmware,
    connscanner_page::build_connscanner,
    daqparam_page::build_daqparam,
    scannerlist_page::build_scannerlist,
    coef_page::build_coefficient,
    unit_page::build_unit,
    status_page::build_status_scaner,
    printcoef_page::build_printcoef,
    stream_page::build_stream
};


pub fn build_sidebar() -> PaddedView<Dialog> {
    let widget = PaddedView::lrtb(
        0, 0, 1, 1,
        Dialog::around(
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
        ).title("AERONITIUM BBTA3")
    );

    widget
}

fn on_submit(s: &mut Cursive, index: &i32) {
    s.call_on_name("pilih_page", |view: &mut StackView| {
        match index {
            0 => {
                view.pop_layer();
                view.add_layer(
                    build_firmware()
                );
            },
            1 => {
                view.pop_layer();
                view.add_layer(
                    build_connscanner()
                );
            },
            2 => {
                view.pop_layer();
                view.add_layer(
                    build_daqparam()
                );
            },
            3 => {
                view.pop_layer();
                view.add_layer(
                    build_scannerlist()
                );
            },
            4 => {
                view.pop_layer();
                view.add_layer(
                    build_coefficient()
                );
            },
            5 => {
                view.pop_layer();
                view.add_layer(
                    build_unit()
                );
            },
            6 => {
                view.pop_layer();
                view.add_layer(
                    build_status_scaner()
                );
            },
            7 => {
                view.pop_layer();
                view.add_layer(
                    build_printcoef()
                );
            },
            8 => {
                view.pop_layer();
                view.add_layer(
                    build_stream()
                );
            },
            _ => {
                view.pop_layer();
            }
        };
    });
}
