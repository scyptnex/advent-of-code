use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

extern crate nom;
use nom::{Finish, IResult, Parser};

use super::StructuredProblem;

#[derive(Debug, Eq)]
enum Lzt {
    Val(i32),
    Seq(Vec<Lzt>),
}
use Lzt::*;

impl Lzt {
    fn p1() -> Self {
        Seq(vec![Seq(vec![Val(2)])])
    }
    fn p2() -> Self {
        Seq(vec![Seq(vec![Val(6)])])
    }
}

impl FromStr for Lzt {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_lzt(s).finish() {
            Ok((_, l)) => Ok(l),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl PartialEq for Lzt {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val(sv), Val(ov)) => sv == ov,
            (Val(_), Seq(ov)) => ov.len() == 1 && self == ov.first().unwrap(),
            (Seq(sv), Val(_)) => sv.len() == 1 && sv.first().unwrap() == other,
            (Seq(sv), Seq(ov)) => sv == ov,
        }
    }
}

impl PartialOrd for Lzt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lzt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Val(sv), Val(ov)) => sv.cmp(ov),
            (Val(sv), Seq(_)) => Seq(vec![Val(*sv)]).cmp(other),
            (Seq(_), Val(ov)) => self.cmp(&Seq(vec![Val(*ov)])),
            (Seq(sv), Seq(ov)) => sv
                .iter()
                .zip(ov.iter())
                .find_map(|(si, oi)| match si.cmp(oi) {
                    std::cmp::Ordering::Equal => None,
                    x => Some(x),
                })
                .unwrap_or_else(|| sv.len().cmp(&ov.len())),
        }
    }
}

fn parse_lzt(input: &str) -> IResult<&str, Lzt> {
    nom::branch::alt((
        nom::character::complete::i32.map(|i| Val(i)),
        nom::sequence::delimited(
            nom::character::complete::char('['),
            nom::multi::separated_list0(nom::character::complete::char(','), parse_lzt),
            nom::character::complete::char(']'),
        )
        .map(|v| Seq(v)),
    ))(input)
}

#[derive(Default)]
pub struct Distress {
    data: Vec<String>,
}

impl Distress {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }
}

impl StructuredProblem for Distress {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(
            self.data
                .chunks(3)
                .enumerate()
                .map(|(i, s)| {
                    (
                        i,
                        s[0].parse::<Lzt>().unwrap(),
                        s[1].parse::<Lzt>().unwrap(),
                    )
                })
                .filter(|(_, l, r)| l < r)
                .map(|(i, _, _)| i + 1)
                .sum::<usize>(),
        )
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let mut pck: Vec<Lzt> = self
            .data
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Lzt>().unwrap())
            .collect();
        pck.push(Lzt::p1());
        pck.push(Lzt::p2());
        pck.sort();
        Box::new(
            pck.iter()
                .enumerate()
                .filter(|(_, l)| *l == &Lzt::p1() || *l == &Lzt::p2())
                .map(|(i, _)| i + 1)
                .product::<usize>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let mut t = Distress::default();
        t.read(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
                .lines()
                .map(|s| String::from(s)),
        );

        assert_eq!(format!("{}", t.solve_1()), "13");
        assert_eq!(format!("{}", t.solve_2()), "140");
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_lzt("42"), Ok(("", Val(42))));

        assert!(parse_lzt("").is_err());
        assert_eq!(parse_lzt("[]"), Ok(("", Seq(vec![]))));
        assert_eq!(parse_lzt("[1]"), Ok(("", Seq(vec![Val(1)]))));
        assert_eq!(parse_lzt("[2,3]"), Ok(("", Seq(vec![Val(2), Val(3)]))));

        assert_eq!(parse_lzt("4"), Ok(("", Val(4))));
        assert_eq!(parse_lzt("[4]"), Ok(("", Seq(vec![Val(4)]))));
        assert_eq!(parse_lzt("[[4]]"), Ok(("", Seq(vec![Seq(vec![Val(4)])]))));

        assert_eq!("[[4]]".parse(), Ok(Seq(vec![Seq(vec![Val(4)])])));
        assert_eq!(
            "[[1],[2,3,4]]".parse(),
            Ok(Seq(vec![
                Seq(vec![Val(1)]),
                Seq(vec![Val(2), Val(3), Val(4)])
            ]))
        );
    }

    #[test]
    fn test_eq() {
        assert_ne!(Val(1), Val(2));
        assert_eq!(Val(1), Val(1));
        assert_eq!(Seq(vec![Val(1)]), Seq(vec![Val(1)]));
        assert_eq!(Seq(vec![Val(1)]), Val(1));
    }

    #[test]
    fn test_ord() {
        assert!("[1,1,3,1,1]".parse::<Lzt>().unwrap() < "[1,1,5,1,1]".parse().unwrap());
        assert!("[[1],4]".parse::<Lzt>().unwrap() > "[[1],[2,3,4]]".parse().unwrap());
    }
}
