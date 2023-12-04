use aoc23::problem::*;

fn to_seq(lst: &str) -> Vec<u64> {
    lst.split(" ")
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn wccount(line: &str) -> usize {
    let (wins, guesses) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let wins: std::collections::HashSet<u64> = to_seq(wins).iter().copied().collect();
    to_seq(guesses).iter().filter(|g| wins.contains(g)).count()
}

fn wcscore(line: &str) -> u64 {
    match wccount(line) {
        0 => 0,
        n => 2u64.pow(n as u32 - 1),
    }
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(wcscore).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut counts: Vec<(u64, usize)> = input.lines().map(|l| (1, wccount(l))).collect();
        for i in 0..counts.len() {
            for j in i + 1..i + 1 + counts[i].1 {
                counts[j].0 += counts[i].0;
            }
        }
        counts.iter().map(|c| c.0).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 30);
    }
}
