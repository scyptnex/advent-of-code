use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default, Debug, Copy, Clone)]
struct Rsrc {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Rsrc {
    fn new(n: usize, ty: &str) -> Self {
        match ty {
            "ore" => Rsrc {
                ore: n,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            "clay" => Rsrc {
                ore: 0,
                clay: n,
                obsidian: 0,
                geode: 0,
            },
            "obsidian" => Rsrc {
                ore: 0,
                clay: 0,
                obsidian: n,
                geode: 0,
            },
            "geode" => Rsrc {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: n,
            },
            _ => panic!(),
        }
    }

    fn parse(s: &str) -> Self {
        s.split_once(' ')
            .map(|(s1, s2)| Rsrc::new(s1.parse::<usize>().unwrap(), s2))
            .unwrap_or_else(|| Rsrc::new(1, s))
    }

    fn and(&self, other: &Rsrc) -> Self {
        Rsrc {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn less(&self, other: &Rsrc) -> Self {
        Rsrc {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }

    fn all_ge(&self, other: &Rsrc) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn tpl(&self) -> (usize, usize, usize, usize) {
        (self.ore, self.clay, self.obsidian, self.geode)
    }
}

struct Bot {
    cost: Rsrc,
    prod: Rsrc,
}

impl Bot {
    fn parse(s: &str) -> Self {
        let (pre, post) = s.split_once(" costs ").unwrap();
        Bot {
            prod: Rsrc::parse(pre.split(' ').skip(1).next().unwrap()),
            cost: post
                .split(" and ")
                .map(|s| Rsrc::parse(s))
                .inspect(|x| {
                    dbg!(x.tpl());
                })
                .reduce(|a, b| a.and(&b))
                .unwrap(),
        }
    }

    fn will_afford(&self, gather: &Rsrc) -> bool {
        (self.cost.ore == 0 || gather.ore > 0)
            && (self.cost.clay == 0 || gather.clay > 0)
            && (self.cost.obsidian == 0 || gather.obsidian > 0)
            && (self.cost.geode == 0 || gather.geode > 0)
    }

    fn can_afford(&self, wallet: &Rsrc) -> bool {
        wallet.all_ge(&self.cost)
    }
}

type Rslt = (Rsrc, Rsrc);

struct Bp {
    idx: usize,
    bot_specs: [Bot; 4],
}

impl Bp {
    fn parse(s: &str) -> Self {
        let (pre, post) = s.split_once(":").unwrap();
        let mut post_i = post.split('.').map(|s| s.trim()).filter(|s| !s.is_empty());
        Bp {
            idx: pre.split_once(' ').unwrap().1.parse().unwrap(),
            bot_specs: [
                Bot::parse(post_i.next().unwrap()),
                Bot::parse(post_i.next().unwrap()),
                Bot::parse(post_i.next().unwrap()),
                Bot::parse(post_i.next().unwrap()),
            ],
        }
    }

    fn most_geodes(&self, t_lim: usize) -> usize {
        let mut lim = Rsrc {
            ore: 1,
            clay: 1,
            obsidian: 1,
            geode: 1,
        };
        loop {
            let (w, c) =
                self.most_geodes_at(t_lim, &Rsrc::default(), &Rsrc::new(1, "ore"), &lim, None);
            if c.geode == lim.geode {
                lim.geode += 1;
            } else if c.obsidian == lim.obsidian {
                lim.obsidian += 1;
            } else if c.clay == lim.clay {
                lim.clay += 1;
            } else if c.ore == lim.ore {
                lim.ore += 1;
            } else {
                return w.geode;
            }
        }
    }

    fn most_geodes_at(
        &self,
        time_left: usize,
        wallet: &Rsrc,
        gather: &Rsrc,
        req: &Rsrc,
        target: Option<usize>,
    ) -> Rslt {
        if time_left == 0 {
            return (wallet.clone(), gather.clone());
        }
        if target.is_none() {
            let mut v: Vec<Rslt> = Vec::new();
            for (i, tb) in self.bot_specs.iter().enumerate() {
                if !tb.will_afford(&gather) {
                    continue;
                }
                if !req.all_ge(&gather.and(&tb.prod)) {
                    continue;
                }
                v.push(self.most_geodes_at(time_left, wallet, gather, req, Some(i)));
            }
            if v.is_empty() {
                return self.most_geodes_at(time_left - 1, wallet, gather, req, None);
            } else {
                return v
                    .into_iter()
                    .max_by_key(|(w1, g1)| (w1.geode, g1.geode, g1.obsidian, g1.clay, g1.ore))
                    .unwrap();
            }
        }
        let tb = &self.bot_specs[target.unwrap()];
        if tb.can_afford(wallet) {
            self.most_geodes_at(
                time_left - 1,
                &wallet.and(gather).less(&tb.cost),
                &gather.and(&tb.prod),
                req,
                None,
            )
        } else {
            self.most_geodes_at(time_left - 1, &wallet.and(gather), gather, req, target)
        }
    }
}

#[derive(Default)]
pub struct Robots {
    data: Vec<Bp>,
}

impl Robots {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.map(|s| Bp::parse(&s)).collect();
    }
}

impl StructuredProblem for Robots {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(
            self.data
                .iter()
                .map(|bp| bp.most_geodes(24) * bp.idx)
                .sum::<usize>(),
        )
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(
            self.data[..3]
                .iter()
                .map(|bp| bp.most_geodes(32))
                .product::<usize>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Robots {
        let mut t = Robots::default();
        t.read(
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_robots() {
        let _t = data();
        //assert_eq!(format!("{}", t.solve_1()), "33");
        //assert_eq!(t.data[0].most_geodes(32), 56);
        //assert_eq!(t.data[1].most_geodes(32), 62);
    }

    #[test]
    fn test_parse() {
        let r1 = Rsrc::parse("geode");
        assert_eq!(r1.tpl(), (0, 0, 0, 1));
        let r2 = Rsrc::parse("3 ore");
        assert_eq!(r2.tpl(), (3, 0, 0, 0));

        let b = Bot::parse("Each obsidian robot costs 3 ore and 8 clay");
        assert_eq!(b.prod.tpl(), (0, 0, 1, 0));
        assert_eq!(b.cost.tpl(), (3, 8, 0, 0));

        let bp = Bp::parse(
            "Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.",
        );
        assert_eq!(bp.idx, 2);
        assert_eq!(bp.bot_specs[1].prod.clay, 1);
        assert_eq!(bp.bot_specs[1].cost.tpl(), (3, 0, 0, 0));
    }
}
