use aoc23::problem::*;

fn to_vec(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn to_vec_2(input: &str) -> u64 {
    input
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn find_lower_upper(time: u64, dist: u64) -> (u64, u64) {
    //efficiency ftw
    let mut lst = (0..=time)
        .map(|hold_t| (hold_t, (time - hold_t) * hold_t))
        .filter(|(_, d)| *d > dist);
    (lst.next().unwrap().0, lst.last().unwrap().0)
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let (t_str, d_str) = input.split_once("\n").unwrap();
        let times = to_vec(t_str);
        let dists = to_vec(d_str);
        times
            .iter()
            .enumerate()
            .map(|(i, time)| find_lower_upper(*time, dists[i]))
            .map(|(l, u)| u - l + 1)
            .product()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let (t_str, d_str) = input.split_once("\n").unwrap();
        let times = to_vec_2(t_str);
        let dists = to_vec_2(d_str);
        let (l, u) = find_lower_upper(times, dists);
        u - l + 1
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 288);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 71503);
    }
}
