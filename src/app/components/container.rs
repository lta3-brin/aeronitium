use cursive::views::{LinearLayout, Dialog, NamedView};
use cursive::traits::{Resizable, Scrollable, Nameable};
use cursive::Cursive;


pub fn aeronitium_container<V, F>(
    view: V,
    callback: F
) -> NamedView<LinearLayout> where V: Resizable, F: 'static + Fn(&mut Cursive) {
    let widget = LinearLayout::vertical()
        .child(
            Dialog::new()
                .title("KONFIGURASI")
                .button("KIRIM", move |s| callback(s))
                .content(view)
                .full_screen()
                .scrollable()
        ).with_name("aeronitium_container");

    widget
}
