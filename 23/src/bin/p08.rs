use aoc23::problem::*;
use num::Integer;

fn to_network(line: &str) -> (&str, [&str; 2]) {
    let (s, o) = line.split_once(" = ").unwrap();
    let (o1, o2) = o[1..o.len() - 1].split_once(", ").unwrap();
    (s, [o1, o2])
}

fn get_run(
    start: &str,
    net: &std::collections::HashMap<&str, [&str; 2]>,
    seq: &Vec<usize>,
) -> Vec<usize> {
    let mut seen = std::collections::HashSet::new();
    let mut cur = start;
    let mut known = Vec::new();
    for steps in 0.. {
        if !seen.insert((steps % seq.len(), cur)) {
            dbg!(known.len());
            return known;
        }
        cur = net.get(cur).unwrap()[seq[steps % seq.len()]];
        if cur.ends_with("Z") {
            known.push(steps + 1);
        }
    }
    panic!()
}

fn lcmx(idx: usize, runs: &Vec<Vec<usize>>) -> u64 {
    let mut lcm = 1;
    let mut ridx = idx;
    dbg!(idx);
    for r in runs {
        let cidx = ridx % r.len();
        ridx = ridx / r.len();
        lcm = lcm.lcm(&r[cidx]);
    }
    lcm as u64
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let (si, li) = input.split_once("\n\n").unwrap();
        let seq: Vec<usize> = si
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            })
            .collect();
        let network: std::collections::HashMap<&str, [&str; 2]> =
            li.lines().map(to_network).collect();
        let mut cur = "AAA";
        let mut steps = 0;
        while cur != "ZZZ" {
            cur = network.get(cur).unwrap()[seq[steps % seq.len()]];
            steps += 1;
        }
        steps as u64
    }
    fn solve_2(&self, input: &str) -> u64 {
        let (si, li) = input.split_once("\n\n").unwrap();
        let seq: Vec<usize> = si
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            })
            .collect();
        let network: std::collections::HashMap<&str, [&str; 2]> =
            li.lines().map(to_network).collect();
        let locations = network
            .iter()
            .map(|t| *t.0)
            .filter(|s| s.ends_with("A"))
            .collect::<Vec<_>>();
        let runs = locations
            .iter()
            .map(|l| get_run(l, &network, &seq))
            .collect::<Vec<_>>();
        let mrl = runs.iter().map(|r| r.len()).product();
        (0..mrl).map(|idx| lcmx(idx, &runs)).min().unwrap()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Prob::new().solve_1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
        assert_eq!(
            Prob::new().solve_1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Prob::new().solve_2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
