use std::error::Error;
use std::fs::File;

use super::Problem;

pub struct Calorie {}

impl Problem for Calorie {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        println!("solving calorie");
        Ok(())
    }
}
