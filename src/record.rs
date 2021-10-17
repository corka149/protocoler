use std::io;

#[derive(PartialEq, Eq)]
enum EntryType {
    Info,
    Decision,
    Task,
}

#[derive(Eq)]
enum Selection {
    Type(EntryType),
    Remove,
    Edit(usize),
    Quit,
    Invalid,
}

impl std::cmp::PartialEq for Selection {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

impl Selection {
    fn from_string(string: &str) -> Selection {
        match string {
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

struct ProtocolEntry {
    entry_type: EntryType,
    said_by: String,
    text: String,
    timestamp: u64,
}

impl ProtocolEntry {
    /// Creates a new protocol entry.
    fn new(entry_type: EntryType, said_by: String, text: String) -> ProtocolEntry {
        let timestamp = 1;
        return ProtocolEntry {
            entry_type,
            said_by,
            text,
            timestamp,
        };
    }

    /// Creates a new protocol entry from stdin.
    fn from_input(entry_type: EntryType) -> io::Result<ProtocolEntry> {
        let said_by = input("---Said by:")?;
        let text = input("---Note:")?;

        return Ok(ProtocolEntry::new(entry_type, said_by, text));
    }

    /// Updates a protocol entry from stdin.
    fn change_by_input(&mut self) {}
}

const USAGE: &str = "Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: ";

/// Start recording.
pub fn start() {
    let mut entries: Vec<ProtocolEntry> = Vec::new();

    while let Ok(selection_str) = input(USAGE) {
        let selection = Selection::from_string(&selection_str);

        match selection {
            Selection::Type(entry_type) => match ProtocolEntry::from_input(entry_type) {
                Ok(entry) => entries.push(entry),
                Err(e) => println!("{}", e),
            },
            Selection::Edit(index) => entries[index].change_by_input(),
            Selection::Remove => {}
            Selection::Quit => break,
            Selection::Invalid => println!("I do not understand '{}'", selection_str),
        }
    }

    println!("Done");
}

/// Asks for an user for input.
fn input(prompt: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("{}", prompt);
    stdin.read_line(&mut buffer)?;

    return Ok(buffer);
}
