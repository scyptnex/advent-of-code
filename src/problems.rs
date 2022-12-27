use std::error::Error;
use std::fmt;

mod calorie;
mod rps;

#[derive(Debug)]
enum ChooseProblemErr {
    Unknown(u8),
}

impl fmt::Display for ChooseProblemErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChooseProblemErr::Unknown(x) => write!(f, "Unknown day {}", x),
        }
    }
}

impl Error for ChooseProblemErr {}

pub trait Problem {
    fn solve(&self);
}

pub fn get(day: u8) -> Result<Box<dyn Problem>, Box<dyn Error>> {
    match day {
        1 => Ok(Box::new(calorie::Calorie {})),
        2 => Ok(Box::new(rps::Rps {})),
        x => Err(Box::new(ChooseProblemErr::Unknown(x))),
    }
}
