use aoc23::problem::*;

type Seed = (u64, u64);

fn seeds(input: &str) -> Vec<Seed> {
    input
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(str::parse)
        .map(Result::unwrap)
        .map(|i| (i, 1))
        .collect()
}

fn range_seeds(input: &str) -> Vec<Seed> {
    seeds(input)
        .as_slice()
        .windows(2)
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, w)| (w[0].0, w[1].0))
        .collect()
}

type Ranges = Vec<(u64, u64, u64)>;

fn shift(seed: &Seed, r: &Ranges) -> Vec<Seed> {
    let mut rem_seed: Seed = *seed;
    let mut out_seed = Vec::new();
    for (d, s, l) in r {
        //
        // s+l 0----------1
        if rem_seed.0 >= s + l {
            continue;
        }
        //     0----------1 s
        if *s >= rem_seed.0 + rem_seed.1 {
            break;
        }
        //     0-----s----1 s+l
        if *s > rem_seed.0 {
            let taken = s - rem_seed.0;
            out_seed.push((rem_seed.0, taken));
            rem_seed.0 = *s;
            rem_seed.1 -= taken;
            dbg!(rem_seed);
        }
        //     s-----s+l--1
        assert!(rem_seed.0 >= *s);
        let distance_from_s_to_0 = rem_seed.0 - s;
        let run_inside_r = std::cmp::min(rem_seed.1, l - distance_from_s_to_0);
        out_seed.push((d + distance_from_s_to_0, run_inside_r));
        rem_seed.0 += run_inside_r;
        rem_seed.1 -= run_inside_r;
    }
    if rem_seed.1 > 0 {
        out_seed.push(rem_seed);
    }
    out_seed
}

fn apply(s: Vec<Seed>, gmap: &str) -> Vec<Seed> {
    let mut ranges: Ranges = gmap
        .lines()
        .skip(1)
        .map(|l| {
            let (a, bc) = l.split_once(" ").unwrap();
            let (b, c) = bc.split_once(" ").unwrap();
            (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|(_, s, _)| *s);
    s.iter()
        .flat_map(|seed| shift(seed, &ranges).into_iter())
        .collect()
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        let mut groups = input.split("\n\n");
        let mut s = seeds(groups.next().unwrap());
        for gmap in groups {
            s = apply(s, gmap);
            dbg!(s.len());
        }
        s.iter().map(|s| s.0).min().unwrap()
    }
    fn solve_2(&self, input: &str) -> u64 {
        let mut groups = input.split("\n\n");
        let mut s = range_seeds(groups.next().unwrap());
        for gmap in groups {
            s = apply(s, gmap);
        }
        s.iter().map(|s| s.0).min().unwrap()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 35);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 46);
    }
}
