use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

type Worry = i64;

fn op_val(s: &str, i: &Worry) -> Worry {
    if s == "old" {
        *i
    } else {
        s.parse::<Worry>().unwrap()
    }
}

fn op_op(s: &str, v1: &Worry, v2: &Worry) -> Worry {
    match s {
        "+" => v1 + v2,
        "*" => v1 * v2,
        _ => panic!("Unknown op {}", s),
    }
}

struct Mnk {
    items: Vec<Worry>,
    op: Vec<String>,
    div: i32,
    t_target: usize,
    f_target: usize,
    inspected: usize,
}

impl Mnk {
    fn parse(input: &[String]) -> Self {
        Mnk {
            items: input[1]
                .split_once(':')
                .unwrap()
                .1
                .split(',')
                .map(|s| s.trim().parse::<Worry>().unwrap())
                .collect(),
            op: input[2]
                .split_once('=')
                .unwrap()
                .1
                .trim()
                .split(' ')
                .map(|s| String::from(s))
                .rev()
                .collect(),
            div: input[3].split(' ').last().unwrap().parse().unwrap(),
            t_target: input[4].split(' ').last().unwrap().parse().unwrap(),
            f_target: input[5].split(' ').last().unwrap().parse().unwrap(),
            inspected: 0,
        }
    }

    fn do_op(&self, i: &Worry) -> Worry {
        let v1 = op_val(&self.op[0], i);
        let v2 = op_val(&self.op[2], i);
        op_op(&self.op[1], &v1, &v2)
    }

    fn business(&mut self, v: &dyn Fn(Worry) -> Worry) -> Vec<(Worry, usize)> {
        let r: Vec<(Worry, usize)> = self
            .items
            .iter()
            .map(|i| self.do_op(i))
            .map(v)
            .map(|i| {
                if i % (self.div as Worry) == 0 {
                    (i, self.t_target)
                } else {
                    (i, self.f_target)
                }
            })
            .collect();
        self.items = vec![];
        self.inspected += r.len();
        r
    }

    fn business_1(&mut self, _: &Worry) -> Vec<(Worry, usize)> {
        let cloj = |i: Worry| i / 3;
        self.business(&cloj)
    }

    fn business_2(&mut self, d: &Worry) -> Vec<(Worry, usize)> {
        let cloj = |i: Worry| i % d;
        self.business(&cloj)
    }
}

#[derive(Default)]
pub struct Monkey {
    data: Vec<String>,
}

impl Monkey {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.collect();
    }

    fn monkeys(&self) -> Vec<Mnk> {
        self.data.chunks(7).map(|c| Mnk::parse(c)).collect()
    }

    fn monkey_business(
        &self,
        rounds: usize,
        bsn: fn(&mut Mnk, &Worry) -> Vec<(Worry, usize)>,
    ) -> u64 {
        let mut monkeys = self.monkeys();
        let modulr: Worry = monkeys.iter().map(|m| m.div as Worry).product();
        for _ in 0..rounds {
            for m in 0..monkeys.len() {
                let business = bsn(&mut monkeys[m], &modulr);
                for (i, t) in business {
                    monkeys[t].items.push(i);
                }
            }
        }
        let mut mb: Vec<u64> = monkeys.iter().map(|m| m.inspected as u64).collect();
        mb.sort();
        mb.iter().rev().take(2).product()
    }
}

impl StructuredProblem for Monkey {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.monkey_business(20, Mnk::business_1))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.monkey_business(10000, Mnk::business_2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> Monkey {
        let mut t = Monkey::default();
        t.read(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_answer() {
        let t = get_data();
        assert_eq!(format!("{}", t.solve_1()), "10605");
        assert_eq!(format!("{}", t.solve_2()), "2713310158");
    }

    #[test]
    fn test_business() {
        let t = get_data();
        assert_eq!(
            t.monkeys()[0].business_1(&(0 as Worry)),
            vec![(500, 3), (620, 3)]
        );
    }

    #[test]
    fn test_parse() {
        let t = get_data();

        let ms = t.monkeys();
        assert_eq!(ms.len(), 4);

        let m = &ms[2];
        assert_eq!(m.items, vec![79, 60, 97]);
        assert_eq!(m.op, vec!["old", "*", "old"]);
        assert_eq!(m.div, 13);
        assert_eq!(m.t_target, 1);
        assert_eq!(m.f_target, 3);
    }
}
