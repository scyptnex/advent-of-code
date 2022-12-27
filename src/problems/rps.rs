use std::error::Error;
use std::fs::File;

use super::Problem;

pub struct Rps {}

impl Problem for Rps {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        println!("solving RPS");
        Ok(())
    }
}
