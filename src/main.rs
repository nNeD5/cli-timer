// TODO: make tests
/* TODO: make possible to test is correct error displayed
        - https://rust-cli.github.io/book/tutorial/errors.html
        - anyhow
*/
// TODO: format hh:mm:ss
// TODO: display with ratatui 
// TODO: input from command line 

use std::time::{Duration};
use colored::Colorize;
use anyhow::{Context, Result, bail};
use std::thread;

const INPUT_DURATION: &str = "   1M    ";

fn format_input(input: &str) -> String {
    input.trim().to_lowercase()
}

fn get_suffix(input: &str) -> Result<char> {
    let suffix = match input.chars().last() {
        Some(c) => c,
        None => bail!("{}", "Error: The input is empty line".red().bold())
    };
    Ok(suffix)
    // eprintln!("{} {}",
    //             "Error: Can't interpret suffix in".red().bold(),
    //             format!("\"{}\"", input).red().bold());
    // eprintln!("{}", "Hint: Suffix can be: 's' - seconds, 'm' - minutes, 'h' - hours".yellow());
}

fn main() -> Result<()> {
    let input = format_input(INPUT_DURATION);
    let suffix = get_suffix(&input);
    let value: u64 = match input[..input.len() - 1].parse() {
        Ok(num) => num,
        Err(_) => { 
            eprintln!("{} {}",
                        "Error: Can't parse number in".red().bold(),
                        format!("`{}`", input).red().bold());
            std::process::exit(1);
            },
    };
    let start = match suffix {
        Some('h') => Duration::from_secs(value) * 60 * 60,
        Some('m') => Duration::from_secs(value) * 60,
        Some('s') => Duration::from_secs(value),
        _ => {
            eprintln!("{} {}",
                        "Error: Can't interpret suffix in".red().bold(),
                        format!("`{}`", input).red().bold());
            eprintln!("{}", "Hint: Suffix can be: 's' - seconds, 'm' - minutes, 'h' - hours".yellow());
            std::process::exit(1);
        },
    };

    for i in 0..start.as_secs_f32() as i32 {
        println!("{}", start.as_secs_f32() - i as f32);
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_input() {
        assert_eq!(format_input("1m"), "1m");
        assert_eq!(format_input("2M"), "2m");
        assert_eq!(format_input("13s"), "13s");
        assert_eq!(format_input("13S"), "13s");
        assert_eq!(format_input("  13h  "), "13h");
        assert_eq!(format_input("  13H  "), "13h");
    }

    #[test]
    fn test_get_suffix() {
        assert_eq!(get_suffix("1m"), 'm');
        assert_eq!(get_suffix("15s"), 's');
        assert_eq!(get_suffix("15h"), 'h');
        // assert_eq!(get_suffix(""), 'h');
    }
}

