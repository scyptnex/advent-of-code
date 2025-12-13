use aoc25::*;

fn fewest(input: &str) -> u64 {
    let (sqb, input) = input.split_once("]").unwrap();
    let (btn, _) = input.split_once("{").unwrap();
    let buttons = btn
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once('(').unwrap().1)
        .map(|s| s.split_once(')').unwrap().0)
        .map(|s| {
            let mut n = 0;
            for v in s.split(',') {
                n |= 1u64 << v.parse::<usize>().unwrap();
            }
            n
        })
        .collect::<Vec<u64>>();
    let mut target: u64 = 0;
    for (i, t) in sqb.chars().skip(1).enumerate() {
        if t == '.' {
            continue;
        }
        target |= 1 << i;
    }
    let mut visited = std::collections::HashSet::<u64>::new();
    let mut frontier = vec![0];
    visited.insert(0);
    for generation in 1.. {
        let mut new_front = Vec::new();
        for f in frontier {
            for b in &buttons {
                let f2 = f ^ b;
                if f2 == target {
                    return generation;
                }
                if visited.insert(f2) {
                    new_front.push(f2);
                }
            }
        }
        frontier = new_front;
    }
    0
}

fn solve_1(input: &str) -> u64 {
    input.lines().map(fewest).sum()
}
fn solve_2(input: &str) -> u64 {
    (input.len() - input.len()) as u64
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 0);
    }
}
