use aoc23::problem::*;
use num::BigInt;
use num::BigRational;

type Xyz = (i64, i64, i64);
type Hail = (Xyz, Xyz);
type Data = Vec<Hail>;
type Rat = BigRational;

fn to_xyz(s: &str) -> Xyz {
    let inty = |s: &str| s.trim().parse().unwrap();
    let (x, yz) = s.split_once(',').unwrap();
    let (y, z) = yz.split_once(',').unwrap();
    (inty(x), inty(y), inty(z))
}

fn solve_simult(a: &Rat, b: &Rat, c: &Rat, d: &Rat, p: &Rat, q: &Rat) -> Option<(Rat, Rat)> {
    //    1    | d   -b | | p |
    // ------- |        | |   |
    // ad - bc | -c   a | | q |
    let denom = a * d - b * c;
    if denom == Rat::from(BigInt::from(0)) {
        return None;
    }
    Some(((p * d - b * q) / &denom, (q * a - p * c) / &denom))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Prob(Rat, Rat);

impl Prob {
    fn new(least: i64, most: i64) -> Self {
        Prob(
            Rat::from(BigInt::from(least)),
            Rat::from(BigInt::from(most)),
        )
    }

    fn isect(&self, a: &Hail, b: &Hail) -> bool {
        let zero = Rat::from(BigInt::from(0));
        let pxa = Rat::from(BigInt::from(a.0 .0));
        let pya = Rat::from(BigInt::from(a.0 .1));
        let pxb = Rat::from(BigInt::from(b.0 .0));
        let pyb = Rat::from(BigInt::from(b.0 .1));
        let vxa = Rat::from(BigInt::from(a.1 .0));
        let vya = Rat::from(BigInt::from(a.1 .1));
        let vxb = Rat::from(BigInt::from(b.1 .0));
        let vyb = Rat::from(BigInt::from(b.1 .1));
        let soln = solve_simult(&vxa, &-vxb, &vya, &-vyb, &(pxb - &pxa), &(pyb - &pya));
        if let Some((ta, tb)) = soln {
            let xi = &ta * vxa + &pxa;
            let yi = &ta * vya + &pya;
            //println!("{}, {} : {} {}", ta, tb, xi, yi);
            if &ta < &zero || &tb < &zero {
                false
            } else {
                xi >= self.0 && xi <= self.1 && yi >= self.0 && yi <= self.1
            }
        } else {
            false
        }
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let d: Data = input
            .lines()
            .map(|l| l.split_once('@').unwrap())
            .map(|(p, v)| (to_xyz(p), to_xyz(v)))
            .collect();
        let mut count = 0;
        for a in 0..d.len() {
            for b in a..d.len() {
                if self.isect(&d[a], &d[b]) {
                    count += 1;
                }
            }
        }
        count
    }
    fn solve_2(&self, input: &str) -> u64 {
        (input.len() - input.len()) as u64
    }
}

fn main() {
    solve(Prob::new(200000000000000, 400000000000000));
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new(7, 27).solve_1(TEST_INPUT), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new(7, 27).solve_2(TEST_INPUT), 0);
    }
}
