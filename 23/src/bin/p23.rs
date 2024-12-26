use aoc23::coord::*;
use aoc23::problem::*;
use std::collections::{HashMap, HashSet};

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

type G = HashMap<UCoord, HashMap<UCoord, u64>>;

fn dump(g: &G) {
    for entry in g.iter() {
        print!("{:?} ->", entry.0);
        for se in entry.1.iter() {
            print!(" {:?}:{}", se.0, se.1);
        }
        println!();
    }
}

fn link(g: &mut G, s: UCoord, t: UCoord) {
    g.entry(s).or_insert_with(|| HashMap::new()).insert(t, 1);
}

fn search(g: &mut G, s: UCoord, grid: &Vec<Vec<char>>) {
    for ac in s.adjacent_cardinal() {
        if ac.0 < grid.len() && ac.1 < grid[ac.0].len() && grid[ac.0][ac.1] != '#' {
            link(g, s, ac);
        }
    }
}

fn replace(g: &mut G, s: &UCoord, k: &UCoord, t: UCoord, dist: u64) {
    g.get_mut(s).unwrap().remove(k);
    g.get_mut(s).unwrap().insert(t, dist);
}

fn try_simplify(g: &mut G, k: &UCoord) {
    let sub = g.get(k).unwrap();
    if sub.len() != 2 {
        return;
    }
    let mut iter = sub.iter();
    let s = *iter.next().unwrap().0;
    let t = *iter.next().unwrap().0;
    if !g.get(&s).unwrap().contains_key(k) || !g.get(&t).unwrap().contains_key(k) {
        return;
    }
    let dist = sub.get(&s).unwrap() + sub.get(&t).unwrap();
    replace(g, &s, k, t, dist);
    replace(g, &t, k, s, dist);
    g.remove(k);
}

fn simplify(g: &mut G) {
    loop {
        let all_keys: Vec<UCoord> = g.keys().copied().collect();
        let pre_size = g.len();
        for k in all_keys {
            try_simplify(g, &k);
        }
        if g.len() == pre_size {
            return;
        }
    }
}

fn longest_walk(g: G) -> u64 {
    let start = g.iter().map(|e| e.0).min_by_key(|l| l.0).unwrap();
    let end = g.iter().map(|e| e.0).max_by_key(|l| l.0).unwrap();
    // dbg!(start, end);
    let mut longest = 0;
    let mut walk = Vec::new();
    let mut visited = HashSet::new();
    walk.push((start, 0, g[start].iter()));
    visited.insert(start);
    loop {
        let cur = walk.pop();
        if cur.is_none() {
            break;
        }
        let (cur_n, cur_d, mut cur_i) = cur.unwrap();
        if cur_n == end {
            // println!(
            //     "{}",
            //     walk.iter()
            //         .map(|n| format!("{:?}:{}, ", n.0, n.1))
            //         .collect::<String>()
            // );
            visited.remove(cur_n);
            longest = std::cmp::max(longest, cur_d);
            continue;
        }
        if let Some((tn, td)) = cur_i.next() {
            walk.push((cur_n, cur_d, cur_i));
            if visited.contains(tn) {
                continue;
            } else {
                visited.insert(tn);
                walk.push((tn, cur_d + *td, g[tn].iter()));
            }
        } else {
            visited.remove(cur_n);
        }
    }
    longest
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut g = G::new();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    '.' => search(&mut g, (r, c), &grid),
                    '>' => link(&mut g, (r, c), (r, c + 1)),
                    '<' => link(&mut g, (r, c), (r, c - 1)),
                    '^' => link(&mut g, (r, c), (r - 1, c)),
                    'v' => link(&mut g, (r, c), (r + 1, c)),
                    _ => (),
                }
            }
        }
        simplify(&mut g);
        //dump(&g);
        longest_walk(g)
    }
    fn solve_2(&self, input: &str) -> u64 {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut g = G::new();
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                match grid[r][c] {
                    '#' => (),
                    _ => search(&mut g, (r, c), &grid),
                }
            }
        }
        simplify(&mut g);
        dump(&g);
        longest_walk(g)
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 94);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 154);
    }
}
