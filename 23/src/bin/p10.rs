use aoc23::coord::*;
use aoc23::problem::*;
use itertools::*;

type Grid = Vec<Vec<char>>;

fn start(grid: &Grid) -> UCoord {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|c| grid[c.0][c.1] == 'S')
        .unwrap()
}

fn inside(grid: &Grid, c: &UCoord) -> bool {
    c.0 < grid.len() && c.1 < grid[c.0].len()
}

#[derive(Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn mover(c: &UCoord, d: Dir) -> Option<UCoord> {
    match d {
        Dir::N => c.0.checked_sub(1).map(|c0| (c0, c.1)),
        Dir::E => Some((c.0, c.1 + 1)),
        Dir::S => Some((c.0 + 1, c.1)),
        Dir::W => c.1.checked_sub(1).map(|c1| (c.0, c1)),
    }
}

fn directions(grid: &Grid, c: &UCoord) -> [Option<Dir>; 2] {
    match grid[c.0][c.1] {
        '|' => [Some(Dir::N), Some(Dir::S)],
        '-' => [Some(Dir::E), Some(Dir::W)],
        '7' => [Some(Dir::W), Some(Dir::S)],
        'F' => [Some(Dir::E), Some(Dir::S)],
        'L' => [Some(Dir::E), Some(Dir::N)],
        'J' => [Some(Dir::W), Some(Dir::N)],
        _ => [None, None],
    }
}

fn adjacents(grid: &Grid, c: &UCoord) -> [Option<UCoord>; 2] {
    if !inside(grid, c) {
        return [None, None];
    }
    let dirs = directions(grid, c);
    [
        dirs[0].and_then(|d| mover(c, d)),
        dirs[1].and_then(|d| mover(c, d)),
    ]
}

fn lefter(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::W,
        Dir::E => Dir::N,
        Dir::S => Dir::E,
        Dir::W => Dir::S,
    }
}

fn righter(d: Dir) -> Dir {
    match d {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

type Cset = std::collections::HashSet<UCoord>;

fn go_for_walk(grid: &Grid, s: UCoord, d: Dir) -> Option<(u64, Cset, Cset, Cset)> {
    let mut cur_heading = d;
    let mut cur_loc = s;
    let mut lefts = std::collections::HashSet::<UCoord>::new();
    let mut rights = std::collections::HashSet::<UCoord>::new();
    let mut path = std::collections::HashSet::<UCoord>::new();
    for length in 1.. {
        path.insert(cur_loc);
        if let Some(l) = mover(&cur_loc, lefter(cur_heading)) {
            lefts.insert(l);
            if let Some(ll) = mover(&l, cur_heading) {
                lefts.insert(ll);
            }
        }
        if let Some(r) = mover(&cur_loc, righter(cur_heading)) {
            rights.insert(r);
            if let Some(rr) = mover(&r, cur_heading) {
                rights.insert(rr);
            }
        }
        let next_loc = mover(&cur_loc, cur_heading).filter(|l| inside(grid, l));
        if next_loc.is_none() {
            return None;
        }
        let next_loc = next_loc.unwrap();
        if next_loc == s {
            return Some((length, path, lefts, rights));
        }
        let nld = directions(grid, &next_loc)
            .into_iter()
            .find(|ond| {
                ond.map(|nd| mover(&next_loc, nd))
                    .flatten()
                    .map(|nnl| nnl != cur_loc)
                    .unwrap_or(false)
            })
            .flatten();
        if nld.is_none() {
            return None;
        }
        cur_loc = next_loc;
        cur_heading = nld.unwrap();
    }
    None
}

fn expand(grid: &Grid, path: &Cset, group: Cset) -> Option<u64> {
    let mut cur = group;
    loop {
        let size = cur.len();
        let mut nxt = cur.clone();
        for x in cur.iter() {
            let adj = x.adjacent_cardinal();
            if adj.len() != 4 {
                return None;
            }
            for a in adj {
                if !inside(grid, &a) {
                    return None;
                }
                if path.contains(&a) {
                    continue;
                }
                nxt.insert(a);
            }
        }
        if nxt.len() == size {
            return Some(size as u64);
        }
        cur = nxt;
    }
}

fn walker(grid: &Grid) -> (u64, u64) {
    let s = start(grid);
    for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
        if !mover(&s, d)
            .map(|ns| adjacents(grid, &ns).iter().flatten().any(|ac| *ac == s))
            .unwrap_or(false)
        {
            continue;
        }
        let result = go_for_walk(grid, s, d);
        if result.is_none() {
            continue;
        }
        let (distance, path, lefts, rights) = result.unwrap();
        let ls = expand(
            grid,
            &path,
            lefts
                .difference(&path)
                .cloned()
                .filter(|c| inside(grid, c))
                .collect(),
        );
        let rs = expand(
            grid,
            &path,
            rights
                .difference(&path)
                .cloned()
                .filter(|c| inside(grid, c))
                .collect(),
        );
        if ls.is_none() && rs.is_none() {
            panic!();
        }
        if ls.is_some() && rs.is_some() {
            panic!();
        }
        return ((distance + 1) / 2, ls.unwrap_or_else(|| rs.unwrap()));
    }
    panic!();
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
        walker(&grid).0
    }
    fn solve_2(&self, input: &str) -> u64 {
        let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
        walker(&grid).1
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 4);
        assert_eq!(
            Prob::new().solve_1(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            8
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Prob::new().solve_2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );
        assert_eq!(
            Prob::new().solve_2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
        assert_eq!(
            Prob::new().solve_2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }
}
