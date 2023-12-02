use aoc23::problem::*;

fn colour_is_possible(colour: &str) -> bool {
    let (count, col) = colour.split_once(" ").unwrap();
    let c = count.parse::<u64>().unwrap();
    match col {
        "red" => c <= 12,
        "green" => c <= 13,
        "blue" => c <= 14,
        _ => panic!(),
    }
}

fn game_is_possible(game: &str) -> bool {
    game.split(", ").all(colour_is_possible)
}

fn id_if_possible(line: &str) -> u64 {
    let (id, rest) = line.split_once(": ").unwrap();
    if rest.split("; ").all(game_is_possible) {
        return id.split_once(" ").unwrap().1.parse().unwrap();
    }
    0
}

fn cpower(v: &Vec<&str>, c: &str) -> u64 {
    v.iter()
        .filter(|l| l.ends_with(c))
        .map(|l| l.split_once(" ").unwrap().0.parse().unwrap())
        .max()
        .unwrap_or(0)
}

fn power(line: &str) -> u64 {
    let (_, rest) = line.split_once(": ").unwrap();
    let v = rest
        .split("; ")
        .flat_map(|g| g.split(", "))
        .collect::<Vec<&str>>();
    cpower(&v, "red") * cpower(&v, "blue") * cpower(&v, "green")
}

struct Prob {}

impl Prob {
    fn new() -> Self {
        Prob {}
    }
}

impl Problem<u64, u64> for Prob {
    fn solve_1(&self, input: &str) -> u64 {
        input.lines().map(id_if_possible).sum()
    }
    fn solve_2(&self, input: &str) -> u64 {
        input.lines().map(power).sum()
    }
}

fn main() {
    solve(Prob::new());
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_1() {
        assert_eq!(Prob::new().solve_1(TEST_INPUT), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(Prob::new().solve_2(TEST_INPUT), 2286);
    }
}
