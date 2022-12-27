use std::error::Error;
use std::fs::File;

use crate::input;

mod calorie;
mod rps;

trait Problem {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>>;
}

pub fn solve(day: u8) -> Result<(), Box<dyn Error>> {
    let f = input::open_real_data(day)?;
    match day {
        1 => calorie::Calorie {}.solve(f),
        2 => rps::Rps {}.solve(f),
        _ => Err(format!("Unknown problem for day {day}"))?,
    }
}
