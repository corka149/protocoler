//! Main manu for protocol entries.

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::Index;

use chrono::prelude::*;
use cursive::traits::*;
use cursive::views::ViewRef;
use cursive::Cursive;
use cursive_table_view::{TableView, TableViewItem};

use crate::dialog;
use crate::error::InvalidCsvError;

pub type ProtocolTable = TableView<ProtocolEntry, BasicColumn>;

/// Columns of protocol entry table.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Timestamp,
    Type,
    Owner,
    Message,
}

/// Category of protocol entries.
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

impl From<&str> for EntryType {
    fn from(text: &str) -> Self {
        match text {
            "INFO" => EntryType::Info,
            "DECISION" => EntryType::Decision,
            "TASK" => EntryType::Task,
            _ => EntryType::Info,
        }
    }
}

impl Default for EntryType {
    fn default() -> Self {
        EntryType::Info
    }
}

/// ProtocolEntry holds the details of a protocol note.
#[derive(Clone, Debug)]
pub struct ProtocolEntry {
    pub timestamp: DateTime<Local>,
    pub entry_type: EntryType,
    pub owner: String,
    pub message: String,
}

impl ProtocolEntry {
    pub const CSV_HEADER: &'static str = "timestamp,entry_type,owner,message";

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

    /// Creates a CSV row of the protocol entry.
    pub fn as_csv(&self) -> String {
        format!(
            "'{}','{}','{}','{}'",
            self.timestamp, self.entry_type, self.owner, self.message
        )
    }

    /// Creates a entry from a CSV row.
    pub fn from_csv(csv: &str) -> Result<Self, InvalidCsvError> {
        let split: Vec<&str> = csv
            .trim_start_matches('\'')
            .trim_end_matches('\'')
            .split("','")
            .collect::<Vec<&str>>();

        if split.len() != 4 {
            return Err(InvalidCsvError);
        }

        let timestamp = split.index(0);
        let timestamp = timestamp.parse::<DateTime<Local>>();

        if let Ok(timestamp) = timestamp {
            Ok(ProtocolEntry {
                timestamp,
                entry_type: EntryType::from(*split.index(1)),
                owner: split.index(2).to_string(),
                message: split.index(3).to_string(),
            })
        } else {
            Err(InvalidCsvError)
        }
    }
}

impl Display for ProtocolEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(
            f,
            "{} {} - {}: {}",
            timestamp, self.entry_type, self.owner, self.message
        )
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
pub fn new() -> ProtocolTable {
    ProtocolTable::new()
        .column(BasicColumn::Timestamp, "Timestamp", |c| c.width_percent(18))
        .column(BasicColumn::Owner, "Owner", |c| c.width_percent(18))
        .column(BasicColumn::Type, "Type", |c| c.width_percent(8))
        .column(BasicColumn::Message, "Message", |c| c.width_percent(56))
}

/// Delete an entry.
pub fn delete_entry(siv: &mut Cursive) {
    siv.call_on_name(table_name(), |table: &mut ProtocolTable| {
        if let Some(index) = table.row() {
            table.remove_item(index);
        }
    });
}

/// Edit an entry.
pub fn add_entry(siv: &mut Cursive, name: &str) {
    siv.add_layer(dialog::add_dialog().with_name(name));
}

/// Add a new entry.
pub fn edit_entry(siv: &mut Cursive, name: &str) {
    let table: Option<ViewRef<ProtocolTable>> = siv.find_name(table_name());
    if table.is_none() {
        return;
    }
    let table: ViewRef<ProtocolTable> = table.unwrap();

    if let Some(entry) = get_current_item(&table) {
        siv.add_layer(dialog::edit_dialog(entry).with_name(name));
    }
}

/// Returns the selected protocol entry.
pub fn get_current_item(table: &ProtocolTable) -> Option<&ProtocolEntry> {
    if let Some(index) = table.item() {
        if let Some(entry) = table.borrow_item(index) {
            return Some(entry);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_to_and_from_csv() {
        // Arrange
        let entry = ProtocolEntry::new(
            EntryType::Info,
            "Alice".to_string(),
            "Sleeps or something like that".to_string()
        );

        // Act
        let row = entry.as_csv();
        let parsed = ProtocolEntry::from_csv(&row);

        // Assert
        assert!(parsed.is_ok());

        let parsed_entry = parsed.unwrap();

        assert_eq!(entry.entry_type, parsed_entry.entry_type);
        assert_eq!(entry.owner, parsed_entry.owner);
        assert_eq!(entry.message, parsed_entry.message);
        assert_eq!(entry.timestamp, parsed_entry.timestamp);
    }
}
