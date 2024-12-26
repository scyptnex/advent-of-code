use aoc23::problem::*;

fn hashr(input: &str) -> u64 {
    let mut cur = 0u64;
    for b in input.as_bytes() {
        if *b == b'\n' {
            continue;
        }
        cur = ((cur + *b as u64) * 17u64) % 256u64;
    }
    cur
}

enum Rec {
    Rm(String),
    In(String, u64),
}

impl Rec {
    fn grab(input: &str) -> Self {
        dbg!(input);
        input
            .split_once('=')
            .map(|(a, b)| Rec::In(a.to_owned(), b.parse().unwrap()))
            .unwrap_or_else(|| Rec::Rm(input.split_once('-').unwrap().0.to_owned()))
    }
}

type LBox = Vec<(String, u64)>;

fn locate(bx: &LBox, s: &str) -> Option<usize> {
    (0..bx.len()).find(|i| bx[*i].0 == s)
}

type Cells = Vec<LBox>;

fn remover(cells: &mut Cells, s: String) {
    let idx = hashr(&s) as usize;
    if let Some(l) = locate(&cells[idx], &s) {
        cells[idx].remove(l);
    }
}
fn insertr(cells: &mut Cells, s: String, v: u64) {
    let idx = hashr(&s) as usize;
    if let Some(l) = locate(&cells[idx], &s) {
        cells[idx][l].1 = v;
    } else {
        cells[idx].push((s, v));
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
        input.split(",").map(hashr).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut cells = vec![LBox::new(); 256];
        for cmd in input
            .split('\n')
            .flat_map(|l| l.split(','))
            .filter(|s| !s.is_empty())
            .map(|i| Rec::grab(i))
        {
            match cmd {
                Rec::Rm(s) => remover(&mut cells, s),
                Rec::In(s, v) => insertr(&mut cells, s, v),
            }
        }
        cells
            .iter()
            .enumerate()
            .map(|(ci, cx)| {
                (ci + 1) as u64
                    * cx.iter()
                        .enumerate()
                        .map(|(li, lx)| (li + 1) as u64 * (lx.1))
                        .sum::<u64>()
            })
            .sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 145);
    }
}
