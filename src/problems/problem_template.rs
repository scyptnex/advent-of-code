use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default)]
pub struct Todo {
    data: Vec<String>,
}

impl StructuredProblem for Todo {
    fn ingest(&mut self, f: File) {
        self.data = BufReader::new(f).lines().map(|s| s.unwrap()).collect();
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
    fn test_todo() {
        let t = Todo::default();
        assert_eq!(format!("{}", t.solve_1()), "TODO problem 1");
        assert_eq!(format!("{}", t.solve_2()), "TODO problem 2");
    }
}
