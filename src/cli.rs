//! # Command-line interface
//!

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    #[clap(long, parse(from_occurrences))]
    debug: usize,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        path: PathBuf,
    },
    Metadata {
        id: String,
        #[clap(parse(from_os_str))]
        attributes: Vec<OsString>,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
