use std::io::stdout;
use std::time::Duration;

use crate::error::{CliTimerError, Errors};

use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    // cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition}
    // ExecutableCommand,
    // event,
};

fn seconds_to_hms(seconds: u64) -> (u64, u64, u64) {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    (hours, minutes, seconds)
}



pub fn display_left_time(dur: Duration) -> Result<(), CliTimerError> {
    let seconds = dur.as_secs();
    let (hours, minutes, seconds) = seconds_to_hms(seconds);
    let result = execute!(
        stdout(),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        cursor::SavePosition,
        Clear(ClearType::All),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print(&format!("{}h {}m {}s", hours, minutes, seconds)),
        ResetColor,
        cursor::RestorePosition,
    );
    match result {
        Err(_) => Err(Errors::UnableDisplay),
        Ok(_) => Ok(()),
    }
}
