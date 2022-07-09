use cursive::traits::*;
use cursive::views::{Dialog, EditView, Panel, TextArea};
use cursive_table_view::TableView;

use crate::{BasicColumn, EntryType, LinearLayout, ProtocolEntry, table};

const DIALOG_WIDTH: usize = 70;

pub fn add_dialog() -> Dialog {
    let content = LinearLayout::vertical()
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
        ;

    Dialog::around(content)
        .title("Add")
        .button("Save", move |s| {
            // GET FIELDS
            let owner = s.find_name::<EditView>("owner").map(|e| e.get_content().to_string()).unwrap_or_default();
            let message = s.find_name::<TextArea>("message").map(|t| t.get_content().to_string()).unwrap_or_default();

            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                // CREATE ITEM
                let new = ProtocolEntry::new(EntryType::Info, owner, message);

                // ADD ITEM TO TABLE
                table.insert_item( new);
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}

pub fn edit_dialog(entry: &ProtocolEntry) -> Dialog {
    let content = LinearLayout::vertical()
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
        ;

    Dialog::around(content)
        .title("Edit")
        .button("Save", move |s| {
            // GET FIELDS
            let owner = s.find_name::<EditView>("owner").map(|e| e.get_content().to_string()).unwrap_or_default();
            let message = s.find_name::<TextArea>("message").map(|t| t.get_content().to_string()).unwrap_or_default();

            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                if let Some(old) = table::get_current_item(table) {
                    // CHANGE ITEM
                    let new = old.clone().message(message).owner(owner);

                    // REPLACE ITEM IN TABLE
                    if let Some(index) = table.item() {
                        table.remove_item(index);
                        table.insert_item_at(index, new);
                    }
                }
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}
