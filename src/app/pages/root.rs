use cursive::views::{TextView, StackView, NamedView, Dialog};
use cursive::traits::{Resizable, Nameable};
use cursive::align::Align;


pub fn root_page() -> NamedView<StackView> {
    let banner = r#"
██████╗ ██████╗ ████████╗ █████╗ ██████╗
██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗╚════██╗
██████╔╝██████╔╝   ██║   ███████║ █████╔╝
██╔══██╗██╔══██╗   ██║   ██╔══██║ ╚═══██╗
██████╔╝██████╔╝   ██║   ██║  ██║██████╔╝
╚═════╝ ╚═════╝    ╚═╝   ╚═╝  ╚═╝╚═════╝
    "#;
    let title = "Pilih prosedur yang telah disediakan disamping. \
    Lakukanlah secara berurutan. Tekan Enter atau klik kiri mouse untuk memilih.";

    let widget = StackView::new()
        .layer(
            Dialog::around(TextView::new(
                format!("{}\n{}", banner, title)
            ).align(Align::center())).full_screen()
        )
        .with_name("pilih_page");

    widget
}
