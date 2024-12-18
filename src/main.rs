// TODO: some sync algorithm
use std::env;
use std::thread;
use std::time::{Instant, Duration};
mod ascii;
mod display;
mod error;
mod input;
use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use error::{CliTimerError, Errors};
use std::io;

fn try_main() -> Result<(), CliTimerError> {
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        return Err(Errors::EmptyLine);
    }
    let input = input::format_input(&args[1]);
    let duration = input::as_duration(&input)? as f64;

    let one_sec = Duration::from_secs(1);
    let start = Instant::now();
    let mut from_start = start.elapsed().as_secs_f64();
    while from_start < duration {
        if display::display_left_time((duration - from_start) as u64).is_err() {
            return Err(Errors::UnableDisplay);
        }
        thread::sleep(one_sec);
        from_start = start.elapsed().as_secs_f64();
        dbg!(from_start);
    }
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        execute!(io::stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
        eprintln!("{}", e);
        std::process::exit(1);
    }
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use input::*;

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
        assert_eq!(as_duration("1m").unwrap(), 60);
        assert_eq!(as_duration("15s").unwrap(), 15);
        assert_eq!(as_duration("15h").unwrap(), 15 * 60 * 60);
        assert_eq!(as_duration("0h10m20s").unwrap(), 10 * 60 + 20);
        assert_eq!(as_duration("0h0m20s").unwrap(), 20);
        assert_eq!(as_duration("0h0m0s").unwrap(), 0);
        assert_eq!(as_duration("2h0m0s").unwrap(), 2 * 60 * 60);
        assert_eq!(as_duration("0h5m0s").unwrap(), 60 * 5);
        assert_eq!(as_duration("2h5m0s").unwrap(), (2 * 60 * 60) + (5 * 60));
        assert_eq!(
            as_duration("3h15m20s").unwrap(),
            (3 * 60 * 60) + (15 * 60) + 20
        );
    }
}
