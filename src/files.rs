use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use serde::export::Formatter;

/// Helper struct. Primary purpose is to open files and
///  write data into them.
pub struct Exporter {
    path: String
}

/// Error type which informs the user that an IO error has occurred.
#[derive(Debug)]
pub struct IOError {
    reason: String
}

/// Formatter for `IOError`, simply writes the `reason` into stdin.
impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}


impl Exporter {

    /// Factory method. Creates a new `Exporter` instance.
    pub fn new(path: &str) -> Exporter {
        Exporter { path: String::from(path) }
    }

    /// Opens a file specified by the path in `Exporter` and
    ///  writes provided data into it.
    pub fn export(&self, data: &String) -> Result<(), IOError> {
        return match OpenOptions::new()
            .write(true)
            .create(true).open(Path::new(&self.path)) {
            Ok(file) => {
                let mut file = file;
                if let Err(err) = file.write(data.as_bytes()) {
                    return Err(IOError {
                        reason: format!("Cannot write data {} to file {}: {}", data, &self.path, err)
                    })
                }
                if let Err(err) = file.flush() {
                    return Err(IOError {
                        reason: format!("Cannot flush data to file {}: {}", &self.path, err)
                    })
                }
                Ok(())
            },
            Err(err) =>
                Err(IOError { reason: format!("Cannot open file {}: {}", self.path, err) })
        }
    }

}