use aoc25::problem::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let mut val = 50;
        let mut zeroes = 0;
        for l in input.lines() {
            let (d, v) = l.split_at(1);
            let v = v.parse::<u32>().unwrap() % 100;
            if d == "L" {
                val = (val + 100 - v) % 100;
            } else {
                val = (val + v) % 100
            }
            if val == 0 {
                zeroes += 1;
            }
        }
        zeroes
    }

    fn solve_2(&self, input: &str) -> u64 {
        let mut val = 50;
        let mut zeroes = 0;
        for l in input.lines() {
            let (d, v) = l.split_at(1);
            let v = v.parse::<u64>().unwrap();
            zeroes += v / 100;
            let v = v % 100;
            if d == "L" {
                if v > val && val != 0 {
                    zeroes += 1;
                }
                val = (val + 100 - v) % 100;
            } else {
                if val + v > 100 {
                    zeroes += 1;
                }
                val = (val + v) % 100
            }
            if val == 0 {
                zeroes += 1;
            }
            println!("{} {}", val, zeroes);
        }
        zeroes
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 6);
    }
}
