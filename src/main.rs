// TODO: handle error instead of unwrap
// TODO: delete all spaces
// TODO: make all lowercase
// TODO: format hh:mm:ss
// TODO: display with ratatui 

use std::time::{Duration};
use std::thread;

const INPUT_DURATION: &str = "1m";

fn main() {
    let sufix = INPUT_DURATION.chars().last();
    let value: u64 = INPUT_DURATION.trim()[..INPUT_DURATION.len() - 1].parse().unwrap();
    let start = match sufix {
        Some('h') => Duration::from_secs(value) * 60 * 60,
        Some('m') => Duration::from_secs(value) * 60,
        Some('s') => Duration::from_secs(value),
        _ => Duration::from_secs(value),
    };

    for i in 0..start.as_secs_f32() as i32 {
        println!("{}", start.as_secs_f32() - i as f32);
        thread::sleep(Duration::from_secs(1));
    }
}

