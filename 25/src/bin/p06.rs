use aoc25::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let lines = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<_>>();
        (0..lines[0].len())
            .map(|i| -> u64 {
                let strm = (0..lines.len() - 1)
                    .map(|j| lines[j][i])
                    .flat_map(str::parse::<u64>)
                    .collect::<Vec<_>>();
                if lines.last().unwrap()[i] == "*" {
                    strm.iter().product()
                } else {
                    strm.iter().sum()
                }
            })
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let lines = input.lines().collect::<Vec<_>>();
        let mut cur = b' ';
        let mut total = 0;
        let mut ops = Vec::<u64>::new();
        for c in 0..lines[0].len() {
            let nums = lines[..lines.len() - 1]
                .iter()
                .map(|l| l.as_bytes()[c] as char)
                .filter(|c| *c != ' ')
                .collect::<String>();
            if lines.last().unwrap().as_bytes()[c] != b' ' {
                cur = lines.last().unwrap().as_bytes()[c];
            }
            if nums.is_empty() {
                total += if cur == b'*' {
                    ops.iter().product::<u64>()
                } else {
                    ops.iter().sum::<u64>()
                };
                ops.clear();
                cur = b' ';
            } else {
                ops.push(nums.parse().unwrap());
            }
        }
        total += if cur == b'*' {
            ops.iter().product::<u64>()
        } else {
            ops.iter().sum::<u64>()
        };
        total
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 3263827);
    }
}
