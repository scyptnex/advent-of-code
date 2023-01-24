use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

use super::StructuredProblem;

struct Rec {
    node_c: usize,
    pressure_stride: usize,
    t_max: usize,
    res: Vec<Option<usize>>,
}

impl Rec {
    fn new(p: &Pressure, t_max: usize) -> Self {
        let node_c = p.node_idx.len();
        let pressure_stride = 1 << p.pressure.len();
        Rec {
            node_c,
            pressure_stride,
            t_max,
            res: vec![None; node_c * pressure_stride * t_max],
        }
    }
    fn new_e(p: &Pressure, t_max: usize) -> Self {
        let node_c = p.node_idx.len();
        let pressure_stride = 1 << p.pressure.len();
        dbg!(node_c * node_c * pressure_stride * t_max);
        Rec {
            node_c,
            pressure_stride,
            t_max,
            res: vec![None; node_c * node_c * pressure_stride * t_max],
        }
    }

    fn memo(&mut self, n: usize, r: usize, t: usize, released: usize) -> usize {
        let idx = self.idx(n, r, t);
        self.res[idx] = Some(released);
        released
    }

    fn get(&self, n: usize, r: usize, t: usize) -> Option<usize> {
        self.res[self.idx(n, r, t)]
    }

    fn idx(&self, n: usize, r: usize, t: usize) -> usize {
        n * (self.pressure_stride * self.t_max) + r * self.t_max + t
    }

    fn memo_e(&mut self, n: usize, e: usize, r: usize, t: usize, released: usize) -> usize {
        let idx = self.idx_e(n, e, r, t);
        self.res[idx] = Some(released);
        released
    }

    fn get_e(&self, n: usize, e: usize, r: usize, t: usize) -> Option<usize> {
        self.res[self.idx_e(n, e, r, t)]
    }

    fn idx_e(&self, n: usize, e: usize, r: usize, t: usize) -> usize {
        n * (self.node_c * self.pressure_stride * self.t_max)
            + e * (self.pressure_stride * self.t_max)
            + r * self.t_max
            + t
    }
}

fn pressurize(pidx: usize, cur: usize) -> usize {
    cur | (1 << pidx)
}

fn is_pressurized(pidx: usize, cur: usize) -> bool {
    (cur & (1 << pidx)) != 0
}

#[derive(Default)]
pub struct Pressure {
    node_idx: HashMap<String, usize>,
    adjacents: Vec<Vec<usize>>,
    pressure_idx: Vec<Option<usize>>,
    pressure: Vec<usize>,
}

