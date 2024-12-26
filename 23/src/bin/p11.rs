use aoc23::coord::*;
use aoc23::problem::*;
use std::cmp::{max, min};

type Universe = Vec<Vec<bool>>;

fn expand_coords(g: Universe, gals: Vec<UCoord>, expand: usize) -> Vec<UCoord> {
    let mut out_gals = gals.clone();
    for row in (0..g.len()).filter(|row| g[*row].iter().all(|p| !p)) {
        for (gi, gp) in gals.iter().enumerate() {
            if gp.0 > row {
                out_gals[gi].0 += expand - 1;
            }
        }
    }
    for col in (0..g[0].len()).filter(|col| g.iter().all(|r| !r[*col])) {
        for (gi, gp) in gals.iter().enumerate() {
            if gp.1 > col {
                out_gals[gi].1 += expand - 1;
            }
        }
    }
    out_gals
}

fn manhattan_distance(x: &UCoord, y: &UCoord) -> usize {
    max(x.0, y.0) - min(x.0, y.0) + max(x.1, y.1) - min(x.1, y.1)
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }

    fn solve_2x(&self, input: &str, expand_amt: usize) -> u64 {
        let initial: Universe = input
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let galaxies: Vec<UCoord> = initial
            .iter()
            .enumerate()
            .flat_map(|(ri, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(_, p)| **p)
                    .map(move |(ci, _)| (ri, ci))
            })
            .collect();
        let galaxies = expand_coords(initial, galaxies, expand_amt);
        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                sum += manhattan_distance(&galaxies[i], &galaxies[j]);
            }
        }
        sum as u64
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        self.solve_2x(input, 2)
    }
    fn solve_2(&self, input: &str) -> u64 {
        self.solve_2x(input, 1_000_000)
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 374);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2x(TEST_INPUT, 2), 374);
        assert_eq!(Prob::new().solve_2x(TEST_INPUT, 10), 1030);
        assert_eq!(Prob::new().solve_2x(TEST_INPUT, 100), 8410);
    }
}
