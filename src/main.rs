//TODO: refactor error test out. Impossible to read exptected value  
// TODO: change u32 in parse_duratint to Time object?
// TODO: format hh:mm:ss
// TODO: display with ratatui

use error::CliTimerError;

pub mod error;

const INPUT_DURATION: &str = "11";

fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}
fn get_multiplier_as_secs(suffix: &str) -> Result<u32, error::CliTimerError> {
    match suffix.chars().last() {
        Some(c) => match c {
            's' => Ok(1u32),
            'm' => Ok(60u32),
            'h' => Ok(60u32 * 60u32),
            c => {
                if c.is_digit(10) {
                    return Err(error::ParseError::WithoutSuffix);
                } else {
                    return Err(error::ParseError::UnknownSuffix);
                }
            }
        },
        None => return Err(error::ParseError::UnknownSuffix),
    }
}
fn parse_duration_as_secs(input: &str) -> Result<u32, error::CliTimerError> {
    if input.is_empty() {
        return Err(error::ParseError::EmptyLine);
    }
    let (duration, suffix) = input.split_at(input.len() - 1);
    let sec_multiplier = get_multiplier_as_secs(suffix)?;
    let duration: u32 = match duration.parse() {
        Ok(num) => num,
        Err(_) => return Err(error::ParseError::UnableParseDuration),
    };
    let duration_in_seconds = duration * sec_multiplier;
    Ok(duration_in_seconds)
}
fn try_main() -> Result<(), error::CliTimerError> {
    let input = format_input(INPUT_DURATION);
    let _suffix = parse_duration_as_secs(&input)?;
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
        assert_eq!(parse_duration_as_secs("1m").unwrap(), 60);
        assert_eq!(parse_duration_as_secs("15s").unwrap(), 15);
        assert_eq!(parse_duration_as_secs("15h").unwrap(), 15 * 60 * 60);
    }
    #[test]
    fn test_error_ouput() {
        assert_eq!(format!(
            "{} {}",
            "Error:".red().bold(),
            "your `input` is an emtpy line".red()
        ), error::ParseError::EmptyLine.to_string());
        assert_eq!(format!(
            "{} {}\n{} {}",
            "Error:".red().bold(),
            "can't find suffix".red(),
            "Hint:".yellow().bold(),
            "you should add suffix in your input. s - seconds, m - minute, h - hours".yellow()
        ), error::ParseError::WithoutSuffix.to_string());
        assert_eq!(format!(
            "{} {}\n{} {}",
            "Error:".red().bold(),
            "unknown suffix".red(),
            "Hint:".yellow().bold(),
            "possible suffix: s - seconds, m - minute, h - hours".yellow()
        ), error::ParseError::UnknownSuffix.to_string());
        assert_eq!(format!(
            "{} {}",
            "Error:".red().bold(),
            "unable to parse duration".red()
        ), error::ParseError::UnableParseDuration.to_string());
    }
}
