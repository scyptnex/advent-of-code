use aoc25::*;
use std::collections::{HashMap, HashSet};

fn solve_1(input: &str) -> u64 {
    let mut start = (0, 0);
    let mut splits = HashSet::<(usize, usize)>::new();
    let mut bottom = 0;
    for (r, line) in input.lines().enumerate() {
        bottom = r + 1;
        for (c, val) in line.chars().enumerate() {
            if val == 'S' {
                start = (r, c);
            } else if val == '^' {
                splits.insert((r, c));
            }
        }
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    let mut frontier = vec![start];
    let mut encountered = 0;
    visited.insert(start);
    while let Some((r, c)) = frontier.pop() {
        if r + 1 >= bottom {
            continue;
        }
        let d = (r + 1, c);
        if splits.contains(&d) {
            encountered += 1;
            if visited.insert((r + 1, c - 1)) {
                frontier.push((r + 1, c - 1));
            }
            if visited.insert((r + 1, c + 1)) {
                frontier.push((r + 1, c + 1));
            }
        } else {
            if visited.insert(d) {
                frontier.push(d);
            }
        }
    }
    encountered
}

fn solve_2(input: &str) -> u64 {
    let mut start = (0, 0);
    let mut splits = HashSet::<(usize, usize)>::new();
    let mut bottom = 0;
    for (r, line) in input.lines().enumerate() {
        bottom = r + 1;
        for (c, val) in line.chars().enumerate() {
            if val == 'S' {
                start = (r, c);
            } else if val == '^' {
                splits.insert((r, c));
            }
        }
    }
    let mut frontier = HashMap::<usize, usize>::new();
    frontier.insert(start.1, 1);
    let advance = |nf: &mut HashMap<usize, usize>, c: usize, n: usize| {
        if !nf.contains_key(&c) {
            nf.insert(c, n);
        } else {
            nf.insert(c, n + nf[&c]);
        }
    };
    for r in start.0..bottom {
        let mut new_front = HashMap::<usize, usize>::new();
        for (c, n) in frontier {
            let d = (r + 1, c);
            if splits.contains(&d) {
                advance(&mut new_front, c - 1, n);
                advance(&mut new_front, c + 1, n);
            } else {
                advance(&mut new_front, c, n);
            }
        }
        frontier = new_front;
    }
    frontier.iter().map(|(_, v)| *v as u64).sum()
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 40);
    }
}
