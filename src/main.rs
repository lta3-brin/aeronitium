mod app;

use cursive::theme::{PaletteColor, Color, BorderStyle};
use crate::app::AppError;
use crate::app::layouts::default::build_terminal;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::View] = Color::TerminalDefault;
    theme.palette[PaletteColor::Primary] = Color::Rgb(166, 126, 123);
    theme.shadow = false;
    theme.borders = BorderStyle::Simple;

    siv.set_theme(theme);

    build_terminal(&mut siv)?;

    siv.run();

    Ok(())
}
