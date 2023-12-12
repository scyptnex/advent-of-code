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

fn can_place(run: usize, pat: &[u8], pos: usize) -> bool {
    pat[pos + 1..pos + 1 + run].iter().all(|x| *x != b'.')
        && pat[0] != b'#'
        && pat[pos + 1 + run] == b'#'
}

fn explore(pat: &mut [u8], seq: &[usize]) -> u64 {
    let tot = seq.iter().sum::<usize>() + seq.len() + 1;
    dbg!(&pat, tot, &seq);
    if pat.len() < tot {
        return 0;
    }
    let cur = seq[0] + 2;
    let mut total = 0;
    for i in 0..pat.len() - cur {
        if !can_place(seq[0], pat, i) {
            dbg!(i);
            break;
        }
        let saved = pat[i + 1 + seq[0]];
        pat[i + 1 + seq[0]] = b'.';
        total += explore(&mut pat[i + seq[0] + 1..], seq);
        pat[i + 1 + seq[0]] = saved;
    }
    total
}

fn count_configs(line: &str) -> u64 {
    let (pattern, seq) = line.split_once(" ").unwrap();
    let mut pat = pattern.as_bytes().iter().copied().collect::<Vec<u8>>();
    pat.insert(0, b'.');
    pat.push(b'.');
    let seqv = seq
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();
    explore(&mut pat, &seqv)
}

fn count_configs_2(line: &str) -> u64 {
    0
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().take(1).map(count_configs).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(count_configs_2).sum()
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
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 525152);
    }
}
