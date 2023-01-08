use std::error::Error;
use std::fs::File;

use super::Problem;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(i8)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

pub struct Rps {}

impl Problem for Rps {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        println!("solving RPS");
        Ok(())
    }
}

fn get_outcome(theirs: Move, mine: Move) -> i8 {
    let n = (mine as i8 - theirs as i8 + 3) % 3;
    println!("{:?}, {:?}, {}", theirs, mine, n);
    match n {
        0 => 3,
        1 => 6,
        _ => 0,
    }
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
        assert_eq!(get_outcome(Rock, Rock), 3);
        assert_eq!(get_outcome(Rock, Scissors), 0);
        assert_eq!(get_outcome(Rock, Paper), 6);
        assert_eq!(get_outcome(Scissors, Rock), 6);
        assert_eq!(get_outcome(Scissors, Scissors), 3);
        assert_eq!(get_outcome(Scissors, Paper), 0);
    }
}
