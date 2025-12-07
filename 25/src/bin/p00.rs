use aoc25::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        (input.len() - input.len()) as u64
    }
    fn solve_2(&self, input: &str) -> u64 {
        (input.len() - input.len()) as u64
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 0);
    }
}
