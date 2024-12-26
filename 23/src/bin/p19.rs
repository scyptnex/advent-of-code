use aoc23::problem::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Dst<'a> {
    R,
    A,
    Go(&'a str),
}

impl<'a> From<&'a str> for Dst<'a> {
    fn from(d: &'a str) -> Self {
        if d == "R" {
            Dst::R
        } else if d == "A" {
            Dst::A
        } else {
            Dst::Go(d)
        }
    }
}

#[derive(Debug)]
enum Fld {
    X,
    M,
    A,
    S,
}

impl FromStr for Fld {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Fld::X),
            "m" => Ok(Fld::M),
            "a" => Ok(Fld::A),
            "s" => Ok(Fld::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Cmp {
    Lt,
    Gt,
}

type Rules<'a> = HashMap<(&'a str, usize), (Option<(Fld, Cmp, u64)>, Dst<'a>)>;

type Part = (u64, u64, u64, u64);

type Parts = Vec<Part>;

type Range = (u64, u64);

type Bounds = [Range; 4];

fn parse_rpiece(rpiece: &str) -> (Option<(Fld, Cmp, u64)>, Dst) {
    if let Some((cond, dest)) = rpiece.split_once(':') {
        if let Some((lhs, rhs)) = cond.split_once('<') {
            (
                Some((lhs.parse().unwrap(), Cmp::Lt, rhs.parse().unwrap())),
                Dst::from(dest),
            )
        } else {
            let (lhs, rhs) = cond.split_once('>').unwrap();
            (
                Some((lhs.parse().unwrap(), Cmp::Gt, rhs.parse().unwrap())),
                Dst::from(dest),
            )
        }
    } else {
        (None, Dst::from(rpiece))
    }
}

fn parse_rules(rs: &str) -> Rules {
    let mut ret = Rules::new();
    for line in rs.lines() {
        let (rname, rbody) = line.split_once("{").unwrap();
        let rbody = &rbody[..rbody.len() - 1];
        for (idx, rpiece) in rbody.split(',').enumerate() {
            ret.insert((rname, idx), parse_rpiece(rpiece));
        }
    }
    ret
}

fn parse_pval(pv: &str) -> u64 {
    pv.split_once('=').unwrap().1.parse().unwrap()
}

fn parse_part(p: &str) -> Part {
    let mut iter = p[1..p.len() - 1].split(",");
    (
        parse_pval(iter.next().unwrap()),
        parse_pval(iter.next().unwrap()),
        parse_pval(iter.next().unwrap()),
        parse_pval(iter.next().unwrap()),
    )
}

fn parse_parts(ps: &str) -> Parts {
    ps.lines().map(parse_part).collect()
}

fn rpiece_accepts(rpiece: &(Fld, Cmp, u64), p: &Part) -> bool {
    let v = match rpiece.0 {
        Fld::X => p.0,
        Fld::M => p.1,
        Fld::A => p.2,
        Fld::S => p.3,
    };
    match rpiece.1 {
        Cmp::Gt => v > rpiece.2,
        Cmp::Lt => v < rpiece.2,
    }
}

fn accepts(rules: &Rules, p: &Part) -> bool {
    let mut cur = ("in", 0);
    loop {
        let cr = rules.get(&cur).unwrap();
        let nxt: Option<&Dst> = if let Some(grp) = &cr.0 {
            if rpiece_accepts(grp, p) {
                Some(&cr.1)
            } else {
                None
            }
        } else {
            Some(&cr.1)
        };
        if let Some(jmp_nxt) = nxt {
            match jmp_nxt {
                Dst::A => return true,
                Dst::R => return false,
                Dst::Go(x) => {
                    cur = (x, 0);
                }
            }
        } else {
            cur.1 += 1;
        }
    }
}

fn combival(b: Bounds) -> u64 {
    b.iter().map(|r| r.1 - r.0 + 1).product()
}

fn xrange(r: &Range, cmp: &Cmp, pvt: u64) -> (Range, Range) {
    match cmp {
        Cmp::Gt => ((max(r.0, pvt + 1), r.1), (r.0, min(r.1, pvt))),
        Cmp::Lt => ((r.0, min(r.1, pvt - 1)), (max(r.0, pvt), r.1)),
    }
}

fn rstrct((r1, r2): (Range, Range)) -> (Option<Range>, Option<Range>) {
    (
        if r1.0 > r1.1 { None } else { Some(r1) },
        if r2.0 > r2.1 { None } else { Some(r2) },
    )
}

fn substi(
    (r1, r2): (Option<Range>, Option<Range>),
    idx: usize,
    b: &Bounds,
) -> (Option<Bounds>, Option<Bounds>) {
    (
        r1.map(|r| {
            let mut bc = b.clone();
            bc[idx] = r;
            bc
        }),
        r2.map(|r| {
            let mut bc = b.clone();
            bc[idx] = r;
            bc
        }),
    )
}

fn splitb((fld, cmp, pvt): &(Fld, Cmp, u64), b: Bounds) -> (Option<Bounds>, Option<Bounds>) {
    let idx = match fld {
        Fld::X => 0,
        Fld::M => 1,
        Fld::A => 2,
        Fld::S => 3,
    };
    substi(rstrct(xrange(&b[idx], cmp, *pvt)), idx, &b)
}

fn ncombis(rules: &Rules, d: &Dst, b: Bounds) -> u64 {
    match d {
        Dst::A => return combival(b),
        Dst::R => 0,
        Dst::Go(x) => combis(rules, (x, 0), b),
    }
}

fn combis(rules: &Rules, cur: (&str, usize), b: Bounds) -> u64 {
    let cr = rules.get(&cur).unwrap();
    if let Some(grp) = &cr.0 {
        let (btru, bfal) = splitb(grp, b);
        btru.map(|bx| ncombis(rules, &cr.1, bx)).unwrap_or(0)
            + bfal
                .map(|bx| combis(rules, (cur.0, cur.1 + 1), bx))
                .unwrap_or(0)
    } else {
        ncombis(rules, &cr.1, b)
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
        let (rs, ps) = input.split_once("\n\n").unwrap();
        let rules = parse_rules(rs);
        parse_parts(ps)
            .iter()
            .filter(|p| accepts(&rules, p))
            .map(|p| p.0 + p.1 + p.2 + p.3)
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let (rs, _) = input.split_once("\n\n").unwrap();
        let rules = parse_rules(rs);
        combis(&rules, ("in", 0), [(1, 4000); 4])
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 19114);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 167409079868000);
    }
}
