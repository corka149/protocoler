//! Contains things around the CLI of protocoler

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// CSV file from which a previous recorded protocol should be loaded.
    #[clap(value_parser)]
    pub source: Option<PathBuf>,

    /// Disable auto-save on quit.
    #[clap(short, long, action)]
    pub disable_autosave: bool,

    /// Activates no theme and fallback to primitive theme.
    #[clap(short, long, action)]
    pub no_theme: bool,

    #[clap(subcommand)]
    command: Option<Commands>
}

impl Cli {

    /// Determines whether a subcommand should be executed
    pub fn should_launch_subcommand(&self) -> bool {
        self.command.is_some()
    }

}

#[derive(Subcommand)]
enum Commands {
    /// Converts a CSV protocol into another format.
    Convert {
        /// CSV file from which a previous recorded protocol should be loaded.
        #[clap(value_parser)]
        source: PathBuf,

        /// Path of the new format. The file extensions will determine the format (.csv, .md or .txt).
        #[clap(value_parser)]
        target: PathBuf,
    }
}

/// Parses command line args.
pub fn parse() -> Cli {
    Cli::parse()
}
