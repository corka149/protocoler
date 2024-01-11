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
A Fast and minimalistic protocol generator powered by Rust. It can output protocols in different formats.

Usage: protocoler [OPTIONS] [SOURCE] [COMMAND]

Commands:
  convert  Converts a CSV protocol into another format
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [SOURCE]  CSV file from which a previous recorded protocol should be loaded

Options:
  -d, --disable-autosave  Disable auto-save on quit
  -n, --no-theme          Activates no theme and fallback to primitive theme
  -h, --help              Print help
  -V, --version           Print version

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
