extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;

use cursive::traits::*;
use cursive::views::{Dialog, ListView, TextView};
use cursive_table_view::TableView;

mod style;
mod table;

/// MAIN
fn main() {
    let mut siv = cursive::default();

    let mut table = TableView::<table::EntryType, table::BasicColumn>::new();


    let select_view = ListView::new()
        .child("Label", TextView::new("TextView"));

    siv.add_fullscreen_layer(
        Dialog::around(select_view)
            .title("Protocoler")
            .full_screen(),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.update_theme(style::set_terminal_default);

    siv.run();
}
