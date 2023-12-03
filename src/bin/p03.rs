use aoc23::coord::*;
use aoc23::problem::*;
use itertools::*;

fn find_number(l: &str, start: usize) -> Option<UCoord> {
    (start..l.len())
        .find(|i| l.as_bytes()[*i].is_ascii_digit())
        .map(|d| {
            (
                d,
                (d..l.len())
                    .find(|j| !l.as_bytes()[*j].is_ascii_digit())
                    .unwrap_or(l.len()),
            )
        })
}

fn ln_numbers(line: &str, num: usize) -> Vec<(u64, Vec<UCoord>)> {
    let mut cur = 0;
    let mut ret = vec![];
    loop {
        if let Some((s, e)) = find_number(line, cur) {
            let val: u64 = String::from_utf8_lossy(&line.as_bytes()[s..e])
                .parse()
                .unwrap();
            ret.push((val, (s..e).map(|c| (num, c)).collect()));
            cur = e + 1;
            continue;
        }
        break;
    }
    ret
}

fn numbers(input: &str) -> Vec<(u64, Vec<UCoord>)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| ln_numbers(l, i).into_iter())
        .collect()
}

fn pieces(inpt: &str) -> Vec<(u8, usize, usize)> {
    inpt.lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(c, part)| {
                    if !part.is_ascii_digit() && *part != b'.' {
                        Some((*part, r, c))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let p_locs: std::collections::HashSet<UCoord> =
            pieces(input).iter().map(|x| (x.1, x.2)).collect();
        numbers(input)
            .iter()
            .filter(|l| {
                l.1.iter()
                    .any(|c| c.adjacent_all().iter().any(|c| p_locs.contains(&c)))
            })
            .map(|l| l.0)
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut g_to_nums: std::collections::HashMap<UCoord, Vec<u64>> = pieces(input)
            .iter()
            .filter(|x| x.0 == b'*')
            .map(|x| ((x.1, x.2), vec![]))
            .collect();
        for n in numbers(input) {
            for adj in
                n.1.iter()
                    .flat_map(|nc| nc.adjacent_all().into_iter())
                    .unique()
            {
                if g_to_nums.contains_key(&adj) {
                    g_to_nums.get_mut(&adj).unwrap().push(n.0);
                }
            }
        }
        g_to_nums
            .iter()
            .filter(|x| x.1.len() == 2)
            .map(|x| x.1[0] * x.1[1])
            .sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_1() {
        assert_eq!(find_number("467..114..", 0), Some((0, 3)));
        assert_eq!(find_number("467..114..", 4), Some((5, 8)));
        assert_eq!(find_number("..", 0), None);
        assert_eq!(find_number("", 0), None);
        assert_eq!(find_number("5", 0), Some((0, 1)));
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 467835);
    }
}
