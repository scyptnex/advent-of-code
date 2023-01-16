use std::error::Error;
use std::fs::File;

use crate::input;

mod calorie;
mod cleanup;
mod filesys;
mod problem_template;
mod rps;
mod rucksack;
mod signal;
mod stacks;
mod treehouse;

trait Problem {
    fn solve(&mut self, f: File) -> Result<(), Box<dyn Error>>;
}

trait StructuredProblem {
    fn ingest(&mut self, f: File);
    fn solve_1(&self) -> Box<dyn std::fmt::Display>;
    fn solve_2(&self) -> Box<dyn std::fmt::Display>;
}

impl<T: StructuredProblem> Problem for T {
    fn solve(&mut self, f: File) -> Result<(), Box<dyn Error>> {
        self.ingest(f);
        let sol_1 = self.solve_1();
        println!("Solution 1:");
        println!("{}", sol_1);
        println!();
        let sol_2 = self.solve_2();
        println!("Solution 2:");
        println!("{}", sol_2);
        Ok(())
    }
}

fn get_problem(day: u8) -> Option<Box<dyn Problem>> {
    match day {
        0 => Some(Box::new(problem_template::Todo::default())),
        1 => Some(Box::new(calorie::Calorie {})),
        2 => Some(Box::new(rps::Rps {})),
        3 => Some(Box::new(rucksack::Rucksack {})),
        4 => Some(Box::new(cleanup::Cleanup {})),
        5 => Some(Box::new(stacks::Stacks::default())),
        6 => Some(Box::new(signal::Signal {})),
        7 => Some(Box::new(filesys::Filesys::default())),
        8 => Some(Box::new(treehouse::Treehouse::default())),
        _ => None,
    }
}

pub fn solve(day: u8) -> Result<(), Box<dyn Error>> {
    let f = input::open_real_data(day)?;
    let mut p = get_problem(day).ok_or_else(|| format!("Unknown problem for day {day}"))?;
    p.solve(f)
}
