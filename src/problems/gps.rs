use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

fn mix2(v: &Vec<i64>, times: usize, dc: i64) -> Vec<i64> {
    let len = v.len();
    let mut vm: Vec<(usize, i64)> = v.iter().map(|i| i * dc).enumerate().collect();

    for _ in 0..times {
        for i in 0..len {
            let cur = vm.iter().enumerate().find(|(_, (x, _))| *x == i).unwrap();
            let mut mvmnt = cur.1 .1 % (len as i64 - 1);
            let mut cidx = cur.0;
            while mvmnt != 0 {
                let nidx = if mvmnt > 0 {
                    mvmnt -= 1;
                    if cidx == len - 1 {
                        0
                    } else {
                        cidx + 1
                    }
                } else {
                    mvmnt += 1;
                    if cidx == 0 {
                        len - 1
                    } else {
                        cidx - 1
                    }
                };
                let tmp = vm[nidx];
                vm[nidx] = vm[cidx];
                vm[cidx] = tmp;
                cidx = nidx;
            }
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
