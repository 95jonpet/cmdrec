use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Record and retrieve command results.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Command to execute.
    #[clap(subcommand)]
    pub command: CliCommand,

    /// Base path to store records in.
    #[clap(long)]
    pub base_path: Option<PathBuf>,
}

/// Subcommand to execute.
#[derive(Subcommand, Debug)]
pub(crate) enum CliCommand {
    /// Delete a record.
    Delete { record_id: String },

    /// Expire all records.
    Expire,

    /// Print previously recorded output.
    Output { record_id: String },

    /// Record the status and output of a command.
    Record { args: Vec<String> },

    /// Print a previously recorded status.
    Status { record_id: String },

    /// Print previously recorded error output.
    Stderr { record_id: String },

    /// Print previously recorded output.
    Stdout { record_id: String },
}
