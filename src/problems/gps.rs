use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

fn mix2(v: &Vec<i64>, times: usize, dc: i64) -> Vec<i64> {
    let mut vm: Vec<(usize, i64)> = v.iter().map(|i| i * dc).enumerate().collect();
    for _ in 0..times {
        for i in 0..v.len() {
            let cidx = vm.iter().position(|(x, _)| *x == i).unwrap();
            let itm = vm.remove(cidx);
            let mvmnt = itm.1.rem_euclid(vm.len() as i64);
            let new_idx = (mvmnt as usize + cidx) % vm.len();
            vm.insert(new_idx, itm);
        }
    }
    vm.into_iter().map(|(_, i)| i).collect()
}

fn mix(v: &Vec<i64>) -> Vec<i64> {
    mix2(v, 1, 1)
}

fn sigproc(v: &Vec<i64>) -> i64 {
    let x = v.iter().enumerate().find(|(_, x)| **x == 0).unwrap().0;
    [1000, 2000, 3000]
        .iter()
        .map(|i| v[(x + i) % v.len()])
        .sum()
}

#[derive(Default)]
pub struct Gps {
    data: Vec<i64>,
}

impl Gps {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.map(|s| s.parse::<i64>().unwrap()).collect();
    }
}

impl StructuredProblem for Gps {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(sigproc(&mix(&self.data)))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(sigproc(&mix2(&self.data, 10, 811589153)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Gps {
        let mut t = Gps::default();
        t.read("1\n2\n-3\n3\n-2\n0\n4".lines().map(|s| String::from(s)));
        t
    }

    #[test]
    fn test_gps() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "3");
        assert_eq!(format!("{}", t.solve_2()), "1623178306");
    }

    #[test]
    fn test_mod() {
        assert_eq!(-2 % 5, -2);
        assert_eq!(-1 % 5, -1);
        assert_eq!(-5 % 5, 0);
    }
}
