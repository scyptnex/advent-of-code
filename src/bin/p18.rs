use aoc23::coord::*;
use aoc23::problem::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn go(&self, c: &ICoord) -> ICoord {
        match self {
            Dir::U => (c.0 - 1, c.1),
            Dir::R => (c.0, c.1 + 1),
            Dir::D => (c.0 + 1, c.1),
            Dir::L => (c.0, c.1 - 1),
        }
    }
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().unwrap() {
            'U' => Dir::U,
            'R' => Dir::R,
            'D' => Dir::D,
            'L' => Dir::L,
            _ => panic!(),
        })
    }
}

fn prsl(l: &str) -> (Dir, usize) {
    let (d, rest) = l.split_once(' ').unwrap();
    let (c, _) = rest.split_once(' ').unwrap();
    (d.parse().unwrap(), c.parse().unwrap())
}

fn solve_1(input: &str) -> isize {
    let mut hs = HashSet::new();
    let mut cur: ICoord = (0, 0);
    hs.insert(cur.clone());
    let mut bb = (-1, -1, 1, 1);
    for (d, l) in input.lines().map(prsl) {
        for _ in 0..l {
            cur = d.go(&cur);
            bb = (
                min(bb.0, cur.0 - 1),
                min(bb.1, cur.1 - 1),
                max(bb.2, cur.0 + 1),
                max(bb.3, cur.1 + 1),
            );
            hs.insert(cur.clone());
        }
    }
    let mut wq = vec![(bb.0, bb.1)];
    let mut outside: HashSet<ICoord> = wq.iter().cloned().collect();
    while let Some(cur) = wq.pop() {
        for x in cur
            .adjacent_cardinal()
            .into_iter()
            .filter(|(r, c)| *r >= bb.0 && *r <= bb.2 && *c >= bb.1 && *c <= bb.3)
            .filter(|x| !hs.contains(x))
        {
            if outside.insert(x.clone()) {
                wq.push(x);
            }
        }
    }
    let rs = bb.2 - bb.0 + 1;
    let cs = bb.3 - bb.1 + 1;
    (rs * cs) - (outside.len() as isize)
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

    static TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 62);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 0);
    }
}
