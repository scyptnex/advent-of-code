use aoc23::problem::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type Field = Vec<Vec<u32>>;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Srch {
    cost: u64,
    r: usize,
    c: usize,
    d: Dir,
    m: usize,
}

impl Ord for Srch {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.r.cmp(&other.r))
            .then_with(|| self.c.cmp(&other.c))
            .then_with(|| self.d.cmp(&other.d))
            .then_with(|| self.m.cmp(&other.m))
    }
}

impl PartialOrd for Srch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn mover(f: &Field, r: usize, c: usize, d: Dir) -> Option<(usize, usize)> {
    match d {
        Dir::N => {
            if r == 0 {
                None
            } else {
                Some((r - 1, c))
            }
        }
        Dir::E => {
            if c == f[r].len() - 1 {
                None
            } else {
                Some((r, c + 1))
            }
        }
        Dir::S => {
            if r == f.len() - 1 {
                None
            } else {
                Some((r + 1, c))
            }
        }
        Dir::W => {
            if c == 0 {
                None
            } else {
                Some((r, c - 1))
            }
        }
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
        let cells: Field = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let mut q = BinaryHeap::new();
        q.push(Srch {
            cost: 0,
            r: 0,
            c: 0,
            d: Dir::S, // DONT PICK N or W since you can't turn around
            m: 0,
        });
        let mut visited = HashSet::<(usize, usize, Dir, usize)>::new();
        while let Some(cur) = q.pop() {
            if cur.r == cells.len() - 1 && cur.c == cells[cur.r].len() - 1 {
                return cur.cost;
            }
            if !visited.insert((cur.r, cur.c, cur.d, cur.m)) {
                continue;
            }
            for nd in [Dir::N, Dir::E, Dir::S, Dir::W] {
                if nd == cur.d.opposite() {
                    continue;
                }
                let nm = if cur.d == nd { cur.m + 1 } else { 1 };
                if nm > 3 {
                    continue;
                }
                let npos = mover(&cells, cur.r, cur.c, nd);
                if npos.is_none() {
                    continue;
                }
                let npos = npos.unwrap();
                if visited.contains(&(npos.0, npos.1, nd, nm)) {
                    continue;
                }
                q.push(Srch {
                    cost: cur.cost + cells[npos.0][npos.1] as u64,
                    r: npos.0,
                    c: npos.1,
                    d: nd,
                    m: nm,
                });
            }
        }
        panic!("unreachable");
    }
    fn solve_2(&self, input: &str) -> u64 {
        let cells: Field = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let mut q = BinaryHeap::new();
        q.push(Srch {
            cost: 0,
            r: 0,
            c: 0,
            d: Dir::S, // DONT PICK N or W since you can't turn around
            m: 0,
        });
        q.push(Srch {
            cost: 0,
            r: 0,
            c: 0,
            d: Dir::E, // DONT PICK N or W since you can't turn around
            m: 0,
        });
        let mut visited = HashSet::<(usize, usize, Dir, usize)>::new();
        while let Some(cur) = q.pop() {
            if cur.r == cells.len() - 1 && cur.c == cells[cur.r].len() - 1 && cur.m >= 4 {
                return cur.cost;
            }
            if !visited.insert((cur.r, cur.c, cur.d, cur.m)) {
                continue;
            }
            for nd in [Dir::N, Dir::E, Dir::S, Dir::W] {
                // can't turn around.
                if nd == cur.d.opposite() {
                    continue;
                }
                if nd != cur.d && cur.m < 4 {
                    continue;
                }
                let nm = if cur.d == nd { cur.m + 1 } else { 1 };
                if nm > 10 {
                    continue;
                }
                let npos = mover(&cells, cur.r, cur.c, nd);
                if npos.is_none() {
                    continue;
                }
                let npos = npos.unwrap();
                if visited.contains(&(npos.0, npos.1, nd, nm)) {
                    continue;
                }
                q.push(Srch {
                    cost: cur.cost + cells[npos.0][npos.1] as u64,
                    r: npos.0,
                    c: npos.1,
                    d: nd,
                    m: nm,
                });
            }
        }
        panic!("unreachable");
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 102);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 94);
        assert_eq!(
            Prob::new().solve_2(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        );
    }
}
