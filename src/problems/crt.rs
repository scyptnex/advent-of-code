use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default)]
pub struct Crt {
    data: Vec<String>,
}

impl Crt {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }
}

impl StructuredProblem for Crt {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new("TODO problem 1")
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new("TODO problem 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt() {
        let mut t = Crt::default();
        t.read("".lines().map(|s| String::from(s)));

        assert_eq!(format!("{}", t.solve_1()), "TODO problem 1");
        assert_eq!(format!("{}", t.solve_2()), "TODO problem 2");
    }
}
