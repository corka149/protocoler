extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;

use cursive::{Cursive, CursiveRunnable};
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{DebugView, LinearLayout, Panel};
use cursive_table_view::TableView;

use crate::table::{BasicColumn, EntryType, ProtocolEntry};

mod style;
mod table;
mod help;
mod dialog;

const DIALOG_NAME: &'static str = "data_dialog";

/// MAIN
fn main() {
    let mut app = cursive::default();

    let mut table = table::new();
    let debug_view = DebugView::default();

    dummy_data(&mut table);

    let full_view = LinearLayout::vertical()
        .child(table.with_name(table::table_name()).full_height())
        .child(help::hint_bar())
        .child(debug_view)
        .full_screen();

    app.add_fullscreen_layer(
        Panel::new(full_view)
            .title("Protocoler")
            .full_screen(),
    );

    add_callbacks(&mut app);

    app.update_theme(style::set_default_style);

    app.run();
}

fn add_callbacks(app: &mut CursiveRunnable) {
    // General actions
    app.add_global_callback('q', |s| s.quit());
    app.add_global_callback('x', |s| s.add_layer(help::help_menu()));
    app.add_global_callback(Event::Key(Key::Esc), |app| {
        if is_dialog_open(app) {
            app.pop_layer();
        }
    });

    // Table Actions
    app.add_global_callback('a', |app| {
        if !is_dialog_open(app) {
            table::add_entry(app, DIALOG_NAME)
        }
    });
    app.add_global_callback('e', |app| {
        if !is_dialog_open(app) {
            table::edit_entry(app, DIALOG_NAME)
        }
    });
    app.add_global_callback('d', |app| {
        if !is_dialog_open(app) {
            table::delete_entry(app)
        }
    });
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

fn is_dialog_open(app: &mut Cursive) -> bool {
    app.debug_name(DIALOG_NAME).is_some()
}
