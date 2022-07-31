//! Contains things around the CLI of protocoler

use std::io;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crate::{persist, report};

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
    command: Option<Commands>,
}

impl Cli {
    /// Determines whether a subcommand should be executed
    pub fn should_launch_subcommand(&self) -> bool {
        self.command.is_some()
    }

    pub fn execute_sub_cmd(&self) -> io::Result<()> {
        match &self.command {
            Some(sub_cmd) => sub_cmd.execute(),
            None => Ok(())
        }
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
    },
}

impl Commands {
    /// Executes the selected subcommand.
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::Convert { source, target } => self.convert(source, target)
        }
    }

    fn convert(&self, source: &PathBuf, target: &PathBuf) -> io::Result<()> {
        let entries = persist::load_from_csv(source)?;
        report::save(&entries, target)?;

        Ok(())
    }
}

/// Parses command line args.
pub fn parse() -> Cli {
    Cli::parse()
}
