mod record;

use clap::App;

fn main() {
    let _matches = App::new("Protocoler")
        .version("1.0.0")
        .author("Sebastian Z. <corka149@mailbox.org>")
        .about("A minimalistic typer for protocols")
        .long_about(
            "A Fast and minimalistic protocol generator built powered by clap & Rust.
                It can output the protocol in different formats.",
        )
        .get_matches();

    let entries = record::start();

    let entries = entries
        .into_iter()
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<record::ProtocolEntry>>();

    for e in entries {
        println!("{}", e);
    }
}
