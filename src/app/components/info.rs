use cursive::views::{Dialog, ListView, DummyView, TextView, ScrollView, ResizedView};
use cursive::traits::{Resizable, Scrollable};
use cursive::With;


type Info = ScrollView<ResizedView<ResizedView<Dialog>>>;

pub fn add_info(messages: Vec<(&str, &str)>) -> Info {
    let widget = Dialog::around(
        ListView::new()
            .child("Pesan", DummyView)
            .with(|lst| {
                for ms in messages {
                    lst.add_child(ms.0, TextView::new(ms.1))
                }
            })
    ).title("INFORMASI")
        .fixed_height(10)
        .full_width()
        .scrollable();

    widget
}
