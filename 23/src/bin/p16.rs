use aoc23::problem::*;
use itertools::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, Hash)]
enum Pc {
    None,
    BMir,
    FMir,
    HSpl,
    VSpl,
}

type Beam = (usize, usize, Dir);

struct Grid(Vec<Vec<Pc>>);

impl Grid {
    fn new(input: &str) -> Self {
        Grid {
            0: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Pc::None,
                            '\\' => Pc::BMir,
                            '/' => Pc::FMir,
                            '-' => Pc::HSpl,
                            '|' => Pc::VSpl,
                            _ => panic!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn mover(&self, cur: &Beam, d: Dir) -> Option<Beam> {
        match d {
            Dir::N => {
                if cur.0 == 0 {
                    None
                } else {
                    Some((cur.0 - 1, cur.1, d))
                }
            }
            Dir::E => {
                if cur.1 == self.0[cur.0].len() - 1 {
                    None
                } else {
                    Some((cur.0, cur.1 + 1, d))
                }
            }
            Dir::S => {
                if cur.0 == self.0.len() - 1 {
                    None
                } else {
                    Some((cur.0 + 1, cur.1, d))
                }
            }
            Dir::W => {
                if cur.1 == 0 {
                    None
                } else {
                    Some((cur.0, cur.1 - 1, d))
                }
            }
        }
    }

    fn prop(&self, b: &Beam) -> Vec<Beam> {
        match self.0[b.0][b.1] {
            Pc::None => [self.mover(b, b.2), None],
            Pc::BMir => match b.2 {
                Dir::N => [self.mover(b, Dir::W), None],
                Dir::E => [self.mover(b, Dir::S), None],
                Dir::S => [self.mover(b, Dir::E), None],
                Dir::W => [self.mover(b, Dir::N), None],
            },
            Pc::FMir => match b.2 {
                Dir::N => [self.mover(b, Dir::E), None],
                Dir::E => [self.mover(b, Dir::N), None],
                Dir::S => [self.mover(b, Dir::W), None],
                Dir::W => [self.mover(b, Dir::S), None],
            },
            Pc::HSpl => match b.2 {
                Dir::N => [self.mover(b, Dir::E), self.mover(b, Dir::W)],
                Dir::S => [self.mover(b, Dir::E), self.mover(b, Dir::W)],
                _ => [self.mover(b, b.2), None],
            },
            Pc::VSpl => match b.2 {
                Dir::W => [self.mover(b, Dir::N), self.mover(b, Dir::S)],
                Dir::E => [self.mover(b, Dir::N), self.mover(b, Dir::S)],
                _ => [self.mover(b, b.2), None],
            },
        }
        .into_iter()
        .flatten()
        .collect()
    }

    fn solve(&self, i: Beam) -> u64 {
        let mut frontier = vec![i];
        let mut seen: HashSet<Beam> = frontier.iter().copied().collect();
        while !frontier.is_empty() {
            let new_front = frontier
                .iter()
                .flat_map(|b| self.prop(&b).into_iter())
                .filter(|b| seen.insert(b.clone()))
                .collect();
            frontier = new_front;
        }
        seen.iter().map(|b| (b.0, b.1)).unique().count() as u64
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
        Grid::new(input).solve((0, 0, Dir::E))
    }
    fn solve_2(&self, input: &str) -> u64 {
        let g = Grid::new(input);
        [
            (0..g.0.len())
                .map(|r| (r, 0, Dir::E))
                .map(|b| g.solve(b))
                .max()
                .unwrap(),
            (0..g.0.len())
                .map(|r| (r, g.0[r].len() - 1, Dir::W))
                .map(|b| g.solve(b))
                .max()
                .unwrap(),
            (0..g.0[0].len())
                .map(|c| (0, c, Dir::S))
                .map(|b| g.solve(b))
                .max()
                .unwrap(),
            (0..g.0[0].len())
                .map(|c| (g.0.len() - 1, c, Dir::N))
                .map(|b| g.solve(b))
                .max()
                .unwrap(),
        ]
        .into_iter()
        .max()
        .unwrap()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 46);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 51);
    }
}
