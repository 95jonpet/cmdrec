use std::fs::create_dir_all;
use std::io::Write;
use std::path::Path;
use std::{fs::File, io, process::Command};

use crate::path::{record_path, status_path, stderr_path, stdout_path};

/// Character length of generated IDs.
const RECORD_ID_LENGTH: usize = 8;

/// Executes a command and records its stdout, stderr, and exit status.
///
/// Returns a unique ID referencing the record.
pub(crate) fn record<P: AsRef<Path> + Clone>(
    base_path: P,
    command: &[String],
) -> io::Result<String> {
    let id = generate_id();

    let record_path = record_path(&base_path, &id)?;
    create_dir_all(&record_path)?;

    let stdout = File::create(stdout_path(&base_path, &id)?)?;
    let stderr = File::create(stderr_path(&base_path, &id)?)?;

    let status = Command::new(&command[0])
        .args(&command[1..])
        .stdout(stdout)
        .stderr(stderr)
        .status()?;

    let mut status_file = File::create(status_path(base_path, &id)?)?;
    writeln!(status_file, "{}", status.code().unwrap_or(255))?;
    Ok(id)
}

/// Returns a generated random ID consisting of lowercase hexadecimal characters.
fn generate_id() -> String {
    let alphabet = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    let mut id = String::with_capacity(RECORD_ID_LENGTH);

    for _ in 0..RECORD_ID_LENGTH {
        let index: usize = rand::random();
        let ch = alphabet[index % alphabet.len()];
        id.push(ch);
    }

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_ids() {
        let ids = [generate_id(), generate_id()];
        assert_ne!(ids[0], ids[1], "IDs are unique");
    }
}
