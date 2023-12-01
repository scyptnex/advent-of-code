use aoc23::problem::*;

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn play_score(me: Move) -> u64 {
    match me {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(me: Move, opp: Move) -> u64 {
    match me {
        Move::Rock => match opp {
            Move::Rock => 3,
            Move::Paper => 0,
            Move::Scissors => 6,
        },
        Move::Paper => match opp {
            Move::Rock => 6,
            Move::Paper => 3,
            Move::Scissors => 0,
        },
        Move::Scissors => match opp {
            Move::Rock => 0,
            Move::Paper => 6,
            Move::Scissors => 3,
        },
    }
}

fn score(line: &str) -> u64 {
    let opp = match line.as_bytes()[0] {
        b'A' => Move::Rock,
        b'B' => Move::Paper,
        b'C' => Move::Scissors,
        _ => panic!(),
    };
    let me = match line.as_bytes()[2] {
        b'X' => Move::Rock,
        b'Y' => Move::Paper,
        b'Z' => Move::Scissors,
        _ => panic!(),
    };
    play_score(me) + outcome_score(me, opp)
}

fn score2(line: &str) -> u64 {
    let opp = match line.as_bytes()[0] {
        b'A' => Move::Rock,
        b'B' => Move::Paper,
        b'C' => Move::Scissors,
        _ => panic!(),
    };
    let me = match line.as_bytes()[2] {
        b'X' => match opp {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        b'Y' => opp,
        b'Z' => match opp {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        _ => panic!(),
    };
    play_score(me) + outcome_score(me, opp)
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(score).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(score2).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 12);
    }
}
