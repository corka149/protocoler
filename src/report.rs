//! `report` for saving protocols in different formats.

use std::fs::File;
use std::{env, io};
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use chrono::Local;

use crate::{EntryType, ProtocolEntry};


pub fn tmp_csv_path() -> Result<PathBuf, SystemTimeError> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let timestamp = duration.as_secs();

    let mut temp_path = env::temp_dir();
    let protocol_file = format!("{}_protocol.csv", timestamp);
    temp_path.push(protocol_file);

    Ok(temp_path)
}


/// Saves the protocol in the format inferred from the file extension.
pub fn save(entries: &[ProtocolEntry], target_path: &PathBuf) -> io::Result<()> {
    match target_path.extension() {
        Some(ext) if ext == "md" => save_markdown(entries, target_path),

        Some(ext) if ext == "csv" => save_csv(entries, target_path),

        _ => save_raw(entries, target_path),
    }
}

/// Saves the protocol entries in their String format.
pub fn save_raw(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut text_file = File::create(&path)?;

    let participants = collect_participants(protocol_entries);
    write(
        &mut text_file,
        &format!("Participants: {:?}\n", participants),
    )?;

    for e in protocol_entries {
        text_file.write_all(format!("{}\n", e).as_bytes())?
    }

    Ok(())
}

/// Saves all protocol entries  in CSV format.
pub fn save_csv(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut csv = File::create(&path)?;

    write(&mut csv, &format!("{}\n", ProtocolEntry::CSV_HEADER))?;
    for e in protocol_entries {
        write(&mut csv, &format!("{}\n", e.as_csv()))?;
    }

    Ok(())
}

/// Saves the protocol entries in a Jira compatible Markdown format.
pub fn save_markdown(protocol_entries: &[ProtocolEntry], path: &PathBuf) -> io::Result<()> {
    let mut md_file = File::create(&path)?;
    let mut infos: Vec<&ProtocolEntry> = Vec::new();
    let mut decisions: Vec<&ProtocolEntry> = Vec::new();
    let mut tasks: Vec<&ProtocolEntry> = Vec::new();
    let participants = collect_participants(protocol_entries);

    for e in protocol_entries {
        match e.entry_type {
            EntryType::Info => infos.push(e),
            EntryType::Decision => decisions.push(e),
            EntryType::Task => tasks.push(e),
        }
    }

    write(
        &mut md_file,
        &format!("# Protocol {}\n", Local::now().format("%Y-%m-%d")),
    )?;

    write(&mut md_file, "\n## Participants\n")?;
    for participant in participants {
        write(&mut md_file, &format!("* {}\n", participant))?;
    }

    write(&mut md_file, "\n## Information\n")?;
    write(&mut md_file, "|Time|Said by|text|\n")?;
    write(&mut md_file, "| --- | --- | ---|\n")?;
    for e in infos {
        write(
            &mut md_file,
            &format!(
                "|{}|{}|{}|\n",
                e.timestamp.format("%H:%M:%S"),
                e.owner,
                e.message,
            ),
        )?;
    }

    write(&mut md_file, "---\n## Decisions\n")?;
    for e in decisions {
        write(
            &mut md_file,
            &format!(
                "* <> {} - {}/{}\n",
                e.message,
                e.owner,
                e.timestamp.format("%H:%M:%S")
            ),
        )?;
    }

    write(&mut md_file, "---\n## Tasks\n")?;
    for e in tasks {
        write(
            &mut md_file,
            &format!(
                "* [] {} - {}/{}\n",
                e.message,
                e.owner,
                e.timestamp.format("%H:%M:%S")
            ),
        )?;
    }

    Ok(())
}

fn write(file: &mut File, text: &str) -> io::Result<()> {
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn collect_participants(protocol_entries: &[ProtocolEntry]) -> Vec<String> {
    protocol_entries
        .iter()
        .map(|e| e.owner.clone())
        .collect::<Vec<String>>()
}
