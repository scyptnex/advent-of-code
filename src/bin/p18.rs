use aoc23::coord::*;
use aoc23::problem::*;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn go(&self, c: &ICoord, d: isize) -> ICoord {
        match self {
            Dir::U => (c.0 - d, c.1),
            Dir::R => (c.0, c.1 + d),
            Dir::D => (c.0 + d, c.1),
            Dir::L => (c.0, c.1 - d),
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

fn prsl2(l: &str) -> (Dir, usize) {
    let (d, rest) = l.split_once(' ').unwrap();
    let (c, _) = rest.split_once(' ').unwrap();
    (d.parse().unwrap(), c.parse().unwrap())
}

//   Cutting     Adding     Joining    Splitting
//   xxxxxx      xxxxx      xxx xxx    xxxxxxxx
// &
//   xxx             xx       xxx        xxx
// =
//     xxxx      xxxxxx     xxxxxxx    xxx xxxx
fn resweep(segments: Vec<(isize, usize)>, nxt: (isize, usize)) -> Vec<(isize, usize)> {
    let mut events: Vec<isize> = [vec![nxt], segments]
        .into_iter()
        .flat_map(|v| v.into_iter())
        .flat_map(|(i, l)| [i, i + (l as isize) - 1].into_iter())
        .collect();
    events.sort();
    let mut ret: Vec<(isize, usize)> = vec![];
    let mut on = false;
    let mut cur: isize = isize::MIN;
    for ev in events {
        if ev == cur {
            if !on {
                cur = ret.pop().unwrap().0;
            }
            // TODO remainder
        } else {
            if on {
                ret.push((cur, (ev - cur + 1) as usize));
            }
            cur = ev;
        }
        on = !on;
    }
    ret
}

fn solver(input: &str, parsef: fn(&str) -> (Dir, usize)) -> isize {
    let mut loc: ICoord = (0, 0);
    let mut pq: BinaryHeap<(ICoord, usize)> = BinaryHeap::new();
    for (d, l) in input.lines().map(parsef) {
        let nxt = d.go(&loc, l as isize);
        match d {
            Dir::R => {
                pq.push((loc, l + 1));
            }
            Dir::L => {
                pq.push((nxt, l + 1));
            }
            _ => {}
        }
        loc = nxt;
    }
    // Pq is ordered from bottom up horizontal segments, so start at imax
    let mut sweepline = isize::MAX;
    let mut segments: Vec<(isize, usize)> = vec![];
    let mut tot: isize = 0;
    while let Some(((cur_r, cur_c), cur_l)) = pq.pop() {
        if cur_r < sweepline {
            tot += segments.iter().map(|(_, w)| *w as isize).sum::<isize>() * (sweepline - cur_r);
            sweepline = cur_r;
        }
        segments = resweep(segments, (cur_c, cur_l));
        dbg!(cur_r, cur_c, cur_l, &segments, sweepline, tot);
    }
    tot += segments.iter().map(|(_, w)| *w as isize).sum::<isize>();
    tot
}

fn solve_1(input: &str) -> isize {
    solver(input, prsl)
}
fn solve_2(input: &str) -> isize {
    solver(input, prsl2)
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
