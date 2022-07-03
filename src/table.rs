use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use chrono::prelude::*;
use cursive_table_view::{TableView, TableViewItem};

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

#[derive(Clone, Debug)]
pub struct ProtocolEntry {
    timestamp: DateTime<Local>,
    entry_type: EntryType,
    owner: String,
    message: String,
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

pub fn new() -> TableView<ProtocolEntry, BasicColumn> {
    TableView::<ProtocolEntry, BasicColumn>::new()
        .column(BasicColumn::Timestamp, "Timestamp", |c| c.width_percent(18))
        .column(BasicColumn::Owner, "Owner", |c| c.width_percent(18))
        .column(BasicColumn::Type, "Type", |c| c.width_percent(8))
        .column(BasicColumn::Message, "Message", |c| c.width_percent(56))
}