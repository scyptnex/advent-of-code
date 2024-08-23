use aoc23::problem::*;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Us3([usize; 3]);

impl FromStr for Us3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, yz) = s.split_once(',').unwrap();
        let (y, z) = yz.split_once(',').unwrap();
        Ok(Us3([
            x.parse::<usize>().unwrap(),
            y.parse().unwrap(),
            z.parse().unwrap(),
        ]))
    }
}

#[derive(Debug)]
struct Blk([RangeInclusive<usize>; 3]);

impl FromStr for Blk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('~').unwrap();
        let l = l.parse::<Us3>().unwrap();
        let r = r.parse::<Us3>().unwrap();
        Ok(Blk([
            min(l.0[0], r.0[0])..=max(l.0[0], r.0[0]),
            min(l.0[1], r.0[1])..=max(l.0[1], r.0[1]),
            min(l.0[2], r.0[2])..=max(l.0[2], r.0[2]),
        ]))
    }
}

type Floor = HashMap<(usize, usize), (usize, usize)>;
type Resting = Vec<Vec<usize>>;

impl Blk {
    fn drop(&mut self, si: usize, floor: &mut Floor, resting_on: &mut Resting) {
        assert!(resting_on.len() == si);
        let mut rst = HashSet::new();
        let mut lowest_zr = 0;
        for x in self.0[0].clone() {
            for y in self.0[1].clone() {
                for _ in self.0[2].clone() {
                    let (rz, ri) = *floor.get(&(x, y)).unwrap_or(&(0, si));
                    if lowest_zr < rz {
                        lowest_zr = rz;
                        rst = HashSet::new();
                    }
                    if lowest_zr == rz && ri != si {
                        rst.insert(ri);
                    }
                }
            }
        }
        let zd = self.0[2].start() - lowest_zr;
        self.0[2] = (self.0[2].start() - zd + 1)..=(self.0[2].end() - zd + 1);
        for x in self.0[0].clone() {
            for y in self.0[1].clone() {
                for z in self.0[2].clone() {
                    floor.insert((x, y), (z, si));
                }
            }
        }
        resting_on.push(rst.into_iter().collect());
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
        let mut falling: Vec<Blk> = input
            .lines()
            .map(Blk::from_str)
            .map(|r| r.unwrap())
            .collect();
        falling.sort_by_key(|b| *b.0[2].start());
        let mut floor = Floor::new();
        let mut resting_on = Resting::new();
        falling.iter_mut().enumerate().for_each(|(i, b)| {
            b.drop(i, &mut floor, &mut resting_on);
        });
        let mut rested_on_by: Resting = vec![Vec::new(); falling.len()];
        let mut solos = HashSet::<usize>::new();
        resting_on.iter().enumerate().for_each(|(i, rv)| {
            for r in rv {
                rested_on_by[*r].push(i);
            }
            if rv.len() == 1 {
                solos.insert(rv[0]);
            }
        });
        (falling.len() - solos.len()) as u64
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut falling: Vec<Blk> = input
            .lines()
            .map(Blk::from_str)
            .map(|r| r.unwrap())
            .collect();
        falling.sort_by_key(|b| *b.0[2].start());
        let mut floor = Floor::new();
        let mut resting_on = Resting::new();
        falling.iter_mut().enumerate().for_each(|(i, b)| {
            b.drop(i, &mut floor, &mut resting_on);
        });
        let mut rested_on_by: Resting = vec![Vec::new(); falling.len()];
        resting_on.iter().enumerate().for_each(|(i, rv)| {
            for r in rv {
                rested_on_by[*r].push(i);
            }
        });

        let mut ret = 0;
        for dis_i in 0..falling.len() {
            let mut definitely_falling = HashSet::<usize>::new();
            definitely_falling.insert(dis_i);
            let mut above_falling = vec![dis_i];
            while let Some(cur_drop) = above_falling.pop() {
                for potential_new_fall in &rested_on_by[cur_drop] {
                    // if everything the potential fall is falling, and it wasn't in the fallen set
                    if resting_on[*potential_new_fall]
                        .iter()
                        .all(|pnf_rest| definitely_falling.contains(pnf_rest))
                        && definitely_falling.insert(*potential_new_fall)
                    {
                        above_falling.push(*potential_new_fall);
                    }
                }
            }
            ret += definitely_falling.len() - 1;
        }
        ret as u64
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 7);
    }
}
