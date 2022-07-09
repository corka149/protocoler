use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, Panel, TextArea, TextView, ViewRef};
use cursive_table_view::TableView;

use crate::{BasicColumn, LinearLayout, ProtocolEntry, table};

const DIALOG_WIDTH: usize = 70;

pub fn add_dialog(entry: ProtocolEntry) -> Dialog {
    Dialog::around(TextView::new("Hello Dialog!"))
        .title("Add")
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
            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                // GET ITEM

                // CHANGE ITEM

                // REPLACE ITEM IN TABLE
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}
