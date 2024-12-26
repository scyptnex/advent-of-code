use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

enum SDigit {
    DMin,
    Min,
    Zer,
    Uni,
    Duo,
}

use SDigit::*;

impl SDigit {
    fn from(c: char) -> Self {
        match c {
            '=' => DMin,
            '-' => Min,
            '0' => Zer,
            '1' => Uni,
            '2' => Duo,
            _ => panic!(),
        }
    }

    fn encode(i: &i64) -> Self {
        match *i {
            -2 => DMin,
            -1 => Min,
            0 => Zer,
            1 => Uni,
            2 => Duo,
            _ => panic!(),
        }
    }

    fn val(&self) -> i64 {
        match self {
            DMin => -2,
            Min => -1,
            Zer => 0,
            Uni => 1,
            Duo => 2,
        }
    }

    fn cha(&self) -> char {
        match self {
            DMin => '=',
            Min => '-',
            Zer => '0',
            Uni => '1',
            Duo => '2',
        }
    }
}

struct Encoded {
    digits: Vec<SDigit>,
}

impl Encoded {
    fn from(s: &str) -> Self {
        Encoded {
            digits: s.chars().map(SDigit::from).collect(),
        }
    }

    fn encode(i: i64) -> Self {
        let mut ltm_digits: Vec<i64> = Vec::new();
        let mut i = i;
        while i != 0 {
            ltm_digits.push(i % 5);
            i = i / 5;
        }

        let real_len = ltm_digits.len();
        ltm_digits.push(0);

        for d in 0..real_len {
            if ltm_digits[d] <= 2 {
                continue;
            }
            ltm_digits[d] = ltm_digits[d] - 5;
            ltm_digits[d + 1] = ltm_digits[d + 1] + 1;
        }
        if *ltm_digits.last().unwrap() == 0 && real_len > 0 {
            ltm_digits.pop();
        }
        Encoded {
            digits: ltm_digits.iter().rev().map(SDigit::encode).collect(),
        }
    }

    fn str(&self) -> String {
        self.digits.iter().map(SDigit::cha).collect()
    }

    fn decimal(&self) -> i64 {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| (5 as i64).pow(i as u32) * d.val())
            .sum()
    }
}

#[derive(Default)]
pub struct Snafu {
    data: Vec<String>,
}

impl Snafu {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }
}

impl StructuredProblem for Snafu {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(
            Encoded::encode(
                self.data
                    .iter()
                    .map(String::as_str)
                    .map(Encoded::from)
                    .map(|e| e.decimal())
                    .sum(),
            )
            .str(),
        )
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new("Snafu problem 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu() {
        let mut t = Snafu::default();
        t.read("".lines().map(|s| String::from(s)));

        assert_eq!(format!("{}", t.solve_2()), "Snafu problem 2");
    }

    #[test]
    fn test_decimal() {
        assert_eq!(Encoded::from("1=-0-2").decimal(), 1747);
        assert_eq!(Encoded::from("12111").decimal(), 906);
        assert_eq!(Encoded::from("2=0=").decimal(), 198);
        assert_eq!(Encoded::from("21").decimal(), 11);
        assert_eq!(Encoded::from("2=01").decimal(), 201);
        assert_eq!(Encoded::from("111").decimal(), 31);
        assert_eq!(Encoded::from("20012").decimal(), 1257);
        assert_eq!(Encoded::from("112").decimal(), 32);
        assert_eq!(Encoded::from("1=-1=").decimal(), 353);
        assert_eq!(Encoded::from("1-12").decimal(), 107);
        assert_eq!(Encoded::from("12").decimal(), 7);
        assert_eq!(Encoded::from("1=").decimal(), 3);
        assert_eq!(Encoded::from("122").decimal(), 37);
    }

    #[test]
    fn test_encode() {
        assert_eq!(Encoded::encode(4890).str(), "2=-1=0");
    }
}
