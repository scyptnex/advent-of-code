use std::error::Error;
use std::fs::File;

use crate::input;

mod calorie;
mod cleanup;
mod rps;
mod rucksack;
mod stacks;

trait Problem {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>>;
}

pub fn solve(day: u8) -> Result<(), Box<dyn Error>> {
    let f = input::open_real_data(day)?;
    match day {
        1 => calorie::Calorie {}.solve(f),
        2 => rps::Rps {}.solve(f),
        3 => rucksack::Rucksack {}.solve(f),
        4 => cleanup::Cleanup {}.solve(f),
        5 => stacks::Stacks {}.solve(f),
        _ => Err(format!("Unknown problem for day {day}"))?,
    }
}
