use std::io::{stderr, Write};

use colored::*;
use lazy_static::lazy_static;

pub type UfofmtError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, UfofmtError>;

lazy_static! {
    pub static ref ERROR_INDICATOR: ColoredString = "[ERROR]".red().bold();
    pub static ref OK_INDICATOR: ColoredString = "[OK]".green().bold();
}

pub(crate) fn print_error(err: &UfofmtError) {
    let _ = writeln!(stderr(), "{} {}", *ERROR_INDICATOR, err);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ufofmterror_cast() {
        let custom_err = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let ufe = UfofmtError::from(custom_err);
        assert_eq!(ufe.to_string(), "test error");
    }
}
