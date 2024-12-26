use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Debug, Clone, Copy)]
enum Ins {
    Noop,
    Addx(i32),
}

struct CrtCpu<I> {
    x: i32,
    p: Option<i32>,
    i: I,
}

impl<'a, I: Iterator<Item = &'a Ins>> Iterator for CrtCpu<I> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.p.is_some() {
            let old_x = self.x;
            self.x += self.p.unwrap();
            self.p = None;
            return Some(old_x);
        }
        let next_i = self.i.next();
        if next_i.is_none() {
            return None;
        }
        match next_i.unwrap() {
            Ins::Noop => Some(self.x),
            Ins::Addx(v) => {
                self.p = Some(*v);
                Some(self.x)
            }
        }
    }
}

#[derive(Default)]
pub struct Crt {
    prog: Vec<Ins>,
}

impl Crt {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.prog = i
            .map(|s| match s.split_once(" ") {
                None => Ins::Noop,
                Some((_, v)) => Ins::Addx(v.parse().unwrap()),
            })
            .collect();
    }

    fn run_iter(&self) -> CrtCpu<impl Iterator<Item = &Ins>> {
        CrtCpu {
            x: 1,
            p: None,
            i: self.prog.iter(),
        }
    }
}

impl StructuredProblem for Crt {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(
            self.run_iter()
                .enumerate()
                .skip(19)
                .step_by(40)
                .map(|(t, x)| (t as i32 + 1) * x)
                .sum::<i32>(),
        )
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(
            self.run_iter()
                .enumerate()
                .map(|(t, x)| {
                    if (x - 1..=x + 1).contains(&(t as i32 % 40)) {
                        (t, "#")
                    } else {
                        (t, ".")
                    }
                })
                .map(|(t, s)| {
                    if t % 40 == 0 && t != 0 {
                        format!("\n{}", s)
                    } else {
                        s.to_string()
                    }
                })
                .collect::<String>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_crt() {
        let mut t = Crt::default();
        t.read(DATA.lines().map(|s| String::from(s)));

        assert_eq!(format!("{}", t.solve_1()), "13140");

        let expected_screen = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(format!("{}", t.solve_2()), expected_screen);
    }

    #[test]
    fn test_iter() {
        let mut t = Crt::default();
        t.read("noop\naddx 3\naddx -5".lines().map(|s| String::from(s)));
        assert_eq!(t.run_iter().collect::<Vec<i32>>(), vec![1, 1, 1, 4, 4]);
    }
}
