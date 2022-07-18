extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;

use cursive::{Cursive, CursiveRunnable};
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{LinearLayout, Panel};
use cursive_table_view::TableView;

use crate::table::{BasicColumn, EntryType, ProtocolEntry, ProtocolTable, table_name};

mod style;
mod table;
mod help;
mod dialog;
mod report;
mod persist;
mod util;
mod error;

const DIALOG_NAME: &'static str = "app_dialog";

/// MAIN
fn main() {
    let mut app = cursive::default();

    let mut table = table::new();

    dummy_data(&mut table);

    let full_view = LinearLayout::vertical()
        .child(table.with_name(table::table_name()).full_height())
        .child(help::hint_bar())
        .full_screen();

    app.add_fullscreen_layer(
        Panel::new(full_view)
            .title("Protocoler")
            .full_screen(),
    );

    add_callbacks(&mut app);

    app.update_theme(style::set_default_style);

    app.run();

    save_before_exit(&mut app);
}

fn add_callbacks(app: &mut CursiveRunnable) {
    // General actions
    app.add_global_callback('q', |app| app.quit());
    app.add_global_callback('x', |app| {
        app.add_layer(help::help_menu().with_name(DIALOG_NAME))
    });
    app.add_global_callback('s', | app| {
        let content = persist::get_target_path(app)
            .map(
                |path|
                    path.to_str().map(
                        |path_str| path_str.to_string()
                    ).unwrap_or_default()
            ).unwrap_or_default();

        app.add_layer(persist::save_dialog(content).with_name(DIALOG_NAME))
    });
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

fn save_before_exit(app: &mut CursiveRunnable) {
    let has_path = persist::get_target_path(app).is_some();

    if has_path {
        return;
    }

    app.call_on_name(table_name(), |table: &mut ProtocolTable| {
        let entries = table.borrow_items();

        if let Ok(tmp_csv_path) = util::tmp_csv_path() {
            if let Err(err) = report::save_csv(entries, &tmp_csv_path) {
                eprintln!("{}", err);
            } else {
                if let Some(path_str) = tmp_csv_path.to_str() {
                    println!("Saved protocol to temp file {}", path_str);
                }
            }
        };
    });
}

fn is_dialog_open(app: &mut Cursive) -> bool {
    app.debug_name(DIALOG_NAME).is_some()
}
