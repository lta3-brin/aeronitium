use cursive::views::{LinearLayout, ListView, SelectView, TextView, NamedView};
use crate::app::components::container::aeronitium_container;
use cursive::With;
use crate::app::components::info::add_info;


pub fn build_firmware() -> NamedView<LinearLayout> {
    let widget = aeronitium_container(
        ListView::new()
            .child("CRS: ", SelectView::new()
                .popup()
                .with(|itm| {
                    for i in 111..119 {
                        let lbl = i.to_string();
                        let val = lbl.clone();

                        itm.add_item(lbl, val)
                    }
                })
            )
            .child("",
                   TextView::new("*Cluster Rack Server. Nilai awal 111")
            ),
        |s| {
            let messages = vec![
                ("code:", "Look-At (LAx) commands"),
                ("type:",
                 "Confirmation packet: indicates that command was successfully executed without error"
                ),
                ("status:", "Ok")
            ];

            s.call_on_name("aeronitium_container", |view: &mut LinearLayout| {
                view.add_child(add_info(messages))
            });
        }
    );

    widget
}
