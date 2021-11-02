mod record;
mod report;
mod util;

use clap::App;

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

    let entries = record::start(util::input);

    let entries = entries
        .into_iter()
        .flatten()
        .collect::<Vec<record::ProtocolEntry>>();

    report::output(entries);
}
