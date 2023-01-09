use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Problem;

pub struct Calorie {}

impl Problem for Calorie {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(f);
        let mut vec = Vec::new();
        let mut cur = 0;
        for line in reader.lines() {
            match line.unwrap().parse::<u32>() {
                Ok(x) => {
                    cur += x;
                }
                Err(_) => {
                    vec.push(cur);
                    cur = 0;
                }
            }
        }
        vec.push(cur);
        println!("{}", vec.iter().max().unwrap());
        vec.sort();
        println!("{}", vec.iter().rev().take(3).sum::<u32>());
        Ok(())
    }
}
