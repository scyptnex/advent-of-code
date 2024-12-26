use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Debug, Eq, PartialEq, Clone)]
enum MOp {
    Num(i64),
    Op(String, char, String),
}
use MOp::*;

impl MOp {
    fn parse(s: &str) -> Self {
        match s.split_once(' ') {
            None => Num(s.parse().unwrap()),
            Some((a, b)) => {
                let (c, d) = b.split_once(' ').unwrap();
                Op(a.to_string(), c.chars().next().unwrap(), d.to_string())
            }
        }
    }

    fn uses_on_left(&self, s: &str) -> Option<bool> {
        match self {
            Num(_) => None,
            Op(l, _, r) => {
                if l == s {
                    Some(true)
                } else if r == s {
                    Some(false)
                } else {
                    None
                }
            }
        }
    }
}

fn parse_line(s: String) -> (String, MOp) {
    let (pre, post) = s.split_once(": ").unwrap();
    (pre.to_string(), MOp::parse(post))
}

fn find_tbl(t: &mut HashMap<&str, (&MOp, Option<i64>)>, req: &str) -> i64 {
    if let Some(i) = t[req].1 {
        return i;
    }
    if let Op(l, c, r) = t[req].0 {
        let lv = find_tbl(t, l);
        let rv = find_tbl(t, r);
        let val = match c {
            '+' => lv + rv,
            '-' => lv - rv,
            '*' => lv * rv,
            '/' => lv / rv,
            _ => panic!(),
        };
        t.get_mut(req).unwrap().1 = Some(val);
        return val;
    }
    panic!()
}

fn flippymcgee(
    root_l: &str,
    root_r: &str,
    next_l: &str,
    next_c: &char,
    next_r: &str,
    h_between: String,
    h_after: String,
) -> (MOp, MOp) {
    let x_before = if *root_l == h_between { root_r } else { root_l }.to_string();
    let hp_is_left = *next_l == h_after;
    let x_after: String = if hp_is_left { next_r } else { next_l }.to_string();
    let new_root = Op(h_after, '+', h_between);
    match (next_c, hp_is_left) {
        ('+', false) => (new_root, Op(x_before, '-', x_after)),
        ('-', false) => (new_root, Op(x_after, '-', x_before)),
        ('*', false) => (new_root, Op(x_before, '/', x_after)),
        ('/', false) => (new_root, Op(x_after, '/', x_before)),
        ('+', true) => (new_root, Op(x_before, '-', x_after)),
        ('-', true) => (new_root, Op(x_before, '+', x_after)),
        ('*', true) => (new_root, Op(x_before, '/', x_after)),
        ('/', true) => (new_root, Op(x_before, '*', x_after)),
        _ => panic!(),
    }
}

fn flippedydip(root: &MOp, next: &MOp, h_between: String, h_after: String) -> (MOp, MOp) {
    if let Op(root_l, _, root_r) = root {
        if let Op(next_l, next_c, next_r) = next {
            return flippymcgee(root_l, root_r, next_l, next_c, next_r, h_between, h_after);
        }
    }
    panic!();
}

#[derive(Default, Clone)]
pub struct MMath {
    data: Vec<(String, MOp)>,
}

impl MMath {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.map(parse_line).collect();
    }

    fn flip(&self) -> Self {
        let mut h_path = self.human_path();
        h_path.pop();

        let mut rebuild: HashMap<String, MOp> = self.data.iter().cloned().collect();

        while h_path.len() > 1 {
            let h_next = h_path.pop().unwrap();
            let (new_root, new_other) = flippedydip(
                &rebuild["root"],
                &rebuild[h_next],
                h_next.to_string(),
                h_path.last().unwrap().to_string(),
            );
            *rebuild.get_mut("root").unwrap() = new_root;
            *rebuild.get_mut(h_next).unwrap() = new_other;
        }

        *rebuild.get_mut("humn").unwrap() = Num(0);

        MMath {
            data: rebuild.into_iter().collect(),
        }
    }

    fn human_path(&self) -> Vec<&str> {
        let mut p: Vec<&str> = vec!["humn"];
        while *p.last().unwrap() != "root" {
            let (idx, _) = self.find_user_idx(p.last().unwrap());
            p.push(self.data[idx].0.as_str());
        }
        p
    }

    fn find_user_idx(&self, used_name: &str) -> (usize, bool) {
        self.data
            .iter()
            .enumerate()
            .find_map(|(i, m)| m.1.uses_on_left(used_name).map(|b| (i, b)))
            .unwrap()
    }

    fn init_tbl(&self) -> HashMap<&str, (&MOp, Option<i64>)> {
        self.data
            .iter()
            .map(|(s, m)| {
                (
                    s.as_str(),
                    (
                        m,
                        match m {
                            Num(i) => Some(*i),
                            _ => None,
                        },
                    ),
                )
            })
            .collect()
    }

    fn find_root(&self) -> i64 {
        find_tbl(&mut self.init_tbl(), "root")
    }
}

impl StructuredProblem for MMath {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.find_root())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.flip().find_root())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> MMath {
        let mut t = MMath::default();
        t.read(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_mmath() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "152");
        assert_eq!(format!("{}", t.solve_2()), "301");
    }

    #[test]
    fn test_parse() {
        let t = data();
        assert_eq!(
            t.data[0],
            (
                String::from("root"),
                MOp::Op(String::from("pppw"), '+', String::from("sjmn"))
            )
        );
    }

    #[test]
    fn test_human() {
        let t = data();
        assert_eq!(
            t.human_path(),
            vec!["humn", "ptdq", "lgvd", "cczh", "pppw", "root"]
        );
    }
}
