mod tests {
    use std::{env, ffi::OsString, fs::remove_dir_all, io, path::Path};

    use clap::Parser;
    use tempfile::tempdir;

    use crate::{cli::Cli, execute_cli, path::record_path};

    /// The application's name.
    const APP: &str = env!("CARGO_PKG_NAME");

    /// Executes a command from the application's CLI entrypoint.
    ///
    /// Returns stdout and stderr output from the command.
    ///
    /// The first argument should match the application's name.
    fn execute<P, I, T>(path: P, args: I) -> io::Result<(String, String)>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut cli = Cli::parse_from(args);
        cli.base_path = Some(path.as_ref().to_path_buf());
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();

        execute_cli(&cli, &mut out, &mut err)?;

        let out = String::from_utf8(out).expect("output should be a valid string");
        let err = String::from_utf8(err).expect("output should be a valid string");
        Ok((out, err))
    }

    #[test]
    fn it_records_exit_status() -> io::Result<()> {
        let tests = [("true", "0\n"), ("false", "1\n")];
        for (command, expected_output) in tests {
            let dir = tempdir()?;
            let (id, _) = execute(dir.path(), [APP, "record", command])?;
            let (out, err) = execute(dir.path(), [APP, "status", id.trim()])?;

            assert_eq!(out, expected_output);
            assert_eq!(err, "");
        }
        Ok(())
    }

    #[test]
    fn it_records_stdout() -> io::Result<()> {
        let tests = ["first output", "second output"];
        for expected_output in tests {
            let dir = tempdir()?;
            let (id, _) = execute(dir.path(), [APP, "record", "echo", expected_output])?;
            let (out, err) = execute(dir.path(), [APP, "stdout", id.trim()])?;

            assert_eq!(out, format!("{expected_output}\n"));
            assert_eq!(err, "");
        }
        Ok(())
    }

    #[test]
    fn it_records_stderr() -> io::Result<()> {
        let tests = ["first output", "second output"];
        for expected_output in tests {
            let dir = tempdir()?;
            let bash_command = format!("echo '{expected_output}' >&2");
            let (id, _) = execute(
                dir.path(),
                [APP, "record", "--", "bash", "-c", &bash_command],
            )?;
            let (out, err) = execute(dir.path(), [APP, "stderr", id.trim()])?;

            assert_eq!(out, "");
            assert_eq!(err, format!("{expected_output}\n"));
        }
        Ok(())
    }

    #[test]
    fn it_records_output() -> io::Result<()> {
        let tests = [("out1", "err1"), ("out2", "err2")];
        for (expected_out, expected_err) in tests {
            let dir = tempdir()?;
            let bash_command = format!("echo '{expected_out}'; echo '{expected_err}' >&2");
            let (id, _) = execute(
                dir.path(),
                [APP, "record", "--", "bash", "-c", &bash_command],
            )?;
            let (out, err) = execute(dir.path(), [APP, "output", id.trim()])?;

            assert_eq!(out, format!("{expected_out}\n"));
            assert_eq!(err, format!("{expected_err}\n"));
        }
        Ok(())
    }

    #[test]
    fn it_deletes_records() -> io::Result<()> {
        let dir = tempdir()?;
        let (id, _) = execute(dir.path(), [APP, "record", "true"])?;
        let path = record_path(dir.path(), id.trim())?;

        let (out, err) = execute(dir.path(), [APP, "delete", id.trim()])?;

        assert!(!path.is_dir(), "the record should be deleted");
        assert_eq!(out, "");
        assert_eq!(err, "");
        Ok(())
    }

    #[test]
    fn it_deletes_missing_records() -> io::Result<()> {
        // Attempting to delete the same record multiple times should not be considered
        // an error.
        let dir = tempdir()?;
        let (out, err) = execute(dir.path(), [APP, "delete", "non-existing-record"])?;
        assert_eq!(out, "");
        assert_eq!(err, "");
        Ok(())
    }

    #[test]
    fn it_expires_records() -> io::Result<()> {
        let dir = tempdir()?;
        let (id1, _) = execute(dir.path(), [APP, "record", "true"])?;
        let (id2, _) = execute(dir.path(), [APP, "record", "false"])?;
        let path2 = record_path(dir.path(), id2.trim())?;
        let path1 = record_path(dir.path(), id1.trim())?;

        let (out, err) = execute(dir.path(), [APP, "expire"])?;

        assert!(!path1.is_dir(), "the record should be deleted");
        assert!(!path2.is_dir(), "the record should be deleted");
        assert_eq!(out, "");
        assert_eq!(err, "");

        // Attempting to expire records multiple times should not be considered an
        // error.
        assert!(execute(dir.path(), [APP, "expire"]).is_ok());

        // Expiring records with a missing base directory should not be considered an
        // error.
        remove_dir_all(dir.path())?;
        assert!(execute(dir.path(), [APP, "expire"]).is_ok());

        Ok(())
    }
}
