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

#[derive(Debug, Eq, PartialEq)]
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
}

impl Hdr {
    fn begin(maze: &Maze) -> Self {
        Hdr {
            row: 0,
            col: *maze.x_bounds(0).start(),
            facing: Right,
        }
    }

    fn score(&self) -> usize {
        (self.row + 1) * 1000 + (self.col + 1) * 4 + self.facing.score()
    }

    fn go_forward(&mut self, maze: &Maze, steps: usize) {
        for _ in 0..steps {
            if !self.next_step(maze) {
                break;
            }
        }
    }

    fn next_step(&mut self, maze: &Maze) -> bool {
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
        Box::new("Maze problem 2")
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
    fn test_todo() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "6032");
        assert_eq!(format!("{}", t.solve_2()), "Maze problem 2");
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
