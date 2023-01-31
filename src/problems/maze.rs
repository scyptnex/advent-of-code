use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

extern crate nom;
use nom::{IResult, Parser};

use super::StructuredProblem;

fn to_bounds<I: Iterator<Item = usize>>(i: I) -> RangeInclusive<usize> {
    let mut i = i;
    let s = i.next().unwrap();
    s..=i.last().unwrap()
}

fn warp_inside(d: usize, range: RangeInclusive<usize>, go_positive: bool) -> usize {
    match go_positive {
        true => {
            if d == *range.end() {
                *range.start()
            } else {
                d + 1
            }
        }
        false => {
            if d == *range.start() {
                *range.end()
            } else {
                d - 1
            }
        }
    }
}

fn adjacent(cube: &Vec<Vec<bool>>, r: usize, c: usize, f: &Facing) -> Option<(usize, usize)> {
    let (r_pos, r_neg, c_pos, c_neg) = match f {
        Right => (0, 0, 1, 0),
        Down => (1, 0, 0, 0),
        Left => (0, 0, 0, 1),
        Up => (0, 1, 0, 0),
    };

    (r + r_pos)
        .checked_sub(r_neg)
        .and_then(|new_r| (c + c_pos).checked_sub(c_neg).map(|new_c| (new_r, new_c)))
        .filter(|(new_r, new_c)| {
            cube.len() > *new_r && cube[*new_r].len() > *new_c && cube[*new_r][*new_c]
        })
}

