use std::error::Error;
use std::fs::File;

use crate::input;

mod calorie;
mod cleanup;
mod rps;
mod rucksack;
mod signal;
mod stacks;

trait Problem {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>>;
}

fn get_problem(day: u8) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(calorie::Calorie {})),
        2 => Some(Box::new(rps::Rps {})),
        3 => Some(Box::new(rucksack::Rucksack {})),
        4 => Some(Box::new(cleanup::Cleanup {})),
        5 => Some(Box::new(stacks::Stacks {})),
        6 => Some(Box::new(signal::Signal {})),
        _ => None,
    }
}

pub fn solve(day: u8) -> Result<(), Box<dyn Error>> {
    let f = input::open_real_data(day)?;
    let p = get_problem(day).ok_or_else(|| format!("Unknown problem for day {day}"))?;
    p.solve(f)
}
