extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;

mod style;
mod table;

use cursive::traits::*;
use cursive::views::Dialog;

use crate::table::{EntryType, ProtocolEntry};


/// MAIN
fn main() {
    let mut siv = cursive::default();

    let mut table = table::new();

    table.insert_item(ProtocolEntry::new(
        EntryType::Info, "Alice".to_string(), "Let`s go".to_string()
    ));

    siv.add_fullscreen_layer(
        Dialog::around(table)
            .title("Protocoler")
            .full_screen(),
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.update_theme(style::set_terminal_default);

    siv.run();
}
