use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Problem;

pub struct Rucksack {}

impl Problem for Rucksack {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        let data: Vec<String> = BufReader::new(f).lines().filter_map(|s| s.ok()).collect();
        println!("{}", get_sum_priorities(&data));
        println!("{}", get_group_priorities(&data));
        Ok(())
    }
}

fn get_sum_priorities(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|s| halve(s))
        .map(|v| common_element(v))
        .map(|e| score(e) as i32)
        .sum()
}

fn get_group_priorities(lines: &Vec<String>) -> i32 {
    let mut li = lines.iter().peekable();
    let mut sum = 0;
    while li.peek().is_some() {
        sum += score(common_element(li.take(3).map(|s| s.as_ref()).collect())) as i32
    }
    sum
}

fn halve(s: &str) -> Vec<&str> {
    let (p, s) = s.split_at(s.len() / 2);
    vec![p, s]
}

fn common_element(vs: Vec<&str>) -> u8 {
    let mut vi = vs.iter();
    let main = vi.next().unwrap().as_bytes();
    let others: Vec<HashSet<&u8>> = vi
        .map(|s| HashSet::from_iter(s.as_bytes().iter()))
        .collect();
    for c in main {
        if others.iter().all(|s| s.contains(c)) {
            return c.clone();
        }
    }
    0
}

fn score(c: u8) -> u8 {
    if c < b'a' {
        27 + c - b'A'
    } else {
        1 + c - b'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .lines()
            .map(|s| String::from(s))
            .collect();
        assert_eq!(get_sum_priorities(&data), 157);
        assert_eq!(get_group_priorities(&data), 70);
    }

    #[test]
    fn test_score() {
        assert_eq!(score(b'a'), 1);
        assert_eq!(score(b'z'), 26);
        assert_eq!(score(b'A'), 27);
        assert_eq!(score(b'Z'), 52);
    }

    #[test]
    fn test_halve() {
        assert_eq!(halve("test"), vec!["te", "st"]);
    }

    #[test]
    fn test_common() {
        assert_eq!(common_element(vec!["te", "st"]), b't');
        assert_eq!(common_element(vec!["ABC", "BCD", "CDA"]), b'C');
    }
}
