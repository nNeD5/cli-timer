// TODO: check is ouput from error is correct
// TODO: change u32 in parse_duratint to Time object?
// TODO: write test to check value parsing
// TODO: test to calculate duration based on suffix and value
// TODO: format hh:mm:ss
// TODO: from char to struct with value in seconds
// TODO: display with ratatui
// TODO: ask how to reimplement termination on type Resutl = std::result::Result

pub mod error;

const INPUT_DURATION: &str = "1";

fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}
fn get_multiplier_as_secs(suffix: &str) -> Result<u32, error::ParseError> {
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
fn parse_duration_as_secs(input: &str) -> Result<u32, error::ParseError> {
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
fn try_main() -> Result<(), error::ParseError> {
    let input = format_input(INPUT_DURATION);
    let _suffix = parse_duration_as_secs(&input)?;
    Ok(())
}
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_error_type() {
        assert!(matches!(
            parse_duration_as_secs(""),
            Err(error::ParseError::EmptyLine)
        ));
        assert!(matches!(
            parse_duration_as_secs("15"),
            Err(error::ParseError::WithoutSuffix)
        ));
        assert!(matches!(
            parse_duration_as_secs("1"),
            Err(error::ParseError::WithoutSuffix)
        ));
        assert!(matches!(
            parse_duration_as_secs("1a"),
            Err(error::ParseError::UnknownSuffix)
        ));
        assert!(matches!(
            parse_duration_as_secs("m"),
            Err(error::ParseError::UnableParseDuration)
        ));
        assert!(matches!(
            parse_duration_as_secs("sm"),
            Err(error::ParseError::UnableParseDuration)
        ));
    }
}
