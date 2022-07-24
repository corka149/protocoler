//! A minimalistic typer for protocols.

use cursive::{Cursive, CursiveRunnable};
use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{LinearLayout, Panel};

use crate::cli::Cli;
use crate::table::{BasicColumn, EntryType, ProtocolEntry, ProtocolTable};

mod dialog;
mod error;
mod help;
mod persist;
mod report;
mod style;
mod table;
mod util;
mod cli;

const DIALOG_NAME: &str = "app_dialog";

/// MAIN
fn main() {
    let cli = cli::parse();

    if cli.should_launch_subcommand() {
        unimplemented!()
    } else {
        launch_tui(cli);
    }
}

///
fn launch_tui(cli: Cli) {
    let mut app = cursive::default();
    let table = table::new();

    let full_view = LinearLayout::vertical()
        .child(table.with_name(table::table_name()).full_height())
        .child(help::hint_bar())
        .full_screen();

    app.add_fullscreen_layer(Panel::new(full_view).title("Protocoler").full_screen());

    add_callbacks(&mut app);

    if !cli.no_theme {
        app.update_theme(style::set_default_style);
    }

    app.run();

    save_before_exit(&mut app, &cli);
}

/// Central place for adding callbacks.
fn add_callbacks(app: &mut CursiveRunnable) {
    // General actions
    app.add_global_callback('q', |app| app.quit());
    app.add_global_callback('x', |app| {
        app.add_layer(help::help_menu().with_name(DIALOG_NAME))
    });
    app.add_global_callback('s', |app| {
        let content = persist::get_target_path(app)
            .map(|path| {
                path.to_str()
                    .map(|path_str| path_str.to_string())
                    .unwrap_or_default()
            })
            .unwrap_or_default();

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

/// Tries to save last state of the table.
/// It will use either the entered path or fallback to a temporary CSV path.
fn save_before_exit(app: &mut CursiveRunnable, cli: &Cli) {
    if persist::get_target_path(app).is_none() && cli.disable_autosave {
        return;
    }

    let tar_path = persist::get_target_path(app)
        .map(Ok)
        .unwrap_or_else(util::tmp_csv_path);

    app.call_on_name(table::table_name(), |table: &mut ProtocolTable| {
        let entries = table.borrow_items();

        if let Ok(tar_path) = tar_path {
            if let Err(err) = report::save(entries, &tar_path) {
                eprintln!("{}", err);
            } else if let Some(path_str) = tar_path.to_str() {
                println!("Saved protocol to temp file {}", path_str);
            }
        }
    });
}

/// Determines whether a closable dialog is open.
fn is_dialog_open(app: &mut Cursive) -> bool {
    app.debug_name(DIALOG_NAME).is_some()
}
