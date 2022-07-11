use std::fmt::{Display, Formatter};
use chrono::Local;
use crate::{EntryType, ProtocolEntry};


fn print_raw(protocol_entries: Vec<ProtocolEntry>) {
    for e in protocol_entries {
        println!("{}", e);
    }
}

fn print_csv(protocol_entries: Vec<ProtocolEntry>) {
    println!("{}", ProtocolEntry::CSV_HEADER);
    for e in protocol_entries {
        println!("{}", e.as_csv());
    }
}

fn print_markdown(protocol_entries: Vec<ProtocolEntry>, entry_type: EntryType) {
    let mut infos: Vec<ProtocolEntry> = Vec::new();
    let mut decisions: Vec<ProtocolEntry> = Vec::new();
    let mut tasks: Vec<ProtocolEntry> = Vec::new();
    let participants = collect_participants(&protocol_entries);

    for e in protocol_entries {
        match entry_type {
            EntryType::Info => infos.push(e),
            EntryType::Decision => decisions.push(e),
            EntryType::Task => tasks.push(e),
        }
    }

    println!("# Protocol {}", Local::now().format("%Y-%m-%d"));

    println!("\n## Participants\n");
    participants
        .iter()
        .for_each(|participant| println!("* {}", participant));

    println!("\n## Information\n");
    println!("|Time|Said by|text|");
    println!("| --- | --- | ---|");
    infos.iter().for_each(|e| {
        println!(
            "|{}|{}|{}|",
            e.timestamp.format("%H:%M:%S"),
            e.owner,
            e.message,
        )
    });

    println!("---\n## Decisions\n");
    decisions.iter().for_each(|e| {
        println!(
            "* <> {} - {}/{}",
            e.message,
            e.owner,
            e.timestamp.format("%H:%M:%S")
        )
    });

    println!("---\n## Tasks\n");
    tasks.iter().for_each(|e| {
        println!(
            "* [] {} - {}/{}",
            e.message,
            e.owner,
            e.timestamp.format("%H:%M:%S")
        )
    });
}

fn collect_participants(protocol_entries: &Vec<ProtocolEntry>) -> Vec<String> {
    protocol_entries
        .iter()
        .map(|e| e.owner.clone())
        .collect::<Vec<String>>()
}
