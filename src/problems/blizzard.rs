use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

fn warp(location: i32, range: i32) -> i32 {
    if location == range {
        0
    } else if location == -1 {
        range - 1
    } else {
        location
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Bliz {
    x: i32,
    y: i32,
    h: (i32, i32),
}

impl Bliz {
    fn parse(x: i32, y: i32, dir_char: char) -> Option<Self> {
        let h = match dir_char {
            '^' => (0, 1),
            'v' => (0, -1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => return None,
        };
        Some(Bliz { x, y, h })
    }

    fn advance(&self, map: &Map) -> Self {
        let x = warp(self.x + self.h.0, map.width);
        let y = warp(self.y + self.h.1, map.height);
        Bliz { x, y, h: self.h }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    width: i32,
    height: i32,
    blizzards: Vec<Bliz>,
}

impl Map {
    fn from(v: &Vec<String>) -> Self {
        let height = v.len() as i32 - 2;
        let width = v[0].len() as i32 - 2;
        let blizzards = v
            .iter()
            .skip(1)
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.chars()
                    .skip(1)
                    .enumerate()
                    .filter_map(move |(c_idx, c)| {
                        Bliz::parse(c_idx as i32, height - row_idx as i32 - 1, c)
                    })
            })
            .collect();
        Map {
            width,
            height,
            blizzards,
        }
    }

    fn advance(&self) -> Self {
        Map {
            width: self.width,
            height: self.height,
            blizzards: self.blizzards.iter().map(|b| b.advance(self)).collect(),
        }
    }

    fn in_bounds(&self, (x, y): &(i32, i32)) -> bool {
        if (*x, *y) == self.entry() || (*x, *y) == self.exit() {
            return true;
        }
        (0..self.width as i32).contains(x) && (0..self.height).contains(y)
    }

    fn entry(&self) -> (i32, i32) {
        (0, self.height)
    }

    fn exit(&self) -> (i32, i32) {
        (self.width - 1, -1)
    }
}

type ReachableMap = (Map, HashSet<(i32, i32)>);

fn next_state((m, reachable): ReachableMap) -> ReachableMap {
    let next_m = m.advance();
    let lookup: HashSet<(i32, i32)> = next_m.blizzards.iter().map(|b| (b.x, b.y)).collect();
    let next_reachable = reachable
        .iter()
        .flat_map(|(x, y)| {
            [(*x, *y), (x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)].into_iter()
        })
        .filter(|c| next_m.in_bounds(c))
        .filter(|c| !lookup.contains(c))
        .collect();
    (next_m, next_reachable)
}

#[derive(Default)]
pub struct Blizzard {
    data: Vec<String>,
}

impl Blizzard {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }

    fn initial(&self) -> ReachableMap {
        let m = Map::from(&self.data);
        let e = m.entry();
        (m, HashSet::from([e]))
    }

    fn path_out(&self) -> usize {
        let mut state = self.initial();
        for time in 0.. {
            if state.1.contains(&state.0.exit()) {
                return time;
            }
            state = next_state(state);
        }
        panic!()
    }

    fn path_zig(&self) -> usize {
        let mut state = self.initial();
        let mut time = 0;
        for target in [state.0.exit(), state.0.entry(), state.0.exit()] {
            loop {
                if state.1.contains(&target) {
                    state.1 = HashSet::from([target]);
                    break;
                }
                state = next_state(state);
                time += 1;
            }
        }
        time
    }
}

impl StructuredProblem for Blizzard {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.path_out())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.path_zig())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let mut t = Blizzard::default();
        t.read(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
                .lines()
                .map(|s| String::from(s)),
        );

        assert_eq!(format!("{}", t.solve_1()), "18");
        assert_eq!(format!("{}", t.solve_2()), "54");
    }

    #[test]
    fn test_map() {
        let m = Map::from(
            &"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#"
                .lines()
                .map(String::from)
                .collect(),
        );
        assert_eq!(m.width, 5);
        assert_eq!(m.height, 5);
        assert_eq!(m.blizzards.len(), 2);

        assert_eq!(
            m.blizzards[0],
            Bliz {
                x: 0,
                y: 3,
                h: (1, 0)
            }
        );
    }
}
