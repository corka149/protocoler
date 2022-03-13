//! `protocol` creates protocol reports.
use super::record::{EntryType, ProtocolEntry};
use super::util::*;
use chrono::prelude::*;
use std::process::exit;

const RAW_FORMAT: &str = "raw";
const MARKDOWN_FORMAT: &str = "markdown";
const CSV_FORMAT: &str = "csv";
const ALLOWED_FORMATS: [&str; 3] = [RAW_FORMAT, MARKDOWN_FORMAT, CSV_FORMAT];

/// Outputs the protocol entries in different formats.
pub fn output(participants: Vec<String>, protocol_entries: Vec<ProtocolEntry>) {
    match select_format().as_str() {
        RAW_FORMAT => print_raw(participants, protocol_entries),
        MARKDOWN_FORMAT => print_markdown(participants, protocol_entries),
        CSV_FORMAT => print_csv(protocol_entries),
        unknown => {
            eprintln!("Unknown format'{}'", unknown);
            exit(1)
        }
    }
}

fn select_format() -> String {
    loop {
        match input("Select output format [raw, markdown, csv]") {
            Ok(format) => {
                let is_allowed = ALLOWED_FORMATS.iter().any(|f| f == &format);

                if is_allowed {
                    return format;
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}

fn print_raw(participants: Vec<String>, protocol_entries: Vec<ProtocolEntry>) {
    println!("Participants: {:?}", participants);

    for e in protocol_entries {
        println!("{}", e);
    }
}

fn print_markdown(participants: Vec<String>, protocol_entries: Vec<ProtocolEntry>) {
    let mut infos: Vec<ProtocolEntry> = Vec::new();
    let mut decisions: Vec<ProtocolEntry> = Vec::new();
    let mut tasks: Vec<ProtocolEntry> = Vec::new();

    for e in protocol_entries {
        match e.entry_type() {
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
            e.timestamp().format("%H:%M:%S"),
            e.said_by(),
            e.text(),
        )
    });

    println!("---\n## Decisions\n");
    decisions.iter().for_each(|e| {
        println!(
            "* <> {} - {}/{}",
            e.text(),
            e.said_by(),
            e.timestamp().format("%H:%M:%S")
        )
    });

    println!("---\n## Tasks\n");
    tasks.iter().for_each(|e| {
        println!(
            "* [] {} - {}/{}",
            e.text(),
            e.said_by(),
            e.timestamp().format("%H:%M:%S")
        )
    });
}

fn print_csv(protocol_entries: Vec<ProtocolEntry>) {
    println!("{}", ProtocolEntry::CSV_HEADER);
    for e in protocol_entries {
        println!("{}", e.as_csv());
    }
}
