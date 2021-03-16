use cursive::Cursive;
use cursive::views::{LinearLayout, DummyView};
use crate::app::AppError;
use crate::app::pages::root::root_page;
use crate::app::components::sidebar::build_sidebar;


pub fn build_terminal(s: &mut Cursive) -> Result<(), AppError> {
    s.add_layer(LinearLayout::horizontal()
        .child(build_sidebar())
        .child(DummyView)
        .child(DummyView)
        .child(root_page())
    );

    Ok(())
}
