use std::fs;
use std::io::{prelude::*, BufReader};

fn fuel(x: i64) -> i64 {
    (x / 3) - 2
}

fn fuel2(x: i64) -> i64 {
    let mut total: i64 = 0;
    let mut add: i64 = x;
    loop {
        let new = fuel(add);
        if new <= 0 {
            return total;
        }
        add = new;
        total += add;
    }
}

fn aoc01() {
    let filename = "input/1b.txt";

    println!("{}", fuel2(12));
    println!("{}", fuel2(14));
    println!("{}", fuel2(1969));
    println!("{}", fuel2(100756));
    println!();

    let mut total: i64 = 0;
    let f = fs::File::open(filename).unwrap();
    let r = BufReader::new(f);

    for line in r.lines() {
        total += fuel2(line.unwrap().trim().parse::<i64>().unwrap());
    }
    println!("{}", total);
}

fn main() {
    aoc01();
}
