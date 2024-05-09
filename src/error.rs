//TODO: add colors
use colored::Colorize;
use std::{error::Error, fmt};

pub enum ParseError {
    EmptyLine,
    WithoutSuffix,
    UnableParseDuration,
    UnknownSuffix,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error_handler(self, f)
    }
}
impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error_handler(self, f)
    }
}
impl Error for ParseError {}

fn error_handler(error: &ParseError, f: &mut fmt::Formatter) -> fmt::Result {
    match error {
        ParseError::EmptyLine => writeln!(
            f,
            "{} {}",
            "Error:".red().bold(),
            "your input is an emtpy line".red()
        ),
        ParseError::WithoutSuffix => writeln!(
            f,
            "{} {}",
            "Error:".red().bold(),
            "your input is an emtpy line".red()
        ),
        ParseError::UnableParseDuration => writeln!(
            f,
            "{} {}",
            "Error:".red().bold(),
            "your input is an emtpy line".red()
        ),
        ParseError::UnknownSuffix => writeln!(
            f,
            "{} {}",
            "Error:".red().bold(),
            "your input is an emtpy line".red()
        ),
    }
}
