use cursive::traits::*;
use cursive::views::{Dialog, EditView, Panel, TextArea};
use cursive_table_view::TableView;

use crate::{BasicColumn, EntryType, LinearLayout, ProtocolEntry, table};

const DIALOG_WIDTH: usize = 70;

// ===== ===== ADD DIALOG ===== =====

pub fn add_dialog() -> Dialog {
    let content = add_dialog_content();

    Dialog::around(content)
        .title("Add")
        .button("Save", move |s| {
            // GET FIELDS
            let owner = s.find_name::<EditView>("owner").map(|e| e.get_content().to_string()).unwrap_or_default();
            let message = s.find_name::<TextArea>("message").map(|t| t.get_content().to_string()).unwrap_or_default();

            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                add_entry(table, owner, message);
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}

fn add_entry(table: &mut TableView<ProtocolEntry, BasicColumn>, owner: String, message: String) {
    table.insert_item(ProtocolEntry::new(EntryType::Info, owner, message));
}

fn add_dialog_content() -> LinearLayout {
    LinearLayout::vertical()
        .child(
            // OWNER
            Panel::new(
                EditView::default()
                    .content("")
                    .with_name("owner")
            ).title("Owner").min_width(DIALOG_WIDTH)
        )
        .child(
            // MESSAGE
            Panel::new(
                TextArea::default()
                    .content("")
                    .with_name("message")
                    .min_height(10)
            ).title("Message").min_width(DIALOG_WIDTH)
        )
}

// ===== ===== EDIT DIALOG ===== =====

pub fn edit_dialog(entry: &ProtocolEntry) -> Dialog {
    let content = edit_dialog_content(entry);

    Dialog::around(content)
        .title("Edit")
        .button("Save", move |s| {
            // GET FIELDS
            let owner = s.find_name::<EditView>("owner").map(|e| e.get_content().to_string()).unwrap_or_default();
            let message = s.find_name::<TextArea>("message").map(|t| t.get_content().to_string()).unwrap_or_default();

            s.call_on_name(table::table_name(), |t: &mut TableView<ProtocolEntry, BasicColumn>| {
                edit_entry(t, owner, message);
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}

fn edit_dialog_content(entry: &ProtocolEntry) -> LinearLayout {
    LinearLayout::vertical()
        .child(
            // OWNER
            Panel::new(
                EditView::default()
                    .content(entry.owner.clone())
                    .with_name("owner")
            ).title("Owner").min_width(DIALOG_WIDTH)
        )
        .child(
            // MESSAGE
            Panel::new(
                TextArea::default()
                    .content(entry.message.clone())
                    .with_name("message")
                    .min_height(10)
            ).title("Message").min_width(DIALOG_WIDTH)
        )
}

fn edit_entry(table: &mut TableView<ProtocolEntry, BasicColumn>, owner: String, message: String) {
    if let Some(old) = table::get_current_item(table) {
        let new = old.clone().message(message).owner(owner);

        if let Some(index) = table.item() {
            table.remove_item(index);
            table.insert_item_at(index, new);
        }
    }
}
