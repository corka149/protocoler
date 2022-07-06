extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;

use cursive::traits::*;
use cursive::views::{DebugView, Dialog, LinearLayout};
use cursive_table_view::TableView;

use crate::table::{BasicColumn, EntryType, ProtocolEntry};

mod style;
mod table;
mod help;

/// MAIN
fn main() {
    let mut siv = cursive::default();

    let mut table = table::new();
    let debug_view = DebugView::default();

    dummy_data(&mut table);

    let full_view = LinearLayout::vertical()
        .child(table.with_name(table::table_name()).full_height())
        .child(help::hint_bar())
        .child(debug_view)
        .full_screen();

    siv.add_fullscreen_layer(
        Dialog::around(full_view)
            .title("Protocoler")
            .full_screen(),
    );

    // General actions
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('x', |s| s.add_layer(help::help_menu()));

    // Table Actions
    siv.add_global_callback('a', table::data_dialog);
    siv.add_global_callback('e', table::data_dialog);
    siv.add_global_callback('d', table::delete_entry);

    siv.update_theme(style::set_default_style);

    siv.run();
}

fn dummy_data(table: &mut TableView<ProtocolEntry, BasicColumn>) {
    table.insert_item(ProtocolEntry::new(
        EntryType::Info, "Alice".to_string(), "Let`s go".to_string(),
    ));
    table.insert_item(ProtocolEntry::new(
        EntryType::Decision, "Bob".to_string(), "Are we done?".to_string(),
    ));
    table.insert_item(ProtocolEntry::new(
        EntryType::Task, "Ceasar".to_string(), "yep yep yep".to_string(),
    ));
}
