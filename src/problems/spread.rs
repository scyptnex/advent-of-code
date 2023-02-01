use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cardinal {
    North,
    East,
    South,
    West,
}

use Cardinal::*;

impl Cardinal {
    fn offset(&self) -> Coord {
        match self {
            North => Coord::new(0, 1),
            South => Coord::new(0, -1),
            East => Coord::new(1, 0),
            West => Coord::new(-1, 0),
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    fn counter_clockwise(&self) -> Self {
        self.clockwise().clockwise().clockwise()
    }

    fn order(i: usize) -> Self {
        match i % 4 {
            0 => North,
            1 => South,
            2 => West,
            _ => East,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn add(&self, other: &Coord) -> Self {
        Coord::new(self.x + other.x, self.y + other.y)
    }

    fn move_dir(&self, dir: &Cardinal) -> Self {
        self.add(&dir.offset())
    }

    fn move_range(&self, dir: &Cardinal) -> [Self; 3] {
        let m = self.move_dir(dir);
        [
            m.move_dir(&dir.clockwise()),
            m.move_dir(&dir.counter_clockwise()),
            m,
        ]
    }

    fn surrounding(&self) -> [Self; 8] {
        [
            self.add(&Coord::new(1, 0)),
            self.add(&Coord::new(1, 1)),
            self.add(&Coord::new(0, 1)),
            self.add(&Coord::new(-1, 1)),
            self.add(&Coord::new(-1, 0)),
            self.add(&Coord::new(-1, -1)),
            self.add(&Coord::new(0, -1)),
            self.add(&Coord::new(1, -1)),
        ]
    }
}

fn proposals(v: &Vec<Coord>, round_number: usize) -> Vec<Option<Coord>> {
    let taken: HashSet<&Coord> = HashSet::from_iter(v.iter());
    v.iter()
        .map(|c| {
            if c.surrounding()
                .iter()
                .all(|check_s| !taken.contains(check_s))
            {
                return None;
            }
            (0..4)
                .map(|r| Cardinal::order(r + round_number))
                .find_map(|dir| {
                    if c.move_range(&dir)
                        .iter()
                        .all(|check_c| !taken.contains(check_c))
                    {
                        Some(c.move_dir(&dir))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn moves(v: &Vec<Option<Coord>>) -> Vec<Option<Coord>> {
    let histogram: HashMap<&Coord, usize> =
        v.iter()
            .filter_map(|o| o.as_ref())
            .fold(HashMap::new(), |mut m, co| {
                m.insert(co, m.get(co).copied().unwrap_or(0) + 1);
                m
            });

    v.iter()
        .map(|oc| oc.and_then(|c| if histogram[&c] == 1 { Some(c) } else { None }))
        .collect()
}

fn round(positions: Vec<Coord>, round_number: usize) -> (Vec<Coord>, bool) {
    let props = proposals(&positions, round_number);
    let movements = moves(&props);
    if movements.iter().find(|m| m.is_some()).is_some() {
        (
            movements
                .iter()
                .enumerate()
                .map(|(idx, oc)| oc.unwrap_or(positions[idx]))
                .collect(),
            true,
        )
    } else {
        (positions, false)
    }
}

fn empty_spaces(positions: &Vec<Coord>) -> usize {
    let mins = positions
        .iter()
        .copied()
        .reduce(|mn, co| Coord::new(mn.x.min(co.x), mn.y.min(co.y)))
        .unwrap();
    let maxs = positions
        .iter()
        .copied()
        .reduce(|mn, co| Coord::new(mn.x.max(co.x), mn.y.max(co.y)))
        .unwrap();
    let bb = (maxs.x - mins.x + 1) * (maxs.y - mins.y + 1);
    bb as usize - positions.len()
}

#[derive(Default)]
pub struct Spread {
    data: Vec<String>,
}

impl Spread {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }

    fn initial_positions(&self) -> Vec<Coord> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars().enumerate().filter_map(move |(col_idx, c)| {
                    if c == '#' {
                        Some(Coord::new(col_idx as i32, -(row_idx as i32)))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

impl StructuredProblem for Spread {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        let mut positions = self.initial_positions();
        for r in 0..10 {
            let (pos, _) = round(positions, r);
            positions = pos
        }
        Box::new(empty_spaces(&positions))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let mut positions = self.initial_positions();
        for r in 0.. {
            let (pos, moved) = round(positions, r);
            if !moved {
                return Box::new(r + 1);
            }
            positions = pos
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let mut t = Spread::default();
        t.read(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
                .lines()
                .map(|s| String::from(s)),
        );

        assert_eq!(format!("{}", t.solve_1()), "110");
        assert_eq!(format!("{}", t.solve_2()), "20");
    }
}
