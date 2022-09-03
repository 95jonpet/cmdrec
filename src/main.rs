mod cli;
mod command;
mod data;
mod path;

#[cfg(test)]
mod tests;

use std::{
    io::{self, stderr, stdout, Write},
    path::Path,
};

use clap::Parser;
use cli::{Cli, CliCommand};
use command::record;
use data::{delete, expire_all_records, print_output, print_status, print_stderr, print_stdout};
use path::default_base_path;

/// Main entrypoint for the CLI application.
#[cfg(not(tarpaulin_include))]
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut out = stdout().lock();
    let mut err = stderr().lock();
    let result = execute_cli(&cli, &mut out, &mut err);
    out.flush()?;
    err.flush()?;
    result
}

/// Executes a CLI command.
fn execute_cli<O: Write, E: Write>(cli: &Cli, out: &mut O, err: &mut E) -> io::Result<()> {
    let base_path = cli.base_path.clone().unwrap_or(default_base_path()?);
    match &cli.command {
        CliCommand::Delete { record_id } => delete(base_path, record_id),
        CliCommand::Expire => expire_all_records(base_path),
        CliCommand::Output { record_id } => print_output(base_path, record_id, out, err),
        CliCommand::Record { args } => create_record(base_path, args, out),
        CliCommand::Status { record_id } => print_status(base_path, record_id, out),
        CliCommand::Stderr { record_id } => print_stderr(base_path, record_id, err),
        CliCommand::Stdout { record_id } => print_stdout(base_path, record_id, out),
    }
}

/// Creates a record of an executed command and prints a unique ID referencing
/// the record.
fn create_record<P: AsRef<Path> + Clone, W: Write>(
    base_path: P,
    args: &[String],
    out: &mut W,
) -> io::Result<()> {
    let id = record(base_path, args)?;
    writeln!(out, "{id}")?;
    Ok(())
}
