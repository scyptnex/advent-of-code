use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default)]
pub struct Todo {
    data: Vec<String>,
}

impl Todo {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }
}

impl StructuredProblem for Todo {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new("Todo problem 1")
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new("Todo problem 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let mut t = Todo::default();
        t.read("".lines().map(|s| String::from(s)));

        assert_eq!(format!("{}", t.solve_1()), "Todo problem 1");
        assert_eq!(format!("{}", t.solve_2()), "Todo problem 2");
    }
}
