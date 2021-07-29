use std::fmt;

use colored::*;
use lazy_static::lazy_static;

pub(crate) type Result<T> = std::result::Result<T, UfofmtError>;

lazy_static! {
    pub static ref ERROR_INDICATOR: ColoredString = "[ERROR]".red().bold();
    pub static ref OK_INDICATOR: ColoredString = "[OK]".green().bold();
}

// ufofmt custom error type
#[derive(Debug)]
pub(crate) struct UfofmtError {
    pub(crate) message: String,
    pub(crate) kind: UfofmtErrorKind,
}

impl fmt::Display for UfofmtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match &self.kind {
            UfofmtErrorKind::Read => {
                write!(f, "read error: {}", &self.message)
            }
            UfofmtErrorKind::Write => {
                write!(f, "write error: {}", &self.message)
            }
            UfofmtErrorKind::InvalidPath => {
                write!(f, "invalid path error: {}", &self.message)
            }
        }
    }
}

impl std::error::Error for UfofmtError {}

impl UfofmtError {
    // Associated functions
    pub(crate) fn new(kind: UfofmtErrorKind, message: &str) -> Self {
        Self { kind, message: message.to_owned() }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum UfofmtErrorKind {
    InvalidPath,
    Read,
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ufofmterror_invalid_path() {
        let ufe = UfofmtError::new(UfofmtErrorKind::InvalidPath, "testpath.ufo was not found");
        assert_eq!(ufe.to_string(), "invalid path error: testpath.ufo was not found");
    }

    #[test]
    fn test_ufofmterror_read() {
        let ufe = UfofmtError::new(UfofmtErrorKind::Read, "testpath.ufo");
        assert_eq!(ufe.to_string(), "read error: testpath.ufo");
    }

    #[test]
    fn test_ufofmterror_write() {
        let ufe = UfofmtError::new(UfofmtErrorKind::Write, "testpath.ufo");
        assert_eq!(ufe.to_string(), "write error: testpath.ufo");
    }
}
