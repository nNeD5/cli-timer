use colored::Colorize;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result};

#[derive(Debug)]
pub struct CliTimerError {
    error_text: &'static str,
    hint_text: &'static str,
}
impl Error for CliTimerError {}
impl CliTimerError {
    fn error_formated(&self) -> String {
        format!("{} {}", "Error:".red().bold(), self.error_text.red())
    }
    fn hint_formated(&self) -> String {
        format!("{} {}", "Hint:".yellow().bold(), self.hint_text.yellow())
    }
}
impl Display for CliTimerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.hint_text.is_empty() {
            write!(f, "{}", self.error_formated())
        } else {
            write!(f, "{}\n{}", self.error_formated(), self.hint_formated())
        }
    }
}

#[allow(non_upper_case_globals, non_snake_case)]
pub mod ParseError {
    use super::*;
    pub const EmptyLine: CliTimerError = CliTimerError {
        error_text: "your `input` is an emtpy line",
        hint_text: "",
    };
    pub const WithoutSuffix: CliTimerError = CliTimerError {
        error_text: "can't find suffix",
        hint_text: "you should add suffix in your input. s - seconds, m - minute, h - hours",
    };
    pub const UnknownSuffix: CliTimerError = CliTimerError {
        error_text: "unknown suffix",
        hint_text: "possible suffix: s - seconds, m - minute, h - hours",
    };
    pub const UnableParseDuration: CliTimerError = CliTimerError {
        error_text: "unable to parse duration",
        hint_text: "",
    };
}
