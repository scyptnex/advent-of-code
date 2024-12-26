use aoc23::problem::*;

fn parse_prob(line: &str) -> (Vec<u8>, Vec<usize>) {
    let x = line.split_once(" ").unwrap();
    (
        x.0.as_bytes().iter().copied().collect(),
        x.1.split(",").map(str::parse).map(Result::unwrap).collect(),
    )
}

fn appendulize(inp: (Vec<u8>, Vec<usize>)) -> (Vec<u8>, Vec<usize>) {
    let mut x = inp;
    x.0.push(b'.');
    x
}

fn unfoldulate(inp: (Vec<u8>, Vec<usize>)) -> (Vec<u8>, Vec<usize>) {
    let mut out_pat = inp.0.clone();
    let mut out_seq = inp.1.clone();
    for _ in 1..5 {
        out_pat.push(b'?');
        out_pat.append(&mut inp.0.clone());
        out_seq.append(&mut inp.1.clone());
    }
    (out_pat, out_seq)
}

fn can_start(se: usize, idx: usize, pat: &Vec<u8>) -> bool {
    let end = idx + se;
    pat[end] != b'#' && !pat[idx..end].iter().any(|p| *p == b'.')
}

fn count_configs(pat: Vec<u8>, seq: Vec<usize>) -> u64 {
    let mut table: Vec<Vec<u64>> = vec![vec![0; pat.len() + 1]; seq.len() + 1];
    // There is exactly 1 way or arranging 0 seqences on 0 tiles...
    table[0][0] = 1;
    // using 0 sequences, there is always 1 configuration until you get to a #
    for i in 0..pat.len() {
        if pat[i] == b'#' {
            break;
        }
        table[0][i + 1] = 1;
    }
    for (si, se) in seq.iter().enumerate() {
        for p_start_idx in 0..pat.len() - se {
            let before = table[si][p_start_idx];
            // If we cant build a pattern with pieces before this seq, skip
            if before == 0 {
                continue;
            }
            if !can_start(*se, p_start_idx, &pat) {
                continue;
            }
            for write_idx in p_start_idx + se..pat.len() {
                if pat[write_idx] == b'#' {
                    break;
                }
                table[si + 1][write_idx + 1] += before;
            }
        }
    }
    table[seq.len()][pat.len()]
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input
            .lines()
            .map(parse_prob)
            .map(appendulize)
            .map(|c| count_configs(c.0, c.1))
            .sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input
            .lines()
            .map(parse_prob)
            .map(unfoldulate)
            .map(appendulize)
            .map(|c| count_configs(c.0, c.1))
            .sum()
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
