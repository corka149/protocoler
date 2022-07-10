use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, EditView, Panel, SelectView, TextArea, ViewRef};
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
            let entry_type = s.find_name::<SelectView<EntryType>>("entry_type").map(get_selected_type).unwrap_or_default();

            s.call_on_name(table::table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
                add_entry(table, entry_type, owner, message);
            });
            s.pop_layer();
        })
        .button("Quit", |s| {
            s.pop_layer();
        })
}

fn add_entry(table: &mut TableView<ProtocolEntry, BasicColumn>, entry_type: EntryType, owner: String, message: String) {
    table.insert_item(ProtocolEntry::new(entry_type, owner, message));
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
        .child(
            type_select(None).with_name("type_select")
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
            let entry_type = s.find_name::<SelectView<EntryType>>("entry_type").map(get_selected_type).unwrap_or_default();

            s.call_on_name(table::table_name(), |t: &mut TableView<ProtocolEntry, BasicColumn>| {
                edit_entry(t, entry_type, owner, message);
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
        .child(
            type_select(Some(entry.entry_type)).with_name("type_select")
        )
}

fn edit_entry(table: &mut TableView<ProtocolEntry, BasicColumn>, entry_type: EntryType, owner: String, message: String) {
    if let Some(old) = table::get_current_item(table) {
        let new = old.clone().entry_type(entry_type).message(message).owner(owner);

        if let Some(index) = table.item() {
            table.remove_item(index);
            table.insert_item_at(index, new);
        }
    }
}

// ===== ===== TYPE SELECT ===== =====

fn type_select(selected: Option<EntryType>) -> SelectView<EntryType> {
    let select: SelectView<EntryType> = SelectView::new()
        .item(EntryType::Info.to_string(), EntryType::Info)
        .item(EntryType::Task.to_string(), EntryType::Task)
        .item(EntryType::Decision.to_string(), EntryType::Decision)
        .h_align(HAlign::Center);

    if let Some(selected) = selected {
        match selected {
            EntryType::Info => select.selected(0),
            EntryType::Task => select.selected(1),
            EntryType::Decision => select.selected(2)
        }
    } else {
        select
    }
}

fn get_selected_type(s: ViewRef<SelectView<EntryType>>) -> EntryType {
    let id = s.selected_id().unwrap_or(0);
    let maybe_type: Option<(&str, &EntryType)> = s.get_item(id);

    match maybe_type {
        None => EntryType::default(),
        Some((_, selected_type)) => selected_type.to_owned()
    }
}
