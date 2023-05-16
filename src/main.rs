extern crate cursive;
use cursive::view::Resizable;
use cursive::views::{Dialog, TextView};
use cursive::event::Key;

mod tui;
use cursive::theme::{Color, PaletteColor};

fn main() {
    //initiating tui
    let mut app = cursive::default();
    //setting theme to terminal default
    let mut theme = app.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    app.set_theme(theme);
    //setting global key to quit
    app.add_global_callback(Key::Esc, |s| {s.add_layer(Dialog::around(TextView::new(
        "Are you sure you want to exit?")).title("Exit")
        .button("Cancel", |s| {s.pop_layer();})
        .button("Exit", |s| s.quit()).min_width(30).min_height(8)
    )} );
    //starting tui
    tui::start(&mut app); 
    app.run();
}
