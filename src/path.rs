use std::{
    env, io,
    path::{Path, PathBuf},
};

use standard_paths::{LocationType, StandardPaths};

/// Environment variable key to use as base path.
pub(crate) const BASE_PATH_ENV_KEY: &str = "CMDREC_BASE_PATH";

/// Returns the default base path used for storing records.
pub(crate) fn default_base_path() -> io::Result<PathBuf> {
    if let Ok(path) = env::var(BASE_PATH_ENV_KEY) {
        return Ok(PathBuf::from(path));
    }

    let mut path = StandardPaths::default().writable_location(LocationType::TempLocation)?;
    path.push(env!("CARGO_PKG_NAME"));
    Ok(path)
}

/// Returns the path to a specific record based on its ID.
pub(crate) fn record_path<P: AsRef<Path>>(base_path: P, record_id: &str) -> io::Result<PathBuf> {
    let mut path = PathBuf::from(base_path.as_ref());
    path.push(&record_id);
    Ok(path)
}

/// Returns the path to a record's status file based on its ID.
pub(crate) fn status_path<P: AsRef<Path>>(base_path: P, record_id: &str) -> io::Result<PathBuf> {
    record_relative_path(base_path, record_id, "status")
}

/// Returns the path to a record's stderr file based on its ID.
pub(crate) fn stderr_path<P: AsRef<Path>>(base_path: P, record_id: &str) -> io::Result<PathBuf> {
    record_relative_path(base_path, record_id, "stderr")
}

/// Returns the path to a record's stdout file based on its ID.
pub(crate) fn stdout_path<P: AsRef<Path>>(base_path: P, record_id: &str) -> io::Result<PathBuf> {
    record_relative_path(base_path, record_id, "stdout")
}

/// Returns a child path relative to a record's path.
fn record_relative_path<P: AsRef<Path>>(
    base_path: P,
    record_id: &str,
    child_path: &str,
) -> io::Result<PathBuf> {
    let mut path = record_path(base_path, record_id)?;
    path.push(child_path);
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_uses_env_to_resolve_base_path() -> io::Result<()> {
        let expected_base_path = "/new/base/path";
        env::set_var(BASE_PATH_ENV_KEY, expected_base_path);
        let path = default_base_path();
        env::remove_var(BASE_PATH_ENV_KEY);
        assert!(path.unwrap().starts_with(expected_base_path));
        Ok(())
    }

    #[test]
    fn it_resolves_status_paths() -> io::Result<()> {
        let path = status_path(default_base_path()?, "RECORD")?;
        assert_eq!(path.file_name().unwrap_or_default(), "status");
        Ok(())
    }

    #[test]
    fn it_resolves_stderr_paths() -> io::Result<()> {
        let path = stderr_path(default_base_path()?, "RECORD")?;
        assert_eq!(path.file_name().unwrap_or_default(), "stderr");
        Ok(())
    }

    #[test]
    fn it_resolves_stdout_paths() -> io::Result<()> {
        let path = stdout_path(default_base_path()?, "RECORD")?;
        assert_eq!(path.file_name().unwrap_or_default(), "stdout");
        Ok(())
    }
}
