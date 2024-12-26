use aoc23::problem::*;

type PMap = Vec<Vec<bool>>;

fn reflects_with_errors(field: &PMap, left: usize, errs: usize) -> bool {
    let row_check = std::cmp::min(left, field.len() - left);
    let mut count = 0;
    for r in 0..row_check {
        count += (0..field[0].len())
            .filter(|c| field[left - r - 1][*c] != field[left + r][*c])
            .count();
    }
    return count == errs;
}

fn rscore(field: &PMap, errs: usize) -> u64 {
    (1..field.len())
        .filter(|cl| reflects_with_errors(field, *cl, errs))
        .sum::<usize>() as u64
}

fn reflectscore(input: &str, errs: usize) -> u64 {
    let field: PMap = input
        .lines()
        .map(|l| l.chars().map(|c| c == '.').collect())
        .collect();
    let s = rscore(&field, errs);
    if s != 0 {
        return s * 100;
    }
    let transposed: PMap = (0..field[0].len())
        .map(|j| field.iter().map(|f| f[j]).collect())
        .collect();
    let xs = rscore(&transposed, errs);
    assert!(xs != 0);
    return xs;
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.split("\n\n").map(|i| reflectscore(i, 0)).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.split("\n\n").map(|i| reflectscore(i, 1)).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 405);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 400);
    }
}
