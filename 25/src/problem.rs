use crate::input;
use std::fmt::Display;

pub trait Problem<T1, T2> {
    fn solve_1(&self, input: &str) -> T1;
    fn solve_2(&self, input: &str) -> T2;
}

pub fn solve<T1: Display, T2: Display>(p: impl Problem<T1, T2>) {
    let input = input::get_input_for_current_exe();
    let s1 = p.solve_1(&input);
    let s2 = p.solve_2(&input);
    println!("\nProblem 1:\n{}", s1);
    println!("\nProblem 2:\n{}", s2);
}

struct AutoProblem<T1, T2> {
    f1: fn(&str) -> T1,
    f2: fn(&str) -> T2,
}

impl<T1, T2> Problem<T1, T2> for AutoProblem<T1, T2> {
    fn solve_1(&self, input: &str) -> T1 {
        (self.f1)(input)
    }
    fn solve_2(&self, input: &str) -> T2 {
        (self.f2)(input)
    }
}

pub fn auto_solve<T1: Display, T2: Display>(f1: fn(&str) -> T1, f2: fn(&str) -> T2) {
    solve(AutoProblem { f1, f2 });
}
