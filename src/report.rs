//! `protocol` creates protocol reports.
use super::record::ProtocolEntry;
use super::util::*;
use std::process::exit;

const RAW_FORMAT: &str = "raw";
const MARKDOWN_FORMAT: &str = "markdown";
const CSV_FORMAT: &str = "csv";
const ALLOWED_FORMATS: [&str; 3] = [RAW_FORMAT, MARKDOWN_FORMAT, CSV_FORMAT];

/// Outputs the protocol entries in different formats.
pub fn output(protocol_entries: Vec<ProtocolEntry>) {
    match select_format().as_str() {
        RAW_FORMAT => print_raw(protocol_entries),
        MARKDOWN_FORMAT => todo!(),
        CSV_FORMAT => todo!(),
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

fn print_raw(protocol_entries: Vec<ProtocolEntry>) {
    for e in protocol_entries {
        println!("{}", e);
    }
}
