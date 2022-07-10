//! Main manu for protocol entries.

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use chrono::prelude::*;
use cursive::Cursive;
use cursive::views::ViewRef;
use cursive_table_view::{TableView, TableViewItem};

use crate::dialog;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Timestamp,
    Type,
    Owner,
    Message,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum EntryType {
    Info,
    Decision,
    Task,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Info => write!(f, "INFO"),
            EntryType::Decision => write!(f, "DECISION"),
            EntryType::Task => write!(f, "TASK"),
        }
    }
}

impl Default for EntryType {
    fn default() -> Self {
        EntryType::Info
    }
}

#[derive(Clone, Debug)]
pub struct ProtocolEntry {
    pub timestamp: DateTime<Local>,
    pub entry_type: EntryType,
    pub owner: String,
    pub message: String,
}

impl ProtocolEntry {
    /// Creates a new protocol entry.
    pub fn new(entry_type: EntryType, owner: String, message: String) -> ProtocolEntry {
        let timestamp = Local::now();

        ProtocolEntry {
            timestamp,
            entry_type,
            owner,
            message,
        }
    }

    pub fn entry_type(mut self, entry_type: EntryType) -> Self {
        self.entry_type = entry_type;
        self
    }

    pub fn owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

impl TableViewItem<BasicColumn> for ProtocolEntry {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Timestamp => self.timestamp.to_string(),
            BasicColumn::Type => self.entry_type.to_string(),
            BasicColumn::Owner => self.owner.clone(),
            BasicColumn::Message => self.message.clone(),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
        where
            Self: Sized,
    {
        match column {
            BasicColumn::Timestamp => self.timestamp.cmp(&other.timestamp),
            BasicColumn::Type => self.entry_type.cmp(&other.entry_type),
            BasicColumn::Owner => self.owner.cmp(&other.owner),
            BasicColumn::Message => self.message.cmp(&other.message),
        }
    }
}

// ===== ===== module ===== =====

const TABLE_NAME: &str = "main_table";

/// Global table name.
pub fn table_name() -> &'static str {
    TABLE_NAME
}

/// Creates a new table for protocol entries.
pub fn new() -> TableView<ProtocolEntry, BasicColumn> {
    TableView::<ProtocolEntry, BasicColumn>::new()
        .column(BasicColumn::Timestamp, "Timestamp", |c| c.width_percent(18))
        .column(BasicColumn::Owner, "Owner", |c| c.width_percent(18))
        .column(BasicColumn::Type, "Type", |c| c.width_percent(8))
        .column(BasicColumn::Message, "Message", |c| c.width_percent(56))
}

/// Delete an entry.
pub fn delete_entry(siv: &mut Cursive) {
    siv.call_on_name(table_name(), |table: &mut TableView<ProtocolEntry, BasicColumn>| {
        if let Some(index) = table.row() {
            table.remove_item(index);
        }
    });
}

/// Edit an entry.
pub fn add_entry(siv: &mut Cursive) {
    siv.add_layer(dialog::add_dialog());
}

/// Add a new entry.
pub fn edit_entry(siv: &mut Cursive) {
    let table: Option<ViewRef<TableView<ProtocolEntry, BasicColumn>>> = siv.find_name(table_name());
    if table.is_none() {
        return;
    }
    let table: ViewRef<TableView<ProtocolEntry, BasicColumn>> = table.unwrap();

    if let Some(entry) = get_current_item(&table) {
        siv.add_layer(dialog::edit_dialog(entry));
    }
}

pub fn get_current_item(table: &TableView<ProtocolEntry, BasicColumn>) -> Option<&ProtocolEntry> {
    if let Some(index) = table.item() {
        if let Some(entry) = table.borrow_item(index) {
            return Some(entry);
        }
    }

    return None;
}
