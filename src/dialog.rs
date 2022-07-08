use cursive::Cursive;
use cursive::views::{Dialog, TextView, ViewRef};
use cursive_table_view::TableView;

use crate::{BasicColumn, ProtocolEntry, table};

pub fn add_dialog(entry: ProtocolEntry) -> Dialog {
    Dialog::around(TextView::new("Hello Dialog!"))
        .title("Add")
        .button("Quit", |s| {
            s.pop_layer();
        })
}

pub fn edit_dialog(entry: ProtocolEntry, index: usize) -> Dialog {
    Dialog::around(TextView::new("Hello Dialog!"))
        .title("Edit")
        .button("Save", move |s| {
            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                let mut entry = entry.clone();
                entry.set_message("Another".to_string());

                table.remove_item(index);
                table.insert_item_at(index, entry);
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}
