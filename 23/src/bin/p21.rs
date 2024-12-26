use aoc23::coord::*;
use aoc23::problem::*;
use std::collections::HashSet;

#[derive(Default)]
struct Gardn {
    width: isize,
    height: isize,
    rocks: HashSet<ICoord>,
    start: ICoord,
}

impl Gardn {
    fn reachable(&self, steps: usize) -> u64 {
        let mut locs = HashSet::new();
        locs.insert(self.start);
        for _ in 0..steps {
            locs = locs
                .iter()
                .map(ICoord::adjacent_cardinal)
                .flatten()
                .filter(|l| {
                    l.0 >= 0
                        && l.0 < self.height
                        && l.1 >= 0
                        && l.1 < self.width
                        && !self.rocks.contains(l)
                })
                .collect();
        }
        locs.len() as u64
    }
}

impl std::str::FromStr for Gardn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut rocks = HashSet::new();
        let mut start = ICoord::default();
        for (ri, ln) in s.lines().enumerate() {
            for (ci, cn) in ln.chars().enumerate() {
                let ci = ci as isize;
                let ri = ri as isize;
                width = std::cmp::max(width, ci + 1);
                height = std::cmp::max(height, ri + 1);
                let v = (ri, ci);
                if cn == '#' {
                    rocks.insert(v);
                } else if cn == 'S' {
                    start = v;
                }
            }
        }
        Ok(Gardn {
            width,
            height,
            rocks,
            start,
        })
    }
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.parse::<Gardn>().unwrap().reachable(64)
    }
    fn solve_2(&self, input: &str) -> u64 {
        (input.len() - input.len()) as u64
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_1() {
        assert_eq!(TEST_INPUT.parse::<Gardn>().unwrap().reachable(6), 16);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 0);
    }
}
