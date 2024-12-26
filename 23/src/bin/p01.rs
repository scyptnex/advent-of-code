use aoc23::problem::*;

fn calibrate(line: &str) -> u64 {
    let v: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    (10 * v.first().unwrap() + v.last().unwrap()) as u64
}

fn sub(s: &str) -> Option<u64> {
    if s.as_bytes()[0].is_ascii_digit() {
        return Some((s.as_bytes()[0] - b'0') as u64);
    } else if s.starts_with("zero") {
        return Some(0);
    } else if s.starts_with("one") {
        return Some(1);
    } else if s.starts_with("two") {
        return Some(2);
    } else if s.starts_with("three") {
        return Some(3);
    } else if s.starts_with("four") {
        return Some(4);
    } else if s.starts_with("five") {
        return Some(5);
    } else if s.starts_with("six") {
        return Some(6);
    } else if s.starts_with("seven") {
        return Some(7);
    } else if s.starts_with("eight") {
        return Some(8);
    } else if s.starts_with("nine") {
        return Some(9);
    }
    None
}

fn calibrate2(line: &str) -> u64 {
    let v: Vec<u64> = (0..line.len()).filter_map(|i| sub(&line[i..])).collect();
    (10 * v.first().unwrap() + v.last().unwrap()) as u64
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(calibrate).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(calibrate2).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 142);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Prob::new().solve_2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            0281
        );
    }
}