impl Pressure {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        let re =
            regex::Regex::new(r"Valve ([^ ]*) has flow rate=([0-9]*);.*valves* (.*)$").unwrap();
        for l in i {
            let cap = re.captures(&l).unwrap();
            self.ingest(
                cap[1].as_ref(),
                cap[2].parse().unwrap(),
                cap[3].split(", ").collect(),
            )
        }
    }

    fn ingest(&mut self, node: &str, pressure: usize, adj: Vec<&str>) {
        let idx = self.make_node(node);
        let adj = adj.iter().map(|s| self.make_node(s)).collect();
        self.adjacents[idx] = adj;

        if pressure != 0 {
            self.pressure_idx[idx] = Some(self.pressure.len());
            self.pressure.push(pressure);
        }
    }

    fn make_node(&mut self, node: &str) -> usize {
        if !self.node_idx.contains_key(node) {
            self.node_idx.insert(node.to_string(), self.node_idx.len());
            self.adjacents.push(Vec::new());
            self.pressure_idx.push(None);
        }
        self.node_idx[node]
    }

    fn find_best(&self, max_t: usize) -> usize {
        let mut r = Rec::new(self, max_t);
        self.best(&mut r, self.node_idx["AA"], 0, max_t)
    }

    fn best(&self, r: &mut Rec, cur_idx: usize, released: usize, t_remaining: usize) -> usize {
        if t_remaining == 0 {
            return 0;
        }
        if let Some(x) = r.get(cur_idx, released, t_remaining) {
            return x;
        }
        let mut options: Vec<usize> = Vec::new();
        for adj in self.adjacents[cur_idx].iter() {
            options.push(self.best(r, *adj, released, t_remaining - 1));
        }
        if let Some(pidx) = self.pressure_idx[cur_idx] {
            if !is_pressurized(pidx, released) {
                let will_release = (t_remaining - 1) * self.pressure[pidx];
                let next_move = self.best(r, cur_idx, pressurize(pidx, released), t_remaining - 1);
                options.push(will_release + next_move);
            }
        }
        r.memo(
            cur_idx,
            released,
            t_remaining,
            *options.iter().max().unwrap(),
        )
    }

    fn find_best_e(&self, max_t: usize) -> usize {
        let mut r = Rec::new_e(self, max_t);
        self.best_e(&mut r, self.node_idx["AA"], self.node_idx["AA"], 0, max_t)
    }

    fn best_e(
        &self,
        r: &mut Rec,
        cur_idx: usize,
        e_idx: usize,
        released: usize,
        t_remaining: usize,
    ) -> usize {
        if t_remaining == 0 {
            return 0;
        }
        if let Some(x) = r.get_e(cur_idx, e_idx, released, t_remaining) {
            return x;
        }
        let mut options: Vec<usize> = Vec::new();
        // i move
        for adj in self.adjacents[cur_idx].iter() {
            for e_adj in self.adjacents[e_idx].iter() {
                options.push(self.best_e(r, *adj, *e_adj, released, t_remaining - 1));
            }
            if let Some(epidx) = self.pressure_idx[e_idx] {
                if !is_pressurized(epidx, released) {
                    let will_release = (t_remaining - 1) * self.pressure[epidx];
                    let next_move =
                        self.best_e(r, *adj, e_idx, pressurize(epidx, released), t_remaining - 1);
                    options.push(will_release + next_move);
                }
            }
        }
        // i open
        if let Some(pidx) = self.pressure_idx[cur_idx] {
            if !is_pressurized(pidx, released) {
                let will_release = (t_remaining - 1) * self.pressure[pidx];
                let new_pressurized = pressurize(pidx, released);
                for e_adj in self.adjacents[e_idx].iter() {
                    options.push(
                        self.best_e(r, cur_idx, *e_adj, new_pressurized, t_remaining - 1)
                            + will_release,
                    );
                }
                // both i and elephant open
                if let Some(epidx) = self.pressure_idx[e_idx] {
                    if epidx != pidx && !is_pressurized(epidx, new_pressurized) {
                        let e_will_release = (t_remaining - 1) * self.pressure[epidx];
                        let next_move = self.best_e(
                            r,
                            cur_idx,
                            e_idx,
                            pressurize(epidx, new_pressurized),
                            t_remaining - 1,
                        );
                        options.push(will_release + next_move + e_will_release);
                    }
                }
            }
        }
        r.memo_e(
            cur_idx,
            e_idx,
            released,
            t_remaining,
            *options.iter().max().unwrap(),
        )
    }
}

impl StructuredProblem for Pressure {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.find_best(30))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.find_best_e(26))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Pressure {
        let mut t = Pressure::default();
        t.read(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_problem_1() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "1651");
    }

    #[test]
    fn test_problem_2() {
        let t = data();
        assert_eq!(format!("{}", t.solve_2()), "1707");
    }

    #[test]
    fn test_parse() {
        let t = data();
        let aa = t.node_idx.get("AA");
        assert!(aa.is_some());
        assert_eq!(t.adjacents[*aa.unwrap()].len(), 3);
        assert_eq!(t.pressure_idx[*aa.unwrap()], None);

        assert_ne!(t.pressure_idx[t.node_idx["HH"]], None);
        assert_eq!(t.pressure[t.pressure_idx[t.node_idx["HH"]].unwrap()], 22);
    }

    #[test]
    fn test_rec() {
        let t = data();
        let mut r = Rec::new(&t, 30);
        assert_eq!(r.t_max, 30);
        assert_eq!(r.node_c, 10);
        assert_eq!(r.pressure_stride, 64);
        assert_eq!(r.res.len(), 30 * 10 * 64);

        assert_eq!(r.get(0, 0, 0), None);
        r.memo(0, 0, 0, 1);
        assert_eq!(r.get(0, 0, 0), Some(1));
    }

    #[test]
    fn test_pressurize() {
        assert_eq!(pressurize(0, 0), 1);
        assert_eq!(pressurize(8, 0), 0b1_0000_0000);
        assert_eq!(pressurize(4, 0b1111111), 0b1111111);

        assert!(is_pressurized(0, 0b101));
        assert!(!is_pressurized(1, 0b101));
        assert!(is_pressurized(2, 0b101));
        assert!(!is_pressurized(3, 0b101));
    }
}
