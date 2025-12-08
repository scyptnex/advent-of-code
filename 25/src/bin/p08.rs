use aoc25::*;

struct Ds {
    repr: Vec<usize>,
}

impl Ds {
    fn new(len: usize) -> Self {
        Self {
            repr: (0..len).collect(),
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.repr[v] == v {
            return v;
        }
        let r = self.find(self.repr[v]);
        self.repr[v] = r;
        r
    }
    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.repr[ra] = rb;
    }

    fn spans(&mut self) -> bool {
        if self.repr.is_empty() {
            return true;
        }
        let rr = self.find(0);
        for i in 1..self.repr.len() {
            if self.find(i) != rr {
                return false;
            }
        }
        true
    }
}

fn solve_1x(input: &str, conns: usize) -> u64 {
    let coords = input
        .lines()
        .flat_map(|l| l.split_once(','))
        .map(|(x, yz)| (x, yz.split_once(',').unwrap()))
        .map(|(x, (y, z))| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
        .collect::<Vec<(i64, i64, i64)>>();
    let mut kgraph = Vec::new();
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let xd: i64 = coords[i].0 - coords[j].0;
            let yd: i64 = coords[i].1 - coords[j].1;
            let zd: i64 = coords[i].2 - coords[j].2;
            let d: i64 = xd * xd + yd * yd + zd * zd;
            let d = d as f64;
            let d = d.sqrt();
            kgraph.push((d, i, j));
        }
    }
    kgraph.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut ds = Ds::new(coords.len());
    for i in 0..conns {
        let kc = kgraph[i];
        ds.union(kc.1, kc.2);
    }

    let mut hist = std::collections::HashMap::<usize, u64>::new();
    for r in (0..coords.len()).map(|i| ds.find(i)) {
        if !hist.contains_key(&r) {
            hist.insert(r, 1);
        } else {
            hist.insert(r, hist[&r] + 1);
        }
    }
    let mut count = hist.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    count.sort();
    count.reverse();
    count[0] * count[1] * count[2]
}
fn solve_1(input: &str) -> u64 {
    solve_1x(input, 1000)
}
fn solve_2(input: &str) -> i64 {
    let coords = input
        .lines()
        .flat_map(|l| l.split_once(','))
        .map(|(x, yz)| (x, yz.split_once(',').unwrap()))
        .map(|(x, (y, z))| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
        .collect::<Vec<(i64, i64, i64)>>();
    let mut kgraph = Vec::new();
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let xd: i64 = coords[i].0 - coords[j].0;
            let yd: i64 = coords[i].1 - coords[j].1;
            let zd: i64 = coords[i].2 - coords[j].2;
            let d: i64 = xd * xd + yd * yd + zd * zd;
            let d = d as f64;
            let d = d.sqrt();
            kgraph.push((d, i, j));
        }
    }
    kgraph.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut ds = Ds::new(coords.len());
    for i in 0.. {
        let kc = kgraph[i];
        ds.union(kc.1, kc.2);
        if ds.spans() {
            return coords[kc.1].0 * coords[kc.2].0;
        }
    }
    0
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_1() {
        assert_eq!(solve_1x(TEST_INPUT, 10), 40);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 25272);
    }
}
