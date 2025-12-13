use aoc25::*;
use std::collections::{HashMap, HashSet};

type Preds<'a> = HashMap<&'a str, HashSet<&'a str>>;
type Paths<'a> = HashMap<&'a str, HashMap<u64, u64>>;

fn prd(input: &str) -> Preds {
    let mut preds = Preds::new();
    for l in input.lines() {
        let (pred, succs) = l.split_once(": ").unwrap();
        for s in succs.split(" ") {
            if !preds.contains_key(s) {
                preds.insert(s, HashSet::new());
            }
            preds.get_mut(s).unwrap().insert(pred);
        }
    }
    preds
}

fn pth<'a>(orig: &'a str, preds: &'a Preds) -> Paths<'a> {
    let mut paths = Paths::new();
    paths.insert(orig, HashMap::new());
    paths.get_mut(orig).unwrap().insert(0, 1);
    let mut todos = HashSet::<&str>::new();
    todos.insert(orig);
    for i in 0.. {
        if todos.is_empty() {
            break;
        }
        let mut new_todos = HashSet::<&str>::new();
        for t in todos {
            let fromw: u64 = *paths.get(t).unwrap().get(&i).unwrap();
            for p in preds.get(t).unwrap_or(&HashSet::<&str>::new()).iter() {
                new_todos.insert(p);
                if !paths.contains_key(p) {
                    paths.insert(p, HashMap::new());
                }
                let pmap = paths.get_mut(p).unwrap();
                if !pmap.contains_key(&(i + 1)) {
                    pmap.insert(i + 1, 0);
                }
                let newv: u64 = fromw + pmap.get(&(i + 1)).unwrap();
                pmap.insert(i + 1, newv);
            }
        }
        todos = new_todos;
    }
    paths
}

fn solve_1(input: &str) -> u64 {
    let preds = prd(input);
    let paths = pth("out", &preds);
    paths.get("you").unwrap().iter().map(|e| *e.1).sum()
}
fn solve_2(input: &str) -> u64 {
    let preds = prd(input);
    let from_out = pth("out", &preds);
    let from_dac = pth("dac", &preds);
    let from_fft = pth("fft", &preds);
    // svr -> fft -> dac -> out
    let dac_out = from_out
        .get("dac")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    let fft_dac = from_dac
        .get("fft")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    let srv_fft = from_fft
        .get("svr")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    dbg!(srv_fft, fft_dac, dac_out);
    // svr -> dac -> fft -> out
    let fft_out = from_out
        .get("fft")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    let dac_fft = from_fft
        .get("dac")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    let srv_dac = from_dac
        .get("svr")
        .map(|p| p.iter().map(|e| *e.1).sum())
        .unwrap_or(0);
    dbg!(srv_dac, dac_fft, fft_out);
    (srv_fft * fft_dac * dac_out) + (srv_dac * dac_fft * fft_out)
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(
                "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
            ),
            2
        );
    }
}
