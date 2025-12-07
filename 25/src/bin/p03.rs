use aoc25::*;

fn max_joltage(s: &str) -> u64 {
    let mut maxi = 0;
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            let iv = s.as_bytes()[i] - b'0';
            let jv = s.as_bytes()[j] - b'0';
            let nv = (10 * iv + jv) as u64;
            maxi = std::cmp::max(maxi, nv);
        }
    }
    maxi
}

fn max_jj(s: &str) -> u64 {
    // tabl[i][j] = biggest number starting at the ith position with j digits.
    let mut tabl = vec![vec![0; 13]; s.len() + 1];
    for count in 1usize..=12 {
        for start in (0..s.len() - count + 1).rev() {
            let used = (s.as_bytes()[start] - b'0') as u64;
            let used = used * 10u64.pow((count - 1) as u32);
            let used = used + tabl[start + 1][count - 1];
            let skip = tabl[start + 1][count];
            tabl[start][count] = std::cmp::max(used, skip);
        }
    }
    tabl[0][12]
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(max_joltage).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(max_jj).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 357);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 3121910778619);
    }
}
