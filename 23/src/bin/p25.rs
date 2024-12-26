use aoc23::problem::*;
use petgraph::algo::ford_fulkerson;
use petgraph::graph::Graph;
use std::collections::{HashMap, HashSet};

struct Uf {
    repr: Vec<usize>,
}

impl Uf {
    fn new(s: usize) -> Self {
        Uf {
            repr: (0..s).collect(),
        }
    }

    fn count(&mut self) -> HashMap<usize, usize> {
        let mut ret = HashMap::new();
        for i in 0..self.repr.len() {
            let r = self.find(i);
            if let Some(v) = ret.get(&r) {
                ret.insert(r, v + 1);
            } else {
                ret.insert(r, 1);
            }
        }
        ret
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.repr[rb] = ra;
    }
    fn find(&mut self, a: usize) -> usize {
        if self.repr[a] == a {
            a
        } else {
            self.repr[a] = self.find(self.repr[a]);
            self.repr[a]
        }
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
        let mut pg = Graph::<&str, usize>::new();
        let mut nam = HashMap::<&str, _>::new();
        for l in input.lines() {
            let (s, ts) = l.split_once(':').unwrap();
            if !nam.contains_key(s) {
                nam.insert(s, pg.add_node(s));
            }
            for t in ts.split(' ').filter(|s| !s.is_empty()) {
                if !nam.contains_key(t) {
                    nam.insert(t, pg.add_node(t));
                }
                pg.add_edge(nam[s], nam[t], 1);
                pg.add_edge(nam[t], nam[s], 1);
            }
        }
        // dbg!(&pg);
        let mut nis = pg.node_indices();
        let s = nis.next().unwrap();
        let mut count3 = 0;
        let mut countnot3 = 0;
        for t in nis {
            let (w, _) = ford_fulkerson(&pg, s, t);
            match w {
                3 => count3 += 1,
                _ => countnot3 += 1,
            }
        }
        count3 * (countnot3 + 1)
    }
    fn solve_2(&self, input: &str) -> u64 {
        (input.len() - input.len()) as u64
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 54);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 0);
    }
}
