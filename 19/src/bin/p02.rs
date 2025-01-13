use std::io;

fn compu(n: usize, v: usize, x: Vec<usize>) -> usize {
    let mut x = x;
    x[1] = n;
    x[2] = v;
    let mut cur = 0;
    loop {
        match x[cur] {
            1 => {
                let lhs = x[x[cur + 1]];
                let rhs = x[x[cur + 2]];
                let ans = x[cur + 3];
                x[ans] = lhs + rhs;
            }
            2 => {
                let lhs = x[x[cur + 1]];
                let rhs = x[x[cur + 2]];
                let ans = x[cur + 3];
                x[ans] = lhs * rhs;
            }
            99 => break,
            _ => panic!(),
        }
        cur += 4;
    }
    return x[0];
}

fn compu2(x: Vec<usize>) -> usize {
    for n in 0..=99 {
        for v in 0..=99 {
            if compu(n, v, x.clone()) == 19690720 {
                return 100 * n + v;
            }
        }
    }
    panic!();
}

fn main() {
    let x: Vec<usize> = io::stdin()
        .lines()
        .flatten()
        .flat_map(|s| s.split(',').map(str::parse).flatten().collect::<Vec<_>>())
        .collect();
    println!("{}", compu(12, 2, x.clone()));
    println!("{}", compu2(x));
}
