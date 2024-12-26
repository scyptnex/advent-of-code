use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use super::StructuredProblem;

type Coord = (i32, i32);

fn getc(s: &str) -> i32 {
    let numb = s.split_once('=').unwrap().1;
    let l = numb.len();
    if numb.as_bytes()[l - 1].is_ascii_digit() {
        numb.parse().unwrap()
    } else {
        numb.split_at(l - 1).0.parse().unwrap()
    }
}

struct BPair {
    sensor: Coord,
    closest_beacon: Coord,
}

impl BPair {
    fn mdist(&self) -> i32 {
        (self.sensor.0 - self.closest_beacon.0).abs()
            + (self.sensor.1 - self.closest_beacon.1).abs()
    }

    fn project(&self, y: i32) -> RangeInclusive<i32> {
        let off = self.mdist() - (y - self.sensor.1).abs();
        self.sensor.0 - off..=self.sensor.0 + off
    }
}

#[derive(Default)]
pub struct Beacon {
    data: Vec<BPair>,
}

impl Beacon {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i
            .map(|s| {
                let v: Vec<&str> = s.split(' ').collect();
                BPair {
                    sensor: (getc(v[2]), getc(v[3])),
                    closest_beacon: (getc(v[8]), getc(v[9])),
                }
            })
            .collect();
    }

    fn find_beacon(&self, limit: i32) -> Coord {
        for y in 0..=limit {
            let mut v: Vec<RangeInclusive<i32>> = self
                .data
                .iter()
                .map(|b| b.project(y))
                .filter(|r| !r.is_empty())
                .collect();
            v.sort_by_key(|r| *r.start());

            if v.len() < 2 {
                continue;
            }

            let mut itr = v.iter();
            let mut cur_max = itr.next().unwrap().end();
            for r in itr {
                if cur_max + 1 < *r.start() {
                    return (cur_max + 1, y);
                }
                cur_max = cur_max.max(r.end());
            }
        }
        panic!("can;t find");
    }

    fn signal_desc(&self, limit: i32) -> usize {
        let c = self.find_beacon(limit);
        c.0 as usize * 4000000 + c.1 as usize
    }

    fn count_blocked(&self, y: i32) -> usize {
        let bc = self
            .data
            .iter()
            .filter(|b| b.closest_beacon.1 == y)
            .map(|b| b.closest_beacon.0)
            .collect();
        self.data
            .iter()
            .flat_map(|b| b.project(y))
            .collect::<HashSet<i32>>()
            .difference(&bc)
            .count()
    }
}

impl StructuredProblem for Beacon {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.count_blocked(2000000))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.signal_desc(4000000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_todo() {
        let mut t = Beacon::default();
        t.read(DATA.lines().map(|s| String::from(s)));

        assert_eq!(t.count_blocked(10), 26);
        assert_eq!(t.find_beacon(20), (14, 11));
    }

    #[test]
    fn test_getc() {
        assert_eq!(getc("a=1"), 1);
        assert_eq!(getc("a=1:"), 1);
        assert_eq!(getc("x=-1,"), -1);
    }

    #[test]
    fn test_project() {
        let bp = BPair {
            sensor: (8, 7),
            closest_beacon: (2, 10),
        };

        assert_eq!(bp.project(7), (-1..=17));
        assert_eq!(bp.project(5), (1..=15));
        assert_eq!(bp.project(16), (8..=8));
        assert!(bp.project(18).is_empty());
    }
}
