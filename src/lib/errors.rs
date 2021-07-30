use std::{fmt, path::PathBuf};

use colored::*;
use lazy_static::lazy_static;
use norad;

pub(crate) type Result<T> = std::result::Result<T, Error>;

lazy_static! {
    pub static ref ERROR_INDICATOR: ColoredString = "[ERROR]".red().bold();
    pub static ref OK_INDICATOR: ColoredString = "[OK]".green().bold();
}

// ufofmt custom error type
#[derive(Debug)]
pub(crate) enum Error {
    InvalidPath(PathBuf),
    NoradRead(PathBuf, norad::Error),
    NoradWrite(PathBuf, norad::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match &self {
            Error::NoradRead(p, e) => {
                write!(f, "norad read error: {}: {}", p.display(), e)
            }
            Error::NoradWrite(p, e) => {
                write!(f, "norad write error: {}: {}", p.display(), e)
            }
            Error::InvalidPath(p) => {
                write!(f, "invalid path error: {} was not found", p.display())
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ufofmterror_invalid_path() {
        let ufe = Error::InvalidPath(PathBuf::from("testpath.ufo"));
        assert_eq!(ufe.to_string(), "invalid path error: testpath.ufo was not found");
    }

    #[test]
    fn test_ufofmterror_read() {
        let ne = norad::Error::MissingLayer("test".to_owned());
        let ufe = Error::NoradRead(PathBuf::from("test.ufo"), ne);
        assert!(ufe.to_string().starts_with("norad read error: "));
    }

    #[test]
    fn test_ufofmterror_write() {
        let ne = norad::Error::MissingLayer("test".to_owned());
        let ufe = Error::NoradWrite(PathBuf::from("test.ufo"), ne);
        assert!(ufe.to_string().starts_with("norad write error: "));
    }
}
