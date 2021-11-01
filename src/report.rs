//! `protocol` creates protocol reports.
use super::record::ProtocolEntry;
use super::util::*;

const ALLOWED_FORMATS: [&str; 3] = ["raw", "markdown", "csv"];

/// Outputs the protocol entries in different formats.
pub fn output(protocol_entries: Vec<ProtocolEntry>) {
    match select_format().as_str() {
        "raw" => print_raw(protocol_entries),
        _ => unimplemented!("Not yet implemented"),
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
