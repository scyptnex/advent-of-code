use aoc23::problem::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    Hi,
    Lo,
}

impl Pulse {
    fn invert(&self) -> Pulse {
        match self {
            Pulse::Hi => Pulse::Lo,
            Pulse::Lo => Pulse::Hi,
        }
    }

    fn is_high(&self) -> bool {
        match self {
            Pulse::Hi => true,
            _ => false,
        }
    }
}

type NodeState = HashMap<Id, Pulse>;

type Id = usize;

#[derive(Debug, Copy, Clone)]
enum Nod {
    Brod,
    Flip,
    Conj,
}

impl Nod {
    fn handle(&self, orig: Id, pul: Pulse, ns: &mut NodeState) -> Option<Pulse> {
        match self {
            Nod::Brod => Some(pul),
            Nod::Flip => match pul {
                Pulse::Hi => None,
                Pulse::Lo => {
                    ns.insert(0, ns[&0].invert());
                    Some(ns[&0])
                }
            },
            Nod::Conj => {
                ns.insert(orig, pul);
                if ns.iter().map(|(_, v)| v).all(Pulse::is_high) {
                    Some(Pulse::Lo)
                } else {
                    Some(Pulse::Hi)
                }
            }
        }
    }
}

type Signal = (Id, Pulse, Id);

#[derive(Default, Debug)]
struct Nw {
    broad: Id,
    layout: Vec<(Nod, Vec<Id>)>,
    state: Vec<NodeState>,
}

impl Nw {
    fn button(&mut self) -> (u64, u64) {
        let mut lows = 0;
        let mut highs = 0;
        let mut pulses = Vec::new();
        let mut pos = 0;
        pulses.push((self.broad, Pulse::Lo, self.broad));
        while pos < pulses.len() {
            let (orig, cur_pulse, nxt) = pulses[pos];
            pos += 1;
            match cur_pulse {
                Pulse::Hi => highs += 1,
                Pulse::Lo => lows += 1,
            };
            // Drop signals that go to nothing.
            if nxt >= self.layout.len() {
                continue;
            }
            let maybe_nxt_sig = self.layout[nxt]
                .0
                .handle(orig, cur_pulse, &mut self.state[nxt]);
            if let Some(nxt_pulse) = maybe_nxt_sig {
                for dst in &self.layout[nxt].1 {
                    pulses.push((nxt, nxt_pulse, *dst));
                }
            }
        }
        (lows, highs)
    }
    fn button2(&mut self) -> bool {
        let mut pulses = Vec::new();
        let mut pos = 0;
        pulses.push((self.broad, Pulse::Lo, self.broad));
        while pos < pulses.len() {
            let (orig, cur_pulse, nxt) = pulses[pos];
            pos += 1;
            // Drop signals that go to nothing.
            if nxt >= self.layout.len() {
                if cur_pulse == Pulse::Lo {
                    return true;
                } else {
                    continue;
                }
            }
            let maybe_nxt_sig = self.layout[nxt]
                .0
                .handle(orig, cur_pulse, &mut self.state[nxt]);
            if let Some(nxt_pulse) = maybe_nxt_sig {
                for dst in &self.layout[nxt].1 {
                    pulses.push((nxt, nxt_pulse, *dst));
                }
            }
        }
        false
    }
}

fn parse_nod(s: &str) -> (Nod, &str) {
    if s == "broadcaster" {
        return (Nod::Brod, s);
    }
    let (pre, post) = s.split_at(1);
    if pre == "%" {
        return (Nod::Flip, post);
    } else {
        return (Nod::Conj, post);
    }
}

fn parse(s: &str) -> Nw {
    let pieces = s
        .lines()
        .map(|ln| {
            let (node, rest) = ln.split_once(" -> ").unwrap();
            let rest: Vec<String> = rest.split(',').map(str::trim).map(str::to_owned).collect();
            let (nod_ty, name) = parse_nod(node);
            (name.to_owned(), nod_ty, rest)
        })
        .collect::<Vec<(String, Nod, Vec<String>)>>();
    let name_map = pieces
        .iter()
        .enumerate()
        .map(|(i, (n, _, _))| (n.clone(), i))
        .collect::<HashMap<String, Id>>();
    let layout: Vec<(Nod, Vec<Id>)> = pieces
        .iter()
        .map(|p| {
            (
                p.1,
                p.2.iter()
                    .map(|s| *name_map.get(s).unwrap_or(&pieces.len()))
                    .collect(),
            )
        })
        .collect();
    let mut inverse: Vec<Vec<Id>> = vec![Vec::new(); layout.len()];
    for src in 0..layout.len() {
        for dst in &layout[src].1 {
            if dst >= &layout.len() {
                continue;
            }
            inverse[*dst].push(src);
        }
    }
    let state = layout
        .iter()
        .enumerate()
        .map(|(src, (nod, _))| match nod {
            Nod::Brod => NodeState::new(),
            Nod::Flip => [(0_usize, Pulse::Lo)].into_iter().collect(),
            Nod::Conj => inverse[src].iter().map(|iv| (*iv, Pulse::Lo)).collect(),
        })
        .collect();
    Nw {
        broad: name_map["broadcaster"],
        layout,
        state,
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
        let mut s = parse(input);
        let mut tl = 0;
        let mut th = 0;
        for _ in 0..1000 {
            let (l, h) = s.button();
            tl += l;
            th += h;
        }
        tl * th
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut s = parse(input);
        for i in 0.. {
            if s.button2() {
                return i + 1;
            }
        }
        panic!();
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 32000000);
    }

    #[test]
    fn test_parse() {}
}
