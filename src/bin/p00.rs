use aoc23::problem::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<usize, usize> for Prob {
    fn solve_1(&self, input: &str) -> usize {
        return input.len();
    }
    fn solve_2(&self, input: &str) -> usize {
        return input.lines().count();
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
