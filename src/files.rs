use std::path::Path;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};
use serde::export::Formatter;
use crate::files::IoOperation::{WRITE, FLUSH, OPEN};

/// Helper struct. Primary purpose is to open files and
///  write data into them.
pub struct Io {
    path: String
}

/// Error type which informs the user that an IO error has occurred.
#[derive(Debug)]
pub struct IoError {
    reason: String
}

/// Formatter for `IOError`, simply writes the `reason` into stdin.
impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

enum IoOperation {
    OPEN, WRITE, FLUSH
}

impl IoOperation {

    fn report_error(&self, file: &String, err: &std::io::Error) -> String {
        match self {
            IoOperation::OPEN => format!("Cannot open file {}: {}", file, err),
            IoOperation::FLUSH => format!("Cannot flush data to {}: {}", file, err),
            IoOperation::WRITE => format!("Cannot write data to {}: {}", file, err)
        }
    }

}

impl Io {

    /// Factory method. Creates a new `Exporter` instance.
    pub fn new(path: &str) -> Io {
        Io { path: String::from(path) }
    }

    /// Opens a file specified by the path in `Exporter` and
    ///  writes provided data into it.
    pub fn export(&self, data: &String) -> Result<(), IoError> {
        return match OpenOptions::new()
            .write(true)
            .create(true).open(Path::new(&self.path)) {
            Ok(file) => {
                let mut file = file;
                if let Err(err) = file.write(data.as_bytes()) {
                    return Err(self.report_error(WRITE, &err))
                }
                if let Err(err) = file.flush() {
                    return Err(self.report_error(FLUSH, &err))
                }
                Ok(())
            },
            Err(err) => Err(self.report_error(OPEN, &err))
        }
    }

    pub fn import(&self) -> Result<String, IoError> {
        match read_to_string(Path::new(&self.path)) {
            Ok(data) => Ok(data),
            Err(err) => Err(self.report_error(OPEN, &err))
        }
    }

    fn report_error(&self, op: IoOperation, err: &std::io::Error) -> IoError {
        IoError { reason: op.report_error(&self.path, err) }
    }

}