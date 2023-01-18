use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Debug, Clone, Copy)]
enum Ins {
    Noop,
    Addx(i32),
}

trait Actor {
    fn act(&mut self, t: usize, x: i32);
}

#[derive(Default)]
struct SigStrength {
    ss: i32,
}

impl Actor for SigStrength {
    fn act(&mut self, t: usize, x: i32) {
        if t < 20 {
            return;
        }
        if (t - 20) % 40 != 0 {
            return;
        }
        self.ss += x * t as i32
    }
}

#[derive(Default)]
struct Lcd {
    display: String,
}
impl Actor for Lcd {
    fn act(&mut self, t: usize, x: i32) {
        let px: i32 = (t as i32 - 1) % 40;
        if px == 0 && t != 1 {
            self.display.push('\n');
        }
        if (px - x).abs() <= 1 {
            self.display.push('#');
        } else {
            self.display.push('.');
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

    fn run(&self, a: &mut dyn Actor) {
        let mut t = 1;
        let mut x = 1;
        for i in self.prog.iter() {
            a.act(t, x);
            match i {
                Ins::Noop => {
                    // Do nothing
                }
                Ins::Addx(v) => {
                    t += 1;
                    a.act(t, x);
                    x += v;
                }
            }
            t += 1;
        }
    }
}

impl StructuredProblem for Crt {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        let mut ss = SigStrength::default();
        self.run(&mut ss);
        Box::new(ss.ss)
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let mut lcd = Lcd::default();
        self.run(&mut lcd);
        Box::new(lcd.display)
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
}
