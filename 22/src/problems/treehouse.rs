use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct Treehouse {
    tmap: Vec<Vec<u8>>,
}

fn directions() -> impl Iterator<Item = Direction> {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
}

impl Treehouse {
    fn read<I: IntoIterator<Item = String>>(&mut self, i: I) {
        self.tmap = i
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
    }

    fn height(&self) -> usize {
        self.tmap.len()
    }

    fn width(&self) -> usize {
        self.tmap.first().unwrap().len()
    }

    fn tree_height(&self, x: usize, y: usize) -> u8 {
        self.tmap[y as usize][x as usize]
    }

    fn distance_to_edge(&self, (x, y): (usize, usize), d: Direction) -> usize {
        match d {
            Direction::Up => y,
            Direction::Down => self.height() - y - 1,
            Direction::Left => x,
            Direction::Right => self.width() - x - 1,
        }
    }

    fn from(&self, (x, y): (usize, usize), d: Direction) -> Box<dyn Iterator<Item = u8> + '_> {
        match d {
            Direction::Up => Box::new(self.tmap[..y].iter().rev().map(move |v| v[x])),
            Direction::Down => Box::new(self.tmap[y + 1..].iter().map(move |v| v[x])),
            Direction::Left => Box::new(self.tmap[y][..x].iter().rev().copied()),
            Direction::Right => Box::new(self.tmap[y][x + 1..].iter().copied()),
        }
    }

    fn is_visible(&self, (x, y): (usize, usize)) -> bool {
        let tree_height = self.tree_height(x, y);
        directions().any(|d| {
            self.from((x as usize, y as usize), d)
                .all(|h| h < tree_height)
        })
    }

    fn scenic_score(&self, (x, y): (usize, usize)) -> usize {
        let tree_height = self.tree_height(x, y);
        directions()
            .map(|d| {
                self.from((x as usize, y as usize), d)
                    .enumerate()
                    .find(|(_, h)| *h >= tree_height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(self.distance_to_edge((x, y), d))
            })
            .product()
    }

    fn count_visible(&self) -> usize {
        (0..self.width())
            .flat_map(|x| (0..self.height()).map(move |y| (x, y)))
            .filter(|c| self.is_visible(*c))
            .count()
    }

    fn best_scenic_score(&self) -> usize {
        (1..self.width() - 1)
            .flat_map(|x| (1..self.height() - 1).map(move |y| (x, y)))
            .map(|c| self.scenic_score(c))
            .max()
            .unwrap()
    }
}

impl StructuredProblem for Treehouse {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.count_visible())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.best_scenic_score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Treehouse {
        let mut t = Treehouse::default();
        t.read(
            "30373
25512
65332
33549
35390"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_treehouse() {
        let t = test_data();
        assert_eq!(format!("{}", t.solve_1()), "21");
        assert_eq!(format!("{}", t.solve_2()), "8");
    }

    #[test]
    fn test_visisble() {
        let t = test_data();
        assert_eq!(t.is_visible((0, 0)), true);
        assert_eq!(t.is_visible((3, 1)), false);
    }

    #[test]
    fn test_scenic() {
        let t = test_data();
        assert_eq!(t.scenic_score((2, 1)), 4);
    }

    #[test]
    fn test_from() {
        let t = test_data();
        assert_eq!(t.from((1, 1), Direction::Up).collect::<Vec<u8>>(), vec![0]);
        assert_eq!(
            t.from((1, 1), Direction::Down).collect::<Vec<u8>>(),
            vec![5, 3, 5]
        );
        assert_eq!(
            t.from((1, 1), Direction::Left).collect::<Vec<u8>>(),
            vec![2]
        );
        assert_eq!(
            t.from((1, 1), Direction::Right).collect::<Vec<u8>>(),
            vec![5, 1, 2]
        );
    }
}
