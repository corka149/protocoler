use chrono::prelude::*;
use std::fmt::{Display, Formatter};
use std::io;

// ===== ENTRY TYPE =====

#[derive(PartialEq, Eq)]
enum EntryType {
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

// ===== SELECTION =====

#[derive(PartialEq, Eq)]
enum Selection {
    Type(EntryType),
    Remove,
    Edit(usize),
    Quit,
    Invalid,
}

impl Selection {
    fn from_string(string: &str) -> Selection {
        match string.trim() {
            "i" => Selection::Type(EntryType::Info),
            "d" => Selection::Type(EntryType::Decision),
            "t" => Selection::Type(EntryType::Task),
            "r" => Selection::Remove,
            "q" => Selection::Quit,
            selection => match selection.parse::<usize>() {
                Ok(i) => Selection::Edit(i),
                Err(_) => Selection::Invalid,
            },
        }
    }
}

// ===== PROTOCOL ENTRY =====

pub struct ProtocolEntry {
    entry_type: EntryType,
    said_by: String,
    text: String,
    timestamp: DateTime<Local>,
}

impl ProtocolEntry {
    /// Creates a new protocol entry.
    fn new(entry_type: EntryType, said_by: String, text: String) -> ProtocolEntry {
        let timestamp = Local::now();

        ProtocolEntry {
            entry_type,
            said_by,
            text,
            timestamp,
        }
    }

    /// Creates a new protocol entry from stdin.
    fn from_input(entry_type: EntryType) -> io::Result<ProtocolEntry> {
        let said_by = input("---Said by:")?;
        let text = input("---Note:")?;

        Ok(ProtocolEntry::new(entry_type, said_by, text))
    }

    /// Updates a protocol entry from stdin.
    fn change_by_input(&mut self) {
        let prompt = format!("---Said by ['{}']:", self.said_by);

        match input(&prompt) {
            Ok(sb) => {
                // if empty - keep said_by
                if !sb.trim().is_empty() {
                    self.said_by = sb;
                }
            }
            Err(err) => println!("{}", err),
        };

        let prompt = format!("---Note ['{}']:", self.text);

        match input(&prompt) {
            Ok(new_note) => {
                // If empty - keep note
                if !new_note.trim().is_empty() {
                    self.text = new_note;
                }
            }
            Err(err) => println!("{}", err),
        };
    }
}

impl Display for ProtocolEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(
            f,
            "{} {} - {}: {}",
            timestamp, self.entry_type, self.said_by, self.text
        )
    }
}

// ===== RECORD =====

const USAGE: &str = "Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: ";

/// Start recording.
pub fn start() -> Vec<Option<ProtocolEntry>> {
    let mut entries: Vec<Option<ProtocolEntry>> = Vec::new();

    while let Ok(selection_str) = input(USAGE) {
        let selection = Selection::from_string(&selection_str);

        match selection {
            Selection::Type(entry_type) => match ProtocolEntry::from_input(entry_type) {
                Ok(entry) => entries.push(Some(entry)),
                Err(e) => println!("{}", e),
            },
            Selection::Edit(index) => entries = edit_entry(entries, index),
            Selection::Remove => entries = remove_entry(entries),
            Selection::Quit => break,
            Selection::Invalid => println!("I do not understand '{}'", selection_str),
        }
    }

    entries
}

/// Asks for an user for input.
fn input(prompt: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("{}", prompt);
    stdin.read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

/// Removes an entry by setting in None to keep the index stable.
fn remove_entry(mut entries: Vec<Option<ProtocolEntry>>) -> Vec<Option<ProtocolEntry>> {
    let index = input("---Delete an entry by ID:");

    if let Err(err) = index {
        println!("{}", err);
        return entries;
    }

    let index = index.unwrap();

    if let Ok(possible_index) = index.parse::<usize>() {
        let possible_index = possible_index - 1;

        match entries.get(possible_index) {
            Some(_) => entries[possible_index] = None,
            None => println!("'{}' no entry for removal", possible_index),
        }
    } else {
        println!("'{}' could not be recognized as an possible index", index);
    }

    entries
}

fn edit_entry(mut entries: Vec<Option<ProtocolEntry>>, index: usize) -> Vec<Option<ProtocolEntry>> {
    if index >= entries.len() {
        return entries;
    }

    let entry = &mut entries[index];

    match entry {
        Some(e) => e.change_by_input(),
        None => {
            println!("Entry was already deleted");
            return entries;
        }
    }

    entries
}
