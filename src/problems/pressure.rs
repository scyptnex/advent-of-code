use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

use super::StructuredProblem;

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
    p_adj: Vec<Vec<usize>>,
    opened_cfgs: usize,
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
        for (i, p) in self
            .pressure_idx
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_some())
            .map(|(i, p)| (i, p.unwrap()))
        {
            self.p_adj[p] = self.get_distances(i);
        }
    }

    fn get_distances(&self, idx: usize) -> Vec<usize> {
        let mut d = 0;
        let mut frontier: Vec<usize> = Vec::from([idx]);
        let mut visited: Vec<bool> = vec![false; self.node_idx.len()];
        let mut distances: Vec<usize> = vec![0; self.node_idx.len()];
        visited[idx] = true;
        loop {
            for f in frontier.iter() {
                distances[*f] = d;
            }
            let new_frontier: HashSet<usize> = frontier
                .iter()
                .flat_map(|f| self.adjacents[*f].iter())
                .filter(|nf| !visited[**nf])
                .copied()
                .collect();
            if new_frontier.is_empty() {
                break;
            }
            frontier = Vec::from_iter(new_frontier.into_iter());
            for f in frontier.iter() {
                visited[*f] = true;
            }
            d += 1;
        }
        (0..self.pressure.len())
            .map(|pidx| {
                self.pressure_idx
                    .iter()
                    .enumerate()
                    .filter(|(_, px)| px.map(|p| p == pidx).unwrap_or(false))
                    .next()
                    .unwrap()
            })
            .map(|(i, _)| distances[i])
            .collect()
    }

    fn ingest(&mut self, node: &str, pressure: usize, adj: Vec<&str>) {
        let idx = self.make_node(node);
        let adj = adj.iter().map(|s| self.make_node(s)).collect();
        self.adjacents[idx] = adj;

        if pressure != 0 || node == "AA" {
            self.pressure_idx[idx] = Some(self.pressure.len());
            self.pressure.push(pressure);
            self.p_adj.push(Vec::new());
            self.opened_cfgs = 1 << self.pressure.len();
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

    fn find_best(&self) -> usize {
        let start_idx = self.pressure_idx[self.node_idx["AA"]].unwrap();
        self.best(start_idx, 30, 0)
    }

    fn best(&self, idx: usize, t_left: usize, opened: usize) -> usize {
        //dbg!(idx, t_left, opened);
        if opened == self.opened_cfgs - 1 {
            return 0;
        }
        let mut mx = 0;
        for nxt in 0..self.pressure.len() {
            if is_pressurized(nxt, opened) {
                continue;
            }
            let t_req = self.p_adj[idx][nxt] + 1;
            if t_req > t_left {
                continue;
            }
            let new_t = t_left - t_req;
            let val = new_t * self.pressure[nxt];
            let tot = val + self.best(nxt, new_t, pressurize(nxt, opened));
            mx = mx.max(tot);
        }
        mx
    }

    fn find_best_e(&self) -> usize {
        let mut mx = 0;
        let start_idx = self.pressure_idx[self.node_idx["AA"]].unwrap();
        for h_opens in 0..self.opened_cfgs {
            if h_opens % 1000 == 0 {
                println!("{}/{}", h_opens, self.opened_cfgs);
            }
            let e_opens = (!h_opens) & (self.opened_cfgs - 1);
            let val = self.best(start_idx, 26, h_opens) + self.best(start_idx, 26, e_opens);
            mx = mx.max(val);
        }
        mx
    }
}

impl StructuredProblem for Pressure {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(self.find_best())
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(self.find_best_e())
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
    fn test_p_adj() {
        let t = data();
        let exp: Vec<Vec<usize>> = Vec::new();
        for i in 0..exp.len() {
            assert_eq!(exp[i][i], 0);
        }
    }

    #[test]
    fn test_parse() {
        let t = data();
        let aa = t.node_idx.get("AA");
        assert!(aa.is_some());
        assert_eq!(t.adjacents[*aa.unwrap()].len(), 3);

        assert_eq!(t.pressure_idx[t.node_idx["GG"]], None);
        assert_ne!(t.pressure_idx[t.node_idx["HH"]], None);
        assert_eq!(t.pressure[t.pressure_idx[t.node_idx["HH"]].unwrap()], 22);

        assert_eq!(t.opened_cfgs, 128); //  6 valves with pressure and 1 AA
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
