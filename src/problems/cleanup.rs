use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Problem;

pub struct Cleanup {}

impl Problem for Cleanup {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        let data: Vec<String> = BufReader::new(f).lines().filter_map(|s| s.ok()).collect();
        println!("{}", count_pred(&data, contains));
        println!("{}", count_pred(&data, overlaps));
        Ok(())
    }
}

trait Tinter {
    fn tint(&self) -> (i32, i32);
}

impl Tinter for (&str, &str) {
    fn tint(&self) -> (i32, i32) {
        (self.0.parse().unwrap(), self.1.parse().unwrap())
    }
}

fn count_pred(lines: &Vec<String>, p: fn(&((i32, i32), (i32, i32))) -> bool) -> i32 {
    lines
        .iter()
        .map(|s| range_pairs(s))
        .fold(0, |c: i32, rp| -> i32 {
            if p(&rp) {
                return c + 1;
            }
            c
        })
}

fn range_pairs(line: &str) -> ((i32, i32), (i32, i32)) {
    let (l, r) = line.split_once(',').unwrap();
    (
        l.split_once('-').unwrap().tint(),
        r.split_once('-').unwrap().tint(),
    )
}

fn contains(((l1, l2), (r1, r2)): &((i32, i32), (i32, i32))) -> bool {
    (r1 >= l1 && r2 <= l2) || (l1 >= r1 && l2 <= r2)
}

fn overlaps(((l1, l2), (r1, r2)): &((i32, i32), (i32, i32))) -> bool {
    (r1 >= l1 && r1 <= l2)
        || (r2 >= l1 && r2 <= l2)
        || (l1 >= r1 && l1 <= r2)
        || (l2 >= r1 && l2 <= r2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .lines()
            .map(|s| String::from(s))
            .collect();
        assert_eq!(count_pred(&data, contains), 2);
        assert_eq!(count_pred(&data, overlaps), 4);
    }

    #[test]
    fn test_range_parse() {
        assert_eq!(range_pairs("1-1,2-2"), ((1, 1), (2, 2)));
    }
}
