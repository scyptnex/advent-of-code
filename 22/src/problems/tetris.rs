use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

const HORZ: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const PLUS: [(usize, usize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
const CRNR: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const VERT: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const SQUA: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

fn next_block(count: usize) -> &'static [(usize, usize)] {
    match count % 5 {
        0 => HORZ.as_slice(),
        1 => PLUS.as_slice(),
        2 => CRNR.as_slice(),
        3 => VERT.as_slice(),
        _ => SQUA.as_slice(),
    }
}

trait Column {
    fn tower_height(&self) -> usize;
    fn occupied(&self, piece: (usize, usize)) -> bool;
    fn put(&mut self, block: &[(usize, usize)], block_offset: (usize, usize));
}

#[derive(Default)]
struct VectorColumn {
    col: Vec<[bool; 7]>,
}

impl VectorColumn {
    fn accomodate(&mut self, y: usize) {
        while self.col.len() <= y {
            self.col.push([false; 7]);
        }
    }

    fn put_piece(&mut self, piece: &(usize, usize)) {
        self.accomodate(piece.1);
        self.col[piece.1][piece.0] = true;
    }
}

impl Column for VectorColumn {
    fn tower_height(&self) -> usize {
        return self.col.len();
    }

    fn occupied(&self, piece: (usize, usize)) -> bool {
        if piece.1 >= self.col.len() {
            return false;
        }
        self.col[piece.1][piece.0]
    }

    fn put(&mut self, block: &[(usize, usize)], block_offset: (usize, usize)) {
        for b in block {
            let b_l = (b.0 + block_offset.0, b.1 + block_offset.1);
            self.put_piece(&b_l);
        }
    }
}

fn blow(
    c: &impl Column,
    block: &[(usize, usize)],
    off: (usize, usize),
    left: bool,
) -> (usize, usize) {
    if left {
        if off.0 == 0 {
            return off;
        }
        let nxt = (off.0 - 1, off.1);
        if can_move(c, block, &nxt) {
            nxt
        } else {
            off
        }
    } else {
        let nxt = (off.0 + 1, off.1);
        if can_move(c, block, &nxt) {
            nxt
        } else {
            off
        }
    }
}

fn can_move(c: &impl Column, block: &[(usize, usize)], new_off: &(usize, usize)) -> bool {
    !block
        .iter()
        .map(|(bx, by)| (bx + new_off.0, by + new_off.1))
        .any(|(bx, by)| bx >= 7 || c.occupied((bx, by)))
}

fn drop(c: &mut impl Column, cur_block: usize, cur_wind: usize, t: &Tetris) -> usize {
    let block = next_block(cur_block);
    let mut block_pos = (2, c.tower_height() + 3);
    let mut wind = cur_wind;
    loop {
        block_pos = blow(c, block, block_pos, t.wind_left(wind));
        wind += 1;
        if block_pos.1 == 0 || !can_move(c, block, &(block_pos.0, block_pos.1 - 1)) {
            c.put(block, block_pos);
            break;
        }
        block_pos = (block_pos.0, block_pos.1 - 1);
    }
    wind
}

fn project(rcd: &Rcd, nv: &Vec<(usize, usize)>, target: u64) -> u64 {
    let period = (nv[3].0 - nv[2].0) as u64;
    let g_period = (nv[3].1 - nv[2].1) as u64;

    let rdr: u64 = target - (nv[2].0 as u64);
    let cycles: usize = (rdr / period).try_into().unwrap();
    let rem: usize = TryInto::<usize>::try_into(rdr % period).unwrap() + nv[2].0;

    let has_rem = rcd
        .values()
        .find_map(|v| v.iter().find(|(r, _)| *r == rem))
        .unwrap();
    has_rem.1 as u64 + g_period * cycles as u64
}

#[derive(Default)]
pub struct Tetris {
    data: String,
}

impl Tetris {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }

    fn wind_left(&self, t: usize) -> bool {
        match self.data.as_bytes()[t % self.data.len()] {
            b'<' => true,
            _ => false,
        }
    }

    fn height_after(&self, n: usize) -> usize {
        let mut c = VectorColumn::default();
        let mut w = 0;
        for r in 0..n {
            w = drop(&mut c, r, w, self);
        }
        c.tower_height()
    }
}

type Rcd = HashMap<(usize, usize), Vec<(usize, usize)>>;

impl StructuredProblem for Tetris {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.height_after(2022))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let mut rcd: Rcd = Rcd::new();
        let mut c = VectorColumn::default();
        let mut w = 0;
        for r in 0.. {
            let k = (w % self.data.len(), r % 5);
            if !rcd.contains_key(&k) {
                rcd.insert(k, Vec::new());
            }
            let nv = rcd.get_mut(&k).unwrap();
            nv.push((r, c.tower_height()));
            if nv.len() > 3 {
                return Box::new(project(&rcd, rcd.get(&k).unwrap(), 1000000000000));
            }

            w = drop(&mut c, r, w, self);
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Tetris {
        let mut t = Tetris::default();
        t.read(
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_answer() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "3068");
        assert_eq!(format!("{}", t.solve_2()), "1514285714288");
    }

    #[test]
    fn test_height() {
        let t = data();
        assert_eq!(t.height_after(10), 17);
    }

    #[test]
    fn test_blow() {
        let c = VectorColumn::default();
        assert_eq!(blow(&c, HORZ.as_slice(), (0, 0), true), (0, 0));
        assert_eq!(blow(&c, HORZ.as_slice(), (1, 0), true), (0, 0));
        assert_eq!(blow(&c, HORZ.as_slice(), (1, 0), false), (2, 0));
        assert_eq!(blow(&c, HORZ.as_slice(), (3, 0), false), (3, 0));
    }

    #[test]
    fn test_block() {
        assert_eq!(
            next_block(2)
                .iter()
                .copied()
                .collect::<Vec<(usize, usize)>>(),
            Vec::from(CRNR)
        );
    }

    #[test]
    fn test_wind() {
        let t = data();
        assert_eq!(t.wind_left(0), false);
        assert_eq!(t.wind_left(4), true);
        assert_eq!(t.wind_left(1337), false);
    }

    #[test]
    fn test_drop() {
        let t = data();
        let mut c = VectorColumn::default();
        let w = drop(&mut c, 0, 0, &t);

        assert_eq!(c.tower_height(), 1);
        assert_eq!(
            (0..7).map(|i| c.occupied((i, 0))).collect::<Vec<bool>>(),
            vec![false, false, true, true, true, true, false]
        );

        drop(&mut c, 1, w, &t);
        assert_eq!(c.tower_height(), 4);
        assert_eq!(
            (0..7).map(|i| c.occupied((i, 3))).collect::<Vec<bool>>(),
            vec![false, false, false, true, false, false, false]
        );
    }
}
