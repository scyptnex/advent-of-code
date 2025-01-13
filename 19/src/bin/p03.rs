use std::collections::HashMap;
use std::io;

type Coord = (isize, isize);

#[derive(Debug)]
enum Drc {
    U,
    D,
    L,
    R,
}

fn nxt(d: &Drc, c: &Coord) -> Coord {
    match d {
        Drc::U => (c.0, c.1 + 1),
        Drc::D => (c.0, c.1 - 1),
        Drc::R => (c.0 + 1, c.1),
        Drc::L => (c.0 - 1, c.1),
    }
}

type Wire = (Drc, usize);

fn mhd(c: &Coord) -> isize {
    c.0.abs() + c.1.abs()
}

fn main() {
    let x: Vec<Vec<Wire>> = io::stdin()
        .lines()
        .flatten()
        .map(|s| {
            s.split(',')
                .map(|s| {
                    (
                        match s.chars().next().unwrap() {
                            'U' => Drc::U,
                            'D' => Drc::D,
                            'L' => Drc::L,
                            'R' => Drc::R,
                            _ => panic!(),
                        },
                        s[1..].parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    let mut lst = (0, 0);
    let mut len = 0;
    let mut dst = HashMap::<Coord, usize>::new();
    for (d, l) in &x[0] {
        for _ in 0..*l {
            len += 1;
            lst = nxt(d, &lst);
            if !dst.contains_key(&lst) {
                dst.insert(lst, len);
            }
        }
    }
    lst = (0, 0);
    len = 0;
    let mut closest = (0, 0);
    let mut btd = 0;
    for (d, l) in &x[1] {
        for _ in 0..*l {
            len += 1;
            lst = nxt(d, &lst);
            if let Some(w1d) = dst.get(&lst) {
                if closest == (0, 0) || mhd(&closest) > mhd(&lst) {
                    closest = lst;
                }
                let ctd = w1d + len;
                if btd == 0 || ctd < btd {
                    btd = ctd;
                }
            }
        }
    }
    println!("{}", mhd(&closest));
    println!("{}", btd);
}
