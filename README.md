# protocoler

[![Rust](https://github.com/corka149/protocoler/actions/workflows/rust.yml/badge.svg)](https://github.com/corka149/protocoler/actions/workflows/rust.yml)

> A minimalistic typer for protocols

A Fast and minimalistic protocol generator powered by Rust.
It can output protocols in different formats.

## Usage

`protocoler` is a Text-based UI app. Run `protocoler` and press x for help.

![UI example](.github/screenshot.png)

`protocoler` has also an CL interface.

```
$ ./protocoler --help                                                                                                                                                                         [±main ●▴]
protocoler 0.4.0
Sebastian Ziemann <corka149@mailbox.org>
A Fast and minimalistic protocol generator powered by Rust. It can output protocols in different
formats.

USAGE:
    protocoler [OPTIONS] [SOURCE] [SUBCOMMAND]

ARGS:
    <SOURCE>    CSV file from which a previous recorded protocol should be loaded

OPTIONS:
    -d, --disable-autosave    Disable auto-save on quit
    -h, --help                Print help information
    -n, --no-theme            Activates no theme and fallback to primitive theme
    -V, --version             Print version information

SUBCOMMANDS:
    convert    Converts a CSV protocol into another format
    help       Print this message or the help of the given subcommand(s)

```

## Build `protocoler`

_Requirements:_

- git
- Rust with cargo installation ([see here](https://rustup.rs/))

_Build it:_

```sh
git clone -b v0.3.0 --single-branch git@github.com:corka149/protocoler.git
cargo build -r
```

Enjoy your `protocoler` binary at `target/release/protocoler`.
