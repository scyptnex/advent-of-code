use std::io;

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
    let mut total1: i64 = 0;
    let mut total2: i64 = 0;
    for line in io::stdin().lines() {
        let l = line.unwrap().trim().parse::<i64>().unwrap();
        total1 += fuel(l);
        total2 += fuel2(l);
    }
    println!("{}", total1);
    println!("{}", total2);
}

fn main() {
    aoc01();
}