fn fold_cube(cube: Vec<Vec<bool>>) -> Vec<Vec<[(usize, usize, Facing); 4]>> {
    let faces: Vec<(usize, usize)> = cube
        .iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_i, val)| if *val { Some((row_i, col_i)) } else { None })
        })
        .collect();
    assert!(faces.len() == 6);

    let mut scratch: HashMap<(usize, usize, Facing), (usize, usize, Facing)> = HashMap::new();
    for f in &faces {
        for adj in [Right, Down, Left, Up] {
            if let Some((ar, ac)) = adjacent(&cube, f.0, f.1, &adj) {
                scratch.insert((f.0, f.1, adj), (ar, ac, adj.clockwise().clockwise()));
            }
        }
    }

    while scratch.len() != 24 {
        for f in &faces {
            for d in [Right, Down, Left, Up] {
                let d_clock = d.clockwise();
                let d_adj = scratch.get(&(f.0, f.1, d));
                let d_clock_adj = scratch.get(&(f.0, f.1, d_clock));
                if d_adj.is_none() || d_clock_adj.is_none() {
                    continue;
                }
                let d_new = d_adj
                    .map(|(r, c, h)| (*r, *c, h.counter_clockwise()))
                    .unwrap()
                    .clone();
                let d_clock_new = d_clock_adj
                    .map(|(r, c, h)| (*r, *c, h.clockwise()))
                    .unwrap();

                let d_clock_old = scratch.insert(d_new, d_clock_new);
                if d_clock_old.is_some() {
                    assert_eq!(d_clock_old.unwrap(), d_clock_new);
                }
                scratch.insert(d_clock_new, d_new);
            }
        }
    }

    (0..cube.len())
        .map(|r| {
            (0..cube[r].len())
                .map(|c| {
                    [
                        scratch.remove(&(r, c, Right)).unwrap_or((99, 99, Right)),
                        scratch.remove(&(r, c, Down)).unwrap_or((99, 99, Right)),
                        scratch.remove(&(r, c, Left)).unwrap_or((99, 99, Right)),
                        scratch.remove(&(r, c, Up)).unwrap_or((99, 99, Right)),
                    ]
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Nothing,
    Open,
    Wall,
}

use Cell::*;

impl Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Open,
            '#' => Wall,
            _ => Nothing,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Seq {
    Clockwise,
    CounterClockwise,
    Forward(usize),
}

use Seq::*;

impl Seq {
    fn parse(input: &str) -> Vec<Self> {
        nom::multi::many1(nom::branch::alt((
            Seq::parse_turn,
            nom::character::complete::u32.map(|i| Seq::Forward(i as usize)),
        )))(input)
        .unwrap()
        .1
    }

    fn parse_turn(input: &str) -> IResult<&str, Seq> {
        nom::branch::alt((
            nom::character::complete::char('L').map(|_| Seq::CounterClockwise),
            nom::character::complete::char('R').map(|_| Seq::Clockwise),
        ))(input)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

use Facing::*;

impl Facing {
    fn clockwise(&self) -> Self {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn counter_clockwise(&self) -> Self {
        self.clockwise().clockwise().clockwise()
    }

    fn score(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

struct Hdr {
    row: usize,
    col: usize,
    facing: Facing,
    warps: Option<(usize, Vec<Vec<[(usize, usize, Facing); 4]>>)>,
}

impl Hdr {
    fn begin(maze: &Maze) -> Self {
        Hdr {
            row: 0,
            col: *maze.x_bounds(0).start(),
            facing: Right,
            warps: None,
        }
    }

    fn score(&self) -> usize {
        (self.row + 1) * 1000 + (self.col + 1) * 4 + self.facing.score()
    }

    fn go_forward(&mut self, maze: &Maze, steps: usize) {
        for _ in 0..steps {
            if self.warps.is_some() {
                if !self.next_step_warps(maze) {
                    break;
                }
            } else if !self.next_step_no_warps(maze) {
                break;
            }
        }
    }

    fn warp(&self) -> (usize, usize, Facing) {
        let side_len = self.warps.as_ref().unwrap().0;
        let (cube_r, cube_c) = (self.row / side_len, self.col / side_len);
        let next = self.warps.as_ref().unwrap().1[cube_r][cube_c][self.facing.score()];

        let distance_on_left = match self.facing {
            Right => self.row % side_len,
            Down => side_len - (self.col % side_len) - 1,
            Left => side_len - (self.row % side_len) - 1,
            Up => self.col % side_len,
        };

        match next.2 {
            Right => (
                (next.0 + 1) * side_len - distance_on_left - 1,
                (next.1 + 1) * side_len - 1,
                Left,
            ),
            Down => (
                (next.0 + 1) * side_len - 1,
                next.1 * side_len + distance_on_left,
                Up,
            ),
            Left => (
                next.0 * side_len + distance_on_left,
                next.1 * side_len,
                Right,
            ),
            Up => (
                next.0 * side_len,
                (next.1 + 1) * side_len - distance_on_left - 1,
                Down,
            ),
        }
    }

    fn next_step_warps(&mut self, maze: &Maze) -> bool {
        let side_len = self.warps.as_ref().unwrap().0;
        let potential = match self.facing {
            Right => {
                if self.col % side_len == side_len - 1 {
                    self.warp()
                } else {
                    (self.row, self.col + 1, Right)
                }
            }
            Down => {
                if self.row % side_len == side_len - 1 {
                    self.warp()
                } else {
                    (self.row + 1, self.col, Down)
                }
            }
            Left => {
                if self.col % side_len == 0 {
                    self.warp()
                } else {
                    (self.row, self.col - 1, Left)
                }
            }
            Up => {
                if self.row % side_len == 0 {
                    self.warp()
                } else {
                    (self.row - 1, self.col, Up)
                }
            }
        };
        if maze.data[potential.0][potential.1] != Open {
            false
        } else {
            (self.row, self.col, self.facing) = potential;
            true
        }
    }

    fn next_step_no_warps(&mut self, maze: &Maze) -> bool {
        let (r, c) = (self.row, self.col);
        let potential = match self.facing {
            Right => (r, warp_inside(c, maze.x_bounds(r), true)),
            Down => (warp_inside(r, maze.y_bounds(c), true), c),
            Left => (r, warp_inside(c, maze.x_bounds(r), false)),
            Up => (warp_inside(r, maze.y_bounds(c), false), c),
        };
        if maze.data[potential.0][potential.1] != Open {
            false
        } else {
            (self.row, self.col) = potential;
            true
        }
    }

    fn act(&mut self, maze: &Maze, action: &Seq) {
        match action {
            Clockwise => self.facing = self.facing.clockwise(),
            CounterClockwise => self.facing = self.facing.counter_clockwise(),
            Forward(i) => self.go_forward(maze, *i),
        }
    }
}

#[derive(Default)]
pub struct Maze {
    data: Vec<Vec<Cell>>,
    seq: Vec<Seq>,
}

impl Maze {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        let mut i = i;
        self.data = i
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|s| s.chars().map(Cell::from).collect())
            .collect();
        self.seq = Seq::parse(&i.next().unwrap());
    }

    fn y_bounds(&self, x: usize) -> RangeInclusive<usize> {
        to_bounds(self.data.iter().enumerate().filter_map(|(i, row)| {
            if x >= row.len() {
                return None;
            }
            match row[x] {
                Cell::Nothing => None,
                _ => Some(i),
            }
        }))
    }

    fn x_bounds(&self, y: usize) -> RangeInclusive<usize> {
        to_bounds(
            self.data[y]
                .iter()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    Cell::Nothing => None,
                    _ => Some(i),
                }),
        )
    }

    fn side_len(&self) -> usize {
        (0..self.data.len())
            .map(|i| self.x_bounds(i))
            .map(|r| r.end() - r.start() + 1)
            .min()
            .unwrap()
    }

    fn as_unit_cube(&self) -> Vec<Vec<bool>> {
        let sl = self.side_len();
        (0..self.data.len() / sl)
            .map(|i| &self.data[i * sl])
            .map(|v| v.chunks(sl).map(|c| c[0] != Nothing).collect())
            .collect()
    }
}

impl StructuredProblem for Maze {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        let mut h = Hdr::begin(&self);
        for m in &self.seq {
            h.act(self, &m);
        }
        Box::new(h.score())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let mut h = Hdr::begin(&self);
        h.warps = Some((self.side_len(), fold_cube(self.as_unit_cube())));
        for m in &self.seq {
            h.act(self, &m);
        }
        Box::new(h.score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Maze {
        let mut t = Maze::default();
        t.read(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_side() {
        let t = data();
        assert_eq!(t.side_len(), 4);

        assert_eq!(
            t.as_unit_cube(),
            vec![
                vec![false, false, true],
                vec![true, true, true],
                vec![false, false, true, true],
            ]
        );

        let c = fold_cube(t.as_unit_cube());
        assert_eq!(c[2][3][Left.score()], (2, 2, Right));
        assert_eq!(c[0][2][Down.score()], (1, 2, Up));

        assert_eq!(c[1][1][Up.score()], (0, 2, Left));
    }

    #[test]
    fn test_todo() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "6032");
        assert_eq!(format!("{}", t.solve_2()), "5031");
    }

    #[test]
    fn test_seq() {
        let t = data();
        assert_eq!(
            t.seq[..4],
            [Forward(10), Clockwise, Forward(5), CounterClockwise]
        )
    }

    #[test]
    fn test_move() {
        let maze = data();
        let mut hdr = Hdr {
            row: 0,
            col: 8,
            facing: Right,
            warps: None,
        };
        hdr.act(&maze, &Forward(5));
        assert_eq!(hdr.col, 10);
    }

    #[test]
    fn test_bounds() {
        let t = data();
        assert_eq!(t.x_bounds(3), 8..=11);
        assert_eq!(t.x_bounds(4), 0..=11);
        assert_eq!(t.y_bounds(1), 4..=7);
        assert_eq!(t.y_bounds(14), 8..=11);
    }
}
