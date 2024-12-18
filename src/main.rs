// TODO: refactor error test out. Impossible to read exptected value
// TODO: change u32 in parse_duratint to Time object?
// TODO: format hh:mm:ss
// TODO: display with ratatui
// TODO: from colored to crossterm

use std::env;
use std::thread;
use std::time::Duration;

mod display;
mod error;
use error::{CliTimerError, Errors};

fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}
fn get_multiplier_as_secs(suffix: &str) -> Result<u64, CliTimerError> {
    match suffix.chars().last() {
        Some(c) => match c {
            's' => Ok(1u64),
            'm' => Ok(60u64),
            'h' => Ok(60u64 * 60u64),
            c => {
                if c.is_numeric() {
                    return Err(Errors::WithoutSuffix);
                } else {
                    return Err(Errors::UnknownSuffix);
                }
            }
        },
        None => return Err(Errors::UnknownSuffix),
    }
}
fn parse_input_as_duration(input: &str) -> Result<Duration, CliTimerError> {
    if input.is_empty() {
        return Err(Errors::EmptyLine);
    }
    let (duration, suffix) = input.split_at(input.len() - 1);
    let sec_multiplier = get_multiplier_as_secs(suffix)?;
    let duration: u64 = match duration.parse() {
        Ok(num) => num,
        Err(_) => return Err(Errors::UnableParseDuration),
    };
    let duration_in_seconds = Duration::from_secs(duration * sec_multiplier);
    Ok(duration_in_seconds)
}
fn try_main() -> Result<(), CliTimerError> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return Err(Errors::EmptyLine);
    }
    let input = format_input(&args[1]);
    let mut duration = parse_input_as_duration(&input)?;

    let one_sec = Duration::from_secs(1);
    while !duration.is_zero() {
        match display::display_left_time(duration) {
            Err(_) => return Err(Errors::UnableDisplay),
            Ok(_) => {}
        }
        duration = duration.abs_diff(one_sec);
        thread::sleep(one_sec);
    }
    Ok(())
}
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn test_formating() {
        assert_eq!(format_input("  13h  "), "13h");
        assert_eq!(format_input("  120s    "), "120s");
        assert_eq!(format_input("     15m  "), "15m");
        assert_eq!(format_input("1m"), "1m");
        assert_eq!(format_input("2M"), "2m");
        assert_eq!(format_input("14s"), "14s");
        assert_eq!(format_input("13S"), "13s");
    }
    #[test]
    fn test_duratin_as_secs() {
        assert_eq!(
            parse_input_as_duration("1m").unwrap(),
            Duration::from_secs(60)
        );
        assert_eq!(
            parse_input_as_duration("15s").unwrap(),
            Duration::from_secs(15)
        );
        assert_eq!(
            parse_input_as_duration("15h").unwrap(),
            Duration::from_secs(15 * 60 * 60)
        );
    }
    #[test]
    fn test_error_ouput() {
        assert_eq!(
            format!(
                "{} {}",
                "Error:".red().bold(),
                "your `input` is an emtpy line".red()
            ),
            Errors::EmptyLine.to_string()
        );
        assert_eq!(
            format!(
                "{} {}\n{} {}",
                "Error:".red().bold(),
                "can't find suffix".red(),
                "Hint:".yellow().bold(),
                "you should add suffix in your input. s - seconds, m - minute, h - hours".yellow()
            ),
            Errors::WithoutSuffix.to_string()
        );
        assert_eq!(
            format!(
                "{} {}\n{} {}",
                "Error:".red().bold(),
                "unknown suffix".red(),
                "Hint:".yellow().bold(),
                "possible suffix: s - seconds, m - minute, h - hours".yellow()
            ),
            Errors::UnknownSuffix.to_string()
        );
        assert_eq!(
            format!(
                "{} {}",
                "Error:".red().bold(),
                "unable to parse duration".red()
            ),
            Errors::UnableParseDuration.to_string()
        );
    }
}
