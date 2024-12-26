use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Problem;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(i8)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Move {
    type Error = &'static str;
    fn try_from(v: u8) -> Result<Move, Self::Error> {
        match v {
            0 => Ok(Move::Rock),
            1 => Ok(Move::Paper),
            2 => Ok(Move::Scissors),
            _ => Err("Unknown value"),
        }
    }
}

pub struct Rps {}

impl Problem for Rps {
    fn solve(&mut self, f: File) -> Result<(), Box<dyn Error>> {
        let data: Vec<String> = BufReader::new(f).lines().filter_map(|s| s.ok()).collect();
        println!("{}", data.iter().map(|s| get_score(s) as u32).sum::<u32>());
        println!(
            "{}",
            data.iter().map(|s| get_rigged_score(s) as u32).sum::<u32>()
        );
        Ok(())
    }
}

fn get_score(play: &str) -> u8 {
    let theirs = get_opponent(play.chars().next().unwrap());
    let mine = get_mine(play.chars().rev().next().unwrap());
    get_outcome_score(theirs, mine) + get_move_score(mine)
}

fn get_rigged_score(play: &str) -> u8 {
    let theirs = get_opponent(play.chars().next().unwrap());
    let mine = get_rigged(theirs, play.chars().rev().next().unwrap());
    get_outcome_score(theirs, mine) + get_move_score(mine)
}

fn get_rigged(theirs: Move, outcome: char) -> Move {
    match outcome {
        'X' => Move::try_from((theirs as u8 + 2) % 3).unwrap(),
        'Y' => theirs,
        _ => Move::try_from((theirs as u8 + 1) % 3).unwrap(),
    }
}

fn get_outcome_score(theirs: Move, mine: Move) -> u8 {
    match (mine as i8 - theirs as i8 + 3) % 3 {
        0 => 3,
        1 => 6,
        _ => 0,
    }
}

fn get_move_score(mine: Move) -> u8 {
    mine as u8 + 1
}

fn get_opponent(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        _ => Move::Scissors,
    }
}

fn get_mine(c: char) -> Move {
    match c {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        _ => Move::Scissors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(get_opponent('A'), Move::Rock);
        assert_eq!(get_opponent('B'), Move::Paper);
        assert_eq!(get_opponent('C'), Move::Scissors);

        assert_eq!(get_mine('X'), Move::Rock);
        assert_eq!(get_mine('Y'), Move::Paper);
        assert_eq!(get_mine('Z'), Move::Scissors);
    }

    #[test]
    fn test_match() {
        use super::Move::*;
        assert_eq!(get_outcome_score(Rock, Rock), 3);
        assert_eq!(get_outcome_score(Rock, Scissors), 0);
        assert_eq!(get_outcome_score(Rock, Paper), 6);
        assert_eq!(get_outcome_score(Scissors, Rock), 6);
        assert_eq!(get_outcome_score(Scissors, Scissors), 3);
        assert_eq!(get_outcome_score(Scissors, Paper), 0);
    }

    #[test]
    fn test_score() {
        assert_eq!(get_score("A Y"), 8);
        assert_eq!(get_score("B X"), 1);
        assert_eq!(get_score("C Z"), 6);
    }

    #[test]
    fn test_rigged() {
        assert_eq!(get_rigged_score("A Y"), 4);
        assert_eq!(get_rigged_score("B X"), 1);
        assert_eq!(get_rigged_score("C Z"), 7);
    }
}
