use std::cmp;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Debug)]
enum Drc {
    U,
    D,
    L,
    R,
}

fn to_drc(s: &str) -> Drc {
    match s {
        "U" => Drc::U,
        "D" => Drc::D,
        "L" => Drc::L,
        "R" => Drc::R,
        _ => panic!("unable to parse as direction"),
    }
}

type Coord = (i32, i32);

fn to_offset(d: &Drc) -> Coord {
    match d {
        Drc::U => (0, 1),
        Drc::D => (0, -1),
        Drc::L => (-1, 0),
        Drc::R => (1, 0),
    }
}

fn cmove((x, y): &Coord, d: &Drc) -> Coord {
    let (dx, dy) = to_offset(d);
    (x + dx, y + dy)
}

fn follow((xh, yh): &Coord, (xt, yt): &Coord) -> Coord {
    let xd = xh - xt;
    let yd = yh - yt;
    let xm = cmp::min(1, cmp::max(xd, -1));
    let ym = cmp::min(1, cmp::max(yd, -1));
    if xd * xd + yd * yd > 2 {
        (xt + xm, yt + ym)
    } else {
        (*xt, *yt)
    }
}

fn parse_bit((d, a): (&str, &str)) -> (Drc, usize) {
    (to_drc(d), a.parse().unwrap())
}

#[derive(Default)]
pub struct Rope {
    data: Vec<(Drc, usize)>,
}

impl Rope {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.map(|s| parse_bit(s.split_once(" ").unwrap())).collect();
    }

    fn do_rope(&self, l: usize) -> usize {
        let mut r: Vec<Coord> = std::iter::repeat((0, 0) as Coord).take(l).collect();
        let mut tails: HashSet<Coord> = HashSet::new();
        tails.insert(*r.last().unwrap());
        for m in self
            .data
            .iter()
            .flat_map(|(d, a)| std::iter::repeat(d).take(*a))
        {
            r[0] = cmove(&r[0], m);
            for k in 1..l {
                r[k] = follow(&r[k - 1], &r[k]);
            }
            tails.insert(*r.last().unwrap());
        }
        tails.len()
    }
}

impl StructuredProblem for Rope {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.do_rope(2))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.do_rope(10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Rope {
        let mut t = Rope::default();
        t.read(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            .lines()
            .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_rope() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "13");
        assert_eq!(format!("{}", t.solve_2()), "1");
    }

    #[test]
    fn test_big_rope() {
        let mut t = Rope::default();
        t.read(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                .lines()
                .map(|s| String::from(s)),
        );
        assert_eq!(format!("{}", t.solve_2()), "36");
    }

    #[test]
    fn test_move() {
        assert_eq!(cmove(&(0, 0), &Drc::U), (0, 1));
    }

    #[test]
    fn test_follow() {
        assert_eq!(follow(&(2, 0), &(0, 0)), (1, 0));
        assert_eq!(follow(&(0, -2), &(0, 0)), (0, -1));
        assert_eq!(follow(&(2, -2), &(0, 0)), (1, -1));
    }
}
