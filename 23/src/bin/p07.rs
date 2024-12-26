use aoc23::problem::*;

type Hand = [usize; 5];

fn card(input: &str, idx: usize) -> usize {
    match input.as_bytes()[idx] {
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!(),
    }
}

fn to_numeric(input: &str) -> Hand {
    [
        card(input, 0),
        card(input, 1),
        card(input, 2),
        card(input, 3),
        card(input, 4),
    ]
}

fn make_j_zero(pre: usize) -> usize {
    match pre {
        0..=8 => pre + 1,
        9 => 0,
        _ => pre,
    }
}

fn to_jokey_numeric(h: Hand) -> Hand {
    [
        make_j_zero(h[0]),
        make_j_zero(h[1]),
        make_j_zero(h[2]),
        make_j_zero(h[3]),
        make_j_zero(h[4]),
    ]
}

fn to_score(hand: Hand) -> u64 {
    hand.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| 13u64.pow(i as u32) * (*c as u64))
        .sum()
}

fn to_type(hand: Hand) -> u64 {
    let mut hist = [0u64; 13];
    for c in hand {
        hist[c] = hist[c] + 1;
    }
    let mut freq = hist.iter().enumerate().collect::<Vec<_>>();
    freq.sort_by_key(|(_, f)| *f);
    freq.reverse();
    match *freq[0].1 {
        5 => 6,
        4 => 5,
        3 => match *freq[1].1 {
            2 => 4,
            _ => 3,
        },
        2 => match *freq[1].1 {
            2 => 2,
            _ => 1,
        },
        _ => 0,
    }
}

fn to_jokey_type(hand: Hand) -> u64 {
    let mut hist = [0u64; 13];
    for c in hand {
        hist[c] = hist[c] + 1;
    }
    let jokes = hist[0];
    let mut freq: Vec<_> = hist.iter().skip(1).collect();
    freq.sort();
    freq.reverse();
    match *freq[0] + jokes {
        5 => 6,
        4 => 5,
        3 => match *freq[1] {
            2 => 4,
            _ => 3,
        },
        2 => match *freq[1] {
            2 => 2,
            _ => 1,
        },
        _ => 0,
    }
}

fn compare_hands(a: &(u64, u64, &str), b: &(u64, u64, &str)) -> std::cmp::Ordering {
    (a.0, a.1).cmp(&(b.0, b.1))
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let mut v = input
            .lines()
            .map(|s| s.split_once(" ").unwrap())
            .map(|(h, b)| (to_numeric(h), b))
            .map(|(h, b)| (to_type(h), to_score(h), b))
            .collect::<Vec<_>>();
        v.sort_by(compare_hands);
        v.iter()
            .map(|t| t.2.parse::<u64>().unwrap())
            .enumerate()
            .map(|t| (1 + t.0) as u64 * t.1)
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut v = input
            .lines()
            .map(|s| s.split_once(" ").unwrap())
            .map(|(h, b)| (to_numeric(h), b))
            .map(|(h, b)| (to_jokey_numeric(h), b))
            .map(|(h, b)| (to_jokey_type(h), to_score(h), b))
            .collect::<Vec<_>>();
        v.sort_by(compare_hands);
        v.iter()
            .map(|t| t.2.parse::<u64>().unwrap())
            .enumerate()
            .map(|t| (1 + t.0) as u64 * t.1)
            .sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 5905);
    }
}
