use aoc25::*;

fn solve_1(input: &str) -> u64 {
    (input.len() - input.len()) as u64
}
fn solve_2(input: &str) -> u64 {
    (input.len() - input.len()) as u64
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 0);
    }
}
