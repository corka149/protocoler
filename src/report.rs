use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::Local;

use crate::{EntryType, ProtocolEntry};

pub fn save_raw(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut text_file = File::create(&path)?;

    let participants = collect_participants(&protocol_entries);
    println!("Participants: {:?}", participants);

    for e in protocol_entries {
        text_file.write_all(format!("{}\n", e).as_bytes())?
    }

    return Ok(());
}

pub fn save_csv(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut csv = File::create(&path)?;

    csv.write_all(format!("{}\n", ProtocolEntry::CSV_HEADER).as_bytes())?;
    for e in protocol_entries {
        csv.write_all(format!("{}\n", e.as_csv()).as_bytes())?;
    }

    Ok(())
}

pub fn save_markdown(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut md_file = File::create(&path)?;
    let mut infos: Vec<&ProtocolEntry> = Vec::new();
    let mut decisions: Vec<&ProtocolEntry> = Vec::new();
    let mut tasks: Vec<&ProtocolEntry> = Vec::new();
    let participants = collect_participants(&protocol_entries);

    for e in protocol_entries {
        match e.entry_type {
            EntryType::Info => infos.push(e),
            EntryType::Decision => decisions.push(e),
            EntryType::Task => tasks.push(e),
        }
    }

    write(&mut md_file, &format!("# Protocol {}", Local::now().format("%Y-%m-%d")))?;

    write(&mut md_file, &format!("\n## Participants\n"))?;
    participants
        .iter()
        .for_each(|participant| println!("* {}", participant));

    write(&mut md_file, &format!("\n## Information\n"))?;
    write(&mut md_file, &format!("|Time|Said by|text|"))?;
    write(&mut md_file, &format!("| --- | --- | ---|"))?;
    for e in infos {
        write(&mut md_file,
              &format!(
                  "|{}|{}|{}|",
                  e.timestamp.format("%H:%M:%S"),
                  e.owner,
                  e.message,
              ))?;
    }

    write(&mut md_file, &format!("---\n## Decisions\n"))?;
    for e in decisions {
        write(&mut md_file,
              &format!(
                  "* <> {} - {}/{}",
                  e.message,
                  e.owner,
                  e.timestamp.format("%H:%M:%S")
              ))?;
    }

    write(&mut md_file, &format!("---\n## Tasks\n"))?;
    for e in tasks {
        write(&mut md_file,
              &format!(
                  "* [] {} - {}/{}",
                  e.message,
                  e.owner,
                  e.timestamp.format("%H:%M:%S")
              ))?;
    };

    return Ok(());
}

fn write(file: &mut File, text: &str) -> io::Result<()> {
    file.write_all(text.as_bytes())?;
    return Ok(());
}

fn collect_participants(protocol_entries: &[ProtocolEntry]) -> Vec<String> {
    protocol_entries
        .iter()
        .map(|e| e.owner.clone())
        .collect::<Vec<String>>()
}
