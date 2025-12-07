use aoc25::*;

fn in_any_range(i: &u64, rs: &Vec<(u64, u64)>) -> bool {
    rs.iter().any(|(s, t)| s <= i && t >= i)
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let (ranges, ings) = input.split_once("\n\n").unwrap();
        let ranges = ranges
            .lines()
            .flat_map(|s| s.split_once("-"))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect::<Vec<_>>();
        ings.lines()
            .flat_map(str::parse::<u64>)
            .filter(|i| in_any_range(i, &ranges))
            .count() as u64
    }
    fn solve_2(&self, input: &str) -> u64 {
        let (ranges, _) = input.split_once("\n\n").unwrap();
        let mut ranges = ranges
            .lines()
            .flat_map(|s| s.split_once("-"))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect::<Vec<(u64, u64)>>();
        ranges.sort();
        let mut ranges2 = vec![ranges[0]];
        for i in 1..ranges.len() {
            if ranges[i].0 <= ranges2.last().unwrap().1 {
                let mx = std::cmp::max(ranges[i].1, ranges2.last().unwrap().1);
                ranges2.last_mut().unwrap().1 = mx;
            } else {
                ranges2.push(ranges[i]);
            }
        }
        ranges2.iter().map(|(a, b)| b - a + 1).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 14);
    }
}
