use aoc25::problem::*;

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let tp_is_1: Vec<Vec<usize>> = input
            .lines()
            .map(|l| l.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
            .collect();
        let rows = tp_is_1.len() as i64;
        let mut count = 0;
        for r in 0..rows {
            let cols = tp_is_1[r as usize].len() as i64;
            for c in 0..cols {
                if tp_is_1[r as usize][c as usize] != 1 {
                    continue;
                }
                let mut tpc = 0;
                for rd in -1..=1 {
                    for cd in -1..=1 {
                        if rd == 0 && cd == 0 {
                            continue;
                        }
                        let rv = r + rd;
                        let cv = c + cd;
                        if rv < 0 || rv >= rows || cv < 0 || cv >= cols {
                            continue;
                        }
                        tpc += tp_is_1[rv as usize][cv as usize];
                    }
                }
                if tpc < 4 {
                    count += 1;
                }
            }
        }
        count
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut tp_is_1: Vec<Vec<usize>> = input
            .lines()
            .map(|l| l.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
            .collect();
        let rows = tp_is_1.len() as i64;
        let mut tcount = 0;
        for _ in 1.. {
            let mut count = 0;
            for r in 0..rows {
                let cols = tp_is_1[r as usize].len() as i64;
                for c in 0..cols {
                    if tp_is_1[r as usize][c as usize] != 1 {
                        continue;
                    }
                    let mut tpc = 0;
                    for rd in -1..=1 {
                        for cd in -1..=1 {
                            if rd == 0 && cd == 0 {
                                continue;
                            }
                            let rv = r + rd;
                            let cv = c + cd;
                            if rv < 0 || rv >= rows || cv < 0 || cv >= cols {
                                continue;
                            }
                            tpc += tp_is_1[rv as usize][cv as usize];
                        }
                    }
                    if tpc < 4 {
                        count += 1;
                        tp_is_1[r as usize][c as usize] = 0;
                    }
                }
            }
            if count == 0 {
                break;
            }
            tcount += count;
        }
        tcount
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 43);
    }
}
