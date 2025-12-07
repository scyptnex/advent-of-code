use aoc25::*;

fn repeats(s: &str, prefix_width: usize) -> bool {
    if s.len() % prefix_width != 0 {
        return false;
    }
    for i in prefix_width..s.len() {
        if s.as_bytes()[i] != s.as_bytes()[i % prefix_width] {
            return false;
        }
    }
    true
}

fn is_invalid(s: &str) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    repeats(s, s.len() / 2)
}

fn is_inval_2(s: &str) -> bool {
    let mid = s.len() / 2;
    for cw in 1..mid + 1 {
        if repeats(s, cw) {
            return true;
        }
    }
    false
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input
            .lines()
            .flat_map(|l| l.split(',').filter(|r| !r.is_empty()))
            .flat_map(|s| {
                let (a, b) = s.split_once('-').unwrap();
                a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
            })
            .map(|i| i.to_string())
            .filter(|s| is_invalid(s))
            .flat_map(|s| s.parse::<u64>())
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input
            .lines()
            .flat_map(|l| l.split(',').filter(|r| !r.is_empty()))
            .flat_map(|s| {
                let (a, b) = s.split_once('-').unwrap();
                a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
            })
            .map(|i| i.to_string())
            .filter(|s| is_inval_2(s))
            .flat_map(|s| s.parse::<u64>())
            .sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_repeats() {
        assert_eq!(repeats("11111", 1), true);
        assert_eq!(repeats("121212", 1), false);
        assert_eq!(repeats("121212", 2), true);
        assert_eq!(repeats("121222", 2), false);
        assert_eq!(repeats("1212121", 2), false);
    }

    static TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 4174379265);
    }
}
