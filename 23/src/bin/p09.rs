use aoc23::problem::*;
use itertools::*;

fn extrapolate(initial: Vec<i64>) -> i64 {
    if initial.iter().all(|i| *i == 0) {
        return 0;
    }
    let nxt = initial
        .iter()
        .tuple_windows()
        .map(|t: (&i64, &i64)| t.1 - t.0)
        .collect();
    return initial.last().unwrap() + extrapolate(nxt);
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<i64, i64> for Prob {
    fn solve_1(&self, input: &str) -> i64 {
        input
            .lines()
            .map(|l| {
                l.split(" ")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect()
            })
            .map(extrapolate)
            .sum()
    }
    fn solve_2(&self, input: &str) -> i64 {
        input
            .lines()
            .map(|l| {
                let mut v: Vec<i64> = l
                    .split(" ")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect();
                v.reverse();
                v
            })
            .map(extrapolate)
            .sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 114);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 2);
    }
}
