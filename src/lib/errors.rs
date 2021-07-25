use std::io::{stderr, Write};

use colored::*;

pub type UfofmtError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, UfofmtError>;

pub(crate) fn print_error(err: &UfofmtError) {
    let error_indicator = "[ERROR]".red().bold();
    let _ = writeln!(stderr(), "{} {}", error_indicator, err);
}
