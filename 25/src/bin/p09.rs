use aoc25::*;

type Til = (i64, i64);
type Tilez = std::collections::HashSet<Til>;

fn adj() -> impl Iterator<Item = Til> {
    (-1..=1)
        .flat_map(|xd| (-1..=1).map(move |yd| (xd, yd)))
        .filter(|t| t.0 != 0 || t.1 != 0)
}

fn adt(t1: &Til, t2: &Til) -> Til {
    (t1.0 + t2.0, t1.1 + t2.1)
}

fn solve_1(input: &str) -> u64 {
    let coords = input
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    let area = |(i, j)| -> i64 {
        let (xi, yi): (i64, i64) = coords[i];
        let (xj, yj): (i64, i64) = coords[j];
        ((xi - xj).abs() + 1) * ((yi - yj).abs() + 1)
    };
    (0..coords.len() - 1)
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
        .map(area)
        .max()
        .unwrap() as u64
}
fn solve_2(input: &str) -> u64 {
    let coords = input
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<Til>>();

    let mut gs = Tilez::new();
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        let (xi, yi) = coords[i];
        let (xj, yj) = coords[j];
        for x in std::cmp::min(xi, xj)..std::cmp::max(xi, xj) + 1 {
            for y in std::cmp::min(yi, yj)..std::cmp::max(yi, yj) + 1 {
                gs.insert((x, y));
            }
        }
    }
    let gs = gs;
    dbg!(gs.len());

    let adjs = gs
        .iter()
        .flat_map(|t| adj().map(|a| adt(&a, t)))
        .collect::<Tilez>();

    let perims = adjs.difference(&gs).copied().collect::<Tilez>();

    let arbitrary_p = *perims.iter().next().unwrap();
    let mut side_a = Tilez::new();
    let mut front = Vec::<Til>::new();
    front.push(arbitrary_p);
    side_a.insert(arbitrary_p);
    while let Some(f) = front.pop() {
        for a in adj().map(|a| adt(&a, &f)) {
            if side_a.contains(&a) || gs.contains(&a) || !perims.contains(&a) {
                continue;
            }
            front.push(a);
            side_a.insert(a);
        }
    }
    let side_b = perims.difference(&side_a).copied().collect::<Tilez>();
    let min_a = side_a.iter().map(|t| t.0).min();
    let min_b = side_b.iter().map(|t| t.0).min();
    let outside = match min_a < min_b {
        true => side_a,
        false => side_b,
    };
    dbg!(min_a, min_b);

    let area = |(i, j)| -> i64 {
        let (xi, yi): (i64, i64) = coords[i];
        let (xj, yj): (i64, i64) = coords[j];
        ((xi - xj).abs() + 1) * ((yi - yj).abs() + 1)
    };
    (0..coords.len() - 1)
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, area((i, j))))
        .filter(|(i, j, _)| {
            let (xi, yi): (i64, i64) = coords[*i];
            let (xj, yj): (i64, i64) = coords[*j];
            (std::cmp::min(xi, xj)..=std::cmp::max(xi, xj))
                .map(|x| (x, yi))
                .all(|t| !outside.contains(&t))
                && (std::cmp::min(xi, xj)..=std::cmp::max(xi, xj))
                    .map(|x| (x, yj))
                    .all(|t| !outside.contains(&t))
                && (std::cmp::min(yi, yj)..=std::cmp::max(yi, yj))
                    .map(|y| (xi, y))
                    .all(|t| !outside.contains(&t))
                && (std::cmp::min(yi, yj)..=std::cmp::max(yi, yj))
                    .map(|y| (xj, y))
                    .all(|t| !outside.contains(&t))
        })
        .map(|g| g.2)
        .max()
        .unwrap() as u64

    // let mut inside = Tilez::new();
    // for x in minx..maxx {
    //     for y in miny..maxy {
    //         let c = (x, y);
    //         if !outside.contains(&c) {
    //             inside.insert(c);
    //         }
    //     }
    // }
    // dbg!(inside.len());

    // (0..coords.len() - 1)
    //     .flat_map(|i| (i + 1..coords.len()).map(move |j| (i, j)))
    //     .flat_map(|(i, j)| {
    //         let (xi, yi): (i64, i64) = coords[i];
    //         let (xj, yj): (i64, i64) = coords[j];
    //         for x in std::cmp::min(xi, xj)..std::cmp::max(xi, xj) + 1 {
    //             for y in std::cmp::min(yi, yj)..std::cmp::max(yi, yj) + 1 {
    //                 if outside.contains(&(x, y)) {
    //                     return None;
    //                 }
    //             }
    //         }
    //         Some(((xi - xj).abs() + 1) * ((yi - yj).abs() + 1))
    //     })
    //     .max()
    //     .unwrap() as u64
}

fn main() {
    auto_solve(solve_1, solve_2);
}

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_1() {
        assert_eq!(solve_1(TEST_INPUT), 50);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(TEST_INPUT), 24);
    }
}
