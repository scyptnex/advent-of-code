use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

type RowCol = (usize, usize);

fn invert(h: u8) -> u8 {
    match h {
        b'S' => b'E',
        b'E' => b'S',
        b'a' => b'E',
        x => (b'z' - x) + b'a',
    }
}

#[derive(Default)]
pub struct Hill {
    data: Vec<String>,
}

impl Hill {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }

    fn get(&self, c: &RowCol) -> u8 {
        self.data[c.0].as_bytes()[c.1]
    }

    fn height(&self, c: &RowCol) -> u8 {
        match self.get(c) {
            b'S' => b'a',
            b'E' => b'z',
            x => x,
        }
    }

    fn adjacents<'a>(&'a self, rc: &'a RowCol) -> impl Iterator<Item = RowCol> + 'a {
        let v: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        v.into_iter()
            .filter(|off| rc.0 as i32 >= -off.0 && rc.1 as i32 >= -off.1)
            .map(|off| (off.0 + rc.0 as i32, off.1 + rc.1 as i32))
            .map(|nc| (nc.0 as usize, nc.1 as usize))
            .filter(|nc| nc.0 < self.data.len() && nc.1 < self.data[0].len())
            .filter(|nc| self.height(nc) <= self.height(rc) + 1)
    }

    fn start_coord(&self) -> RowCol {
        self.data
            .iter()
            .enumerate()
            .find_map(|(ri, rs)| {
                rs.chars()
                    .enumerate()
                    .find(|(_, cs)| *cs == 'S')
                    .map(|(ci, _)| (ri, ci))
            })
            .unwrap()
    }

    fn climb(&self) -> usize {
        let mut frontier: Vec<RowCol> = vec![self.start_coord()];
        let mut visited: HashSet<RowCol> = HashSet::from_iter(frontier.iter().copied());
        for i in 0.. {
            let mut new_frontier: Vec<RowCol> = Vec::new();
            while !frontier.is_empty() {
                let cur = frontier.pop().unwrap();
                if self.get(&cur) == b'E' {
                    return i;
                }
                for adj in self.adjacents(&cur) {
                    if visited.contains(&adj) {
                        continue;
                    }
                    new_frontier.push(adj);
                    visited.insert(adj);
                }
            }
            frontier = new_frontier;
            if frontier.is_empty() {
                panic!()
            }
        }
        panic!()
    }

    fn invert(&self) -> Vec<String> {
        self.data
            .iter()
            .map(|s| s.as_bytes().iter().map(|c| invert(*c)).collect::<Vec<u8>>())
            .map(|v| String::from_utf8(v).unwrap())
            .collect()
    }
}

impl StructuredProblem for Hill {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.climb())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let h2 = Hill {
            data: self.invert(),
        };
        Box::new(h2.climb())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Hill {
        let mut t = Hill::default();
        t.read(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_hill() {
        let t = sample();

        assert_eq!(format!("{}", t.solve_1()), "31");
        assert_eq!(format!("{}", t.solve_2()), "29");
    }

    #[test]
    fn test_climb() {
        let mut t = Hill::default();
        t.read(
            "SbcdefghijklmnopqrstuvwxyE"
                .lines()
                .map(|s| String::from(s)),
        );
        assert_eq!(t.climb(), 25);
    }

    #[test]
    fn test_start() {
        assert_eq!(sample().start_coord(), (0, 0))
    }

    #[test]
    fn test_invert() {
        assert_eq!(invert(b'z'), b'a');
        assert_eq!(invert(b'b'), b'y');
        assert_eq!(invert(b'E'), b'S');
        assert_eq!(invert(b'S'), b'E');
        assert_eq!(invert(b'a'), b'E');
    }
}
