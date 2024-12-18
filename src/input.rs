use crate::error::{CliTimerError, Errors};

pub fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}

pub fn get_multiplier_as_secs(suffix: char) -> Result<u64, CliTimerError> {
    match suffix {
        's' => Ok(1u64),
        'm' => Ok(60u64),
        'h' => Ok(60u64 * 60u64),
        _ => {
            if suffix.is_numeric() {
                Err(Errors::WithoutSuffix)
            } else {
                Err(Errors::UnknownSuffix)
            }
        }
    }
}

pub fn as_duration(input: &str) -> Result<u64, CliTimerError> {
    if input.is_empty() {
        return Err(Errors::EmptyLine);
    }
    let mut seconds: u64 = 0;
    let mut digits = String::new();
    for c in input.chars() {
        dbg!(&c);
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            let scale = get_multiplier_as_secs(c)?;
            dbg!(&digits);
            let number = match digits.parse::<u64>() {
                Err(_) => return Err(Errors::UnableParseDuration),
                Ok(n) => n,
            };
            seconds += scale * number;
            digits = String::new();
        }
    }
    Ok(seconds)
}
