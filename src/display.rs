use std::io::stdout;

use crate::ascii;

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

fn seconds_to_hms(seconds: u64) -> [u64; 3] {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    [hours, minutes, seconds]
}

fn print_asciinumber(ascii_numbers: Vec<String>) -> std::io::Result<()> {
    let (term_columns, term_rows) = terminal::size()?;
    let hms_spacing_relative: u16 = 8;
    let hms_spacing: u16 = ascii::SYMBOL_WIDHT * 2 + 1 + hms_spacing_relative;
    let center_x = (term_columns - (ascii::SYMBOL_WIDHT * 6 + hms_spacing_relative + 2)) / 2;
    let center_y = (term_rows - ascii::SYMBOL_HEIGHT) / 2;
    for (n_digit, ascii) in ascii_numbers.iter().enumerate() {
        let mut row = center_y;
        let mut column_offset = center_x;
        for c in ascii.chars() {
            if c == '\n' {
                row += 1;
                column_offset = center_x;
                continue;
            }
            if c != ' ' {
                execute!(
                    stdout(),
                    SetForegroundColor(Color::Red),
                    SetBackgroundColor(Color::Red)
                )?;
            }
            execute!(
                stdout(),
                cursor::MoveTo(column_offset + hms_spacing * n_digit as u16, row),
                Print(c),
            )?;
            execute!(stdout(), ResetColor)?;
            column_offset += 1;
        }
    }
    Ok(())
}

pub fn display_left_time(seconds: u64) -> std::io::Result<()> {
    let ascii_time: Vec<String> = seconds_to_hms(seconds)
        .iter()
        .map(|&x| ascii::number_as_asciiart(x))
        .collect();
    execute!(
        stdout(),
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        cursor::SavePosition,
    )?;
    print_asciinumber(ascii_time)?;
    execute!(stdout(), ResetColor, cursor::RestorePosition,).unwrap();
    Ok(())
}
