// TODO: ask how to reimplement termination on type Resutl = std::result::Result
// TODO: from char to struct with value in seconds
// TODO: write test for checking is suffix correct. Add yellow hints to errors.
// TODO: write test to check value parsing
// TODO: calculate duration based on suffix and value
// TODO: format hh:mm:ss
// TODO: display with ratatui

pub mod error;

const INPUT_DURATION: &str = "       ";

fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}

fn get_suffix(input: &str) -> Result<char, error::ParseError> {
    let suffix = match input.chars().last() {
        Some(c) => c,
        None => return Err(error::ParseError::EmptyLine),
    };
    Ok(suffix)
}

fn try_main() -> Result<(), error::ParseError> {
    let input = format_input(INPUT_DURATION);
    let _suffix = get_suffix(&input)?;
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::ExitCode::FAILURE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_space_deleted_after_format_input() {
        assert_eq!(format_input("  13h  "), "13h");
        assert_eq!(format_input("  120s    "), "120s");
        assert_eq!(format_input("     15m  "), "15m");
    }
    #[test]
    fn is_lowercase_after_format_input() {
        assert_eq!(format_input("1m"), "1m");
        assert_eq!(format_input("2M"), "2m");
        assert_eq!(format_input("14s"), "14s");
        assert_eq!(format_input("13S"), "13s");
    }
    #[test]
    fn is_return_suffix_correct() {
        assert_eq!(get_suffix(&format_input("1m")).unwrap(), 'm');
        assert_eq!(get_suffix(&format_input("15s")).unwrap(), 's');
        assert_eq!(get_suffix(&format_input("15h")).unwrap(), 'h');
    }
    #[test]
    fn is_return_error_in_get_suffix_correct() {
        assert!(matches!(get_suffix(""), Err(error::ParseError::EmptyLine)));
    }
}
