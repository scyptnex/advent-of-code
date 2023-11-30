use crate::input;

pub trait Problem<T1, T2> {
    fn solve_1(&self, input: &str) -> T1;
    fn solve_2(&self, input: &str) -> T2;
}

pub fn solve<T1: std::fmt::Display, T2: std::fmt::Display>(p: impl Problem<T1, T2>) {
    let input = input::get_input_for_current_exe();
    let s1 = p.solve_1(&input);
    let s2 = p.solve_2(&input);
    println!("Problem 1:\n{}", s1);
    println!("Problem 2:\n{}", s2);
}
