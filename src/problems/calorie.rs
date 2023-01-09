use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Problem;

pub struct Calorie {}

impl Problem for Calorie {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(f);
        let mut vec = Vec::new();
        let mut cur: i64 = 0;
        for line in reader.lines() {
            let v: i64 = line
                .map(|l| if l.is_empty() { -1 } else { l.parse().unwrap() })
                .unwrap();
            if v == -1 {
                vec.push(cur);
                cur = 0;
                continue;
            } else {
                cur += v;
            }
        }
        vec.push(cur);
        println!("{}", vec.iter().max().unwrap());
        vec.sort();
        println!("{}", vec.iter().rev().take(3).sum::<i64>());
        Ok(())
    }
}
