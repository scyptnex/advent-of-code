use aoc23::problem::*;
use std::collections::HashMap;

type Field = Vec<Vec<u8>>;

type FieldRecord = HashMap<Field, usize>;

// Returns in "west" orientation.
fn parser(input: &str) -> Field {
    input
        .lines()
        .map(|l| l.as_bytes().iter().cloned().collect())
        .collect()
}

// Rotates CLOCKWISE (north faces east).
// This is because tilts go COUTNERCLOCKWISE (north, west,...) and if you rotate the field
// clockwise you change the apparent rolling direction counterclockwise.
fn rotate_cl(f: Field) -> Field {
    let out_len = f[0].len();
    let mut iters: Vec<_> = f.into_iter().rev().map(|n| n.into_iter()).collect();
    (0..out_len)
        .map(|_| iters.iter_mut().map(|n| n.next().unwrap()).collect())
        .collect()
}

fn roll_to_zero(run: &mut Vec<u8>) {
    let mut stop_point = 0;
    for i in 0..run.len() {
        match run[i] {
            b'#' => stop_point = i + 1,
            b'O' => {
                run[i] = b'.';
                run[stop_point] = b'O';
                stop_point += 1;
            }
            _ => (),
        }
    }
}

fn all_to_zero(f: &mut Field) {
    for i in 0..f.len() {
        roll_to_zero(&mut f[i]);
    }
}

fn spinn(f: Field) -> Field {
    let mut f: Field = f;
    all_to_zero(&mut f);
    f = rotate_cl(f);
    all_to_zero(&mut f);
    f = rotate_cl(f);
    all_to_zero(&mut f);
    f = rotate_cl(f);
    all_to_zero(&mut f);
    rotate_cl(f)
}

fn rweight(run: &Vec<u8>) -> u64 {
    run.iter()
        .enumerate()
        .filter(|(_, x)| **x == b'O')
        .map(|(i, _)| run.len() as u64 - i as u64)
        .sum()
}

fn weight(f: &Field) -> u64 {
    f.iter().map(rweight).sum()
}

fn do_cycle(first: usize, second: usize, r: &FieldRecord) -> u64 {
    let headless = 1_000_000_000 - first;
    let rem = headless % (second - first);
    for p in r {
        if *p.1 == rem + first {
            return weight(p.0);
        }
    }
    panic!();
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let mut f = rotate_cl(rotate_cl(rotate_cl(parser(input))));
        all_to_zero(&mut f);
        weight(&f)
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut f = rotate_cl(rotate_cl(rotate_cl(parser(input))));
        let mut r = FieldRecord::new();
        for c in 0.. {
            let old_o = r.insert(f.clone(), c);
            if let Some(old) = old_o {
                return do_cycle(old, c, &r);
            }
            f = spinn(f);
        }
        panic!();
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 136);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 64);
    }

    #[test]
    fn test_roll() {
        let mut t: Vec<u8> = b"OOO#...#.O###O.".to_vec();
        roll_to_zero(&mut t);
        assert_eq!(t, b"OOO#...#O.###O.".to_vec());
    }

    #[test]
    fn test_weight() {
        assert_eq!(rweight(&b".O.".to_vec()), 2);
    }
}
