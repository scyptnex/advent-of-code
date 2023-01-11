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
    lines
        .chunks(3)
        .map(|c| common_element(c))
        .map(|e| score(e) as i32)
        .sum()
}

fn halve(s: &str) -> Vec<&str> {
    let (p, s) = s.split_at(s.len() / 2);
    vec![p, s]
}

fn common_element<I, T>(vs: I) -> u8
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    *vs.into_iter()
        .map(|s| HashSet::from_iter(s.as_ref().as_bytes().iter().copied()))
        .reduce(|accum: HashSet<u8>, cur| HashSet::from_iter(accum.intersection(&cur).copied()))
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn score(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => 27 + c - b'A',
        b'a'..=b'z' => 1 + c - b'a',
        _ => panic!("unrecognized char"),
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
