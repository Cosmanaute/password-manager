extern crate cursive;
use cursive::Cursive;

mod tui;
use cursive::theme::{Color, PaletteColor};

fn main() {
    //initiating tui
    let mut app = cursive::default();
    //setting theme to terminal default
    let mut theme = app.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    app.set_theme(theme);
    //starting tui
    tui::start(&mut app); 
    app.run();
}
