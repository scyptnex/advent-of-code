use aoc23::problem::*;

type Field = Vec<Vec<u8>>;

fn parser(input: &str) -> Field {
    input
        .lines()
        .map(|l| l.as_bytes().iter().cloned().collect())
        .collect()
}

fn roll_north(f: &mut Field) {
    for ri in 0..f.len() {
        for ci in 0..f[0].len() {
            if f[ri][ci] != b'O' {
                continue;
            }
            dbg!(ri, ci);
            for rx in (0..ri).map(|rx| ri - rx - 1) {
                if f[rx][ci] != b'.' {
                    break;
                }
                f[rx][ci] = f[rx + 1][ci];
                f[rx + 1][ci] = b'.';
            }
        }
    }
}

fn weight(f: &Field) -> u64 {
    f.iter()
        .enumerate()
        .map(|(ri, row)| (f.len() - ri) * row.iter().filter(|c| **c == b'O').count())
        .sum::<usize>() as u64
}

fn dmp(f: &Field) {
    for r in f {
        println!("{}", String::from_utf8_lossy(r.as_slice()));
    }
    println!();
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let mut f = parser(input);
        dmp(&f);
        roll_north(&mut f);
        dmp(&f);
        weight(&f)
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

    static TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 136);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 0);
    }
}
