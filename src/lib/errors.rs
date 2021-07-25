use std::io::{stderr, Write};

use colored::*;

pub type UfofmtError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, UfofmtError>;

pub(crate) fn print_error(err: &UfofmtError) {
    let error_indicator = "[ERROR]".red().bold();
    let _ = writeln!(stderr(), "{} {}", error_indicator, err);
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
