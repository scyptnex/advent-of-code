use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default)]
pub struct Sand {
    data: Vec<String>,
}

impl Sand {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }
}

impl StructuredProblem for Sand {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(count_falls(blocks(self.data.iter())))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(count_falls(floor(blocks(self.data.iter()))))
    }
}

fn lowest(blocks: &HashSet<Coord>) -> Option<i32> {
    blocks.iter().map(|(_, y)| y).max().copied()
}

fn floor(blocks: HashSet<Coord>) -> HashSet<Coord> {
    let lowest = lowest(&blocks).unwrap();
    let mut blocks = blocks;
    blocks.extend((500 - lowest - 2..=500 + lowest + 2).map(|x| (x, lowest + 2)));
    blocks
}

type Coord = (i32, i32);

fn blocks<T: AsRef<str>, I: Iterator<Item = T>>(i: I) -> HashSet<Coord> {
    i.map(|s| {
        let xv = s
            .as_ref()
            .split(" -> ")
            .map(|ss| ss.split_once(',').unwrap())
            .map(|ss| (ss.0.parse().unwrap(), ss.1.parse().unwrap()))
            .collect::<Vec<Coord>>();
        xv.windows(2)
            .flat_map(|p| {
                let xs = std::cmp::min(p[0].0, p[1].0);
                let xe = std::cmp::max(p[0].0, p[1].0);
                let ys = std::cmp::min(p[0].1, p[1].1);
                let ye = std::cmp::max(p[0].1, p[1].1);
                (xs..=xe).flat_map(move |x| std::iter::repeat(x).zip(ys..=ye))
            })
            .collect::<Vec<Coord>>()
    })
    .flat_map(|v| v.into_iter())
    .collect()
}

fn fall(blocks: &HashSet<Coord>, start: &Coord) -> Option<Coord> {
    if blocks.contains(start) {
        return None;
    }
    let lowest = lowest(blocks);
    if lowest.is_none() {
        return None;
    }
    let lowest = lowest.unwrap();
    let mut cur = *start;
    while cur.1 < lowest {
        let nc = (cur.0, cur.1 + 1);
        if !blocks.contains(&nc) {
            cur = nc;
            continue;
        }
        let nc = (cur.0 - 1, cur.1 + 1);
        if !blocks.contains(&nc) {
            cur = nc;
            continue;
        }
        let nc = (cur.0 + 1, cur.1 + 1);
        if !blocks.contains(&nc) {
            cur = nc;
            continue;
        }
        return Some(cur);
    }
    None
}

fn count_falls(blocks: HashSet<Coord>) -> usize {
    let mut blocks = blocks;
    let start = (500, 0);
    for i in 0.. {
        let snd = fall(&blocks, &start);
        match snd {
            None => return i,
            Some(x) => blocks.insert(x),
        };
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_todo() {
        let mut t = Sand::default();
        t.read(DATA.lines().map(|s| String::from(s)));

        assert_eq!(format!("{}", t.solve_1()), "24");
        assert_eq!(format!("{}", t.solve_2()), "93");
    }

    #[test]
    fn test_blocks() {
        assert_eq!(
            blocks("498,4 -> 498,6 -> 496,6".lines()),
            HashSet::from_iter(vec![(498, 4), (498, 5), (498, 6), (497, 6), (496, 6)].into_iter())
        );
    }

    #[test]
    fn test_fall() {
        assert_eq!(fall(&HashSet::new(), &(0, 0)), None);
        assert_eq!(fall(&blocks("0,1 -> 0,1".lines()), &(0, 0)), None);
        assert_eq!(fall(&blocks("-1,1 -> 1,1".lines()), &(0, 0)), Some((0, 0)));

        let bl = blocks(DATA.lines());
        assert_eq!(fall(&bl, &(500, 0)), Some((500, 8)));
    }
}
