use std::{
    fs::{read_to_string, remove_dir_all, File},
    io::{self, Write},
    path::Path,
};

use crate::path::{record_path, status_path, stderr_path, stdout_path};

/// Deletes a record by its ID.
///
/// This method does nothing if the referenced record does not exist.
pub(crate) fn delete<P: AsRef<Path>>(base_path: P, record_id: &str) -> io::Result<()> {
    let path = record_path(base_path, record_id)?;
    if !path.exists() {
        return Ok(());
    }

    remove_dir_all(path)
}

/// Prints both stdout and stderr output of a record by its ID.
///
/// Prints recorded stdout to stdout and recorded stderr to stderr respectively.
pub(crate) fn print_output<P: AsRef<Path> + Clone, O: Write, E: Write>(
    base_path: P,
    record_id: &str,
    out: &mut O,
    err: &mut E,
) -> io::Result<()> {
    let stdout_source = stdout_path(base_path.clone(), record_id)?;
    let stderr_source = stderr_path(base_path, record_id)?;

    print(stdout_source, out)?;
    print(stderr_source, err)?;

    Ok(())
}

/// Prints the exit status of a record by its ID.
pub(crate) fn print_status<P: AsRef<Path>, W: Write>(
    base_path: P,
    record_id: &str,
    out: &mut W,
) -> io::Result<()> {
    let status_string = read_to_string(status_path(base_path, record_id)?)?;
    writeln!(out, "{}", status_string.trim())?;
    Ok(())
}

/// Prints the stdout output of a record by its ID.
pub(crate) fn print_stdout<P: AsRef<Path>, W: Write>(
    base_path: P,
    record_id: &str,
    out: &mut W,
) -> io::Result<()> {
    print(stdout_path(base_path, record_id)?, out)
}

/// Prints the stderr output of a record by its ID.
pub(crate) fn print_stderr<P: AsRef<Path>, W: Write>(
    base_path: P,
    record_id: &str,
    err: &mut W,
) -> io::Result<()> {
    print(stderr_path(base_path, record_id)?, err)
}

/// Deletes all records.
pub(crate) fn expire_all_records<P: AsRef<Path>>(base_path: P) -> io::Result<()> {
    let base_directory = base_path.as_ref();
    if !base_directory.is_dir() {
        return Ok(());
    }

    for path in base_directory.read_dir()? {
        remove_dir_all(path.unwrap().path())?;
    }

    Ok(())
}

/// Prints the file contents on a path to a writer.
fn print<P: AsRef<Path>, W: Write>(path: P, write: &mut W) -> io::Result<()> {
    let mut file = File::open(path)?;
    io::copy(&mut file, write)?;
    Ok(())
}
