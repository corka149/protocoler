mod record;
mod report;
mod util;

use clap::App;
use std::io;

fn main() {
    let _matches = App::new("Protocoler")
        .version("1.0.0")
        .author("Sebastian Z. <corka149@mailbox.org>")
        .about("A minimalistic typer for protocols")
        .long_about(
            "
A Fast and minimalistic protocol generator built powered by 
clap & Rust. It can output the protocol in different formats.

<https://github.com/corka149/protocoler>
",
        )
        .get_matches();

    let participants = record_participants();

    let entries = record::start(util::input);

    let entries = entries
        .into_iter()
        .flatten()
        .collect::<Vec<record::ProtocolEntry>>();

    report::output(participants, entries);
}

fn record_participants() -> Vec<String> {
    loop {
        let participants: Result<Vec<String>, io::Error> =
            util::input("Participants of meeting? (Separate by comma)").map(|all_participants| {
                all_participants
                    .split(",")
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect::<Vec<String>>()
            });

        if participants.is_ok() {
            let participants = participants.unwrap();

            println!("{}", participants.len());
            if !participants.is_empty() {
                return participants;
            } else {
                eprintln!("No participants provided!")
            }
        } else {
            eprintln!(
                "Error while reading participants: {}",
                participants.unwrap_err()
            )
        }
    }
}
