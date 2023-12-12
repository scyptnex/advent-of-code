use aoc23::problem::*;

fn rec_configs(pattern: &mut Vec<char>, idx: usize, seq: &Vec<usize>) -> u64 {
    if let Some(nxt) = (idx..pattern.len()).find(|j| pattern[*j] == '?') {
        pattern[nxt] = '.';
        let part = rec_configs(pattern, nxt + 1, seq);
        pattern[nxt] = '#';
        let part2 = rec_configs(pattern, nxt + 1, seq);
        pattern[nxt] = '?';
        part + part2
    } else {
        let mut found_pat: Vec<usize> = vec![];
        let mut cur_count = 0;
        for p in pattern.iter() {
            if *p == '#' {
                cur_count += 1;
            } else if cur_count != 0 {
                found_pat.push(cur_count);
                cur_count = 0;
            }
        }
        if cur_count != 0 {
            found_pat.push(cur_count);
        }
        match &found_pat == seq {
            true => 1,
            false => 0,
        }
    }
}

fn count_configs(line: &str) -> u64 {
    let (pattern, seq) = line.split_once(" ").unwrap();
    rec_configs(
        &mut pattern.chars().collect(),
        0,
        &seq.split(",").map(str::parse).map(Result::unwrap).collect(),
    )
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(count_configs).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(count_configs).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 0);
    }
}
