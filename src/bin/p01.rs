use aoc23::problem::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<u64>().unwrap()).sum())
            .max()
            .unwrap()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut cals: Vec<u64> = input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<u64>().unwrap()).sum())
            .collect();
        cals.sort();
        cals.reverse();
        cals[0] + cals[1] + cals[2]
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 45000);
    }
}
