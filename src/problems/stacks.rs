use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use super::Problem;

pub struct Stacks {}

impl Problem for Stacks {
    fn solve(&self, f: File) -> Result<(), Box<dyn Error>> {
        let (initial, cmds) = splitify(BufReader::new(f).lines().map(|s| s.unwrap()));
        println!("{}", Pile::new(&initial).do_commands(&cmds, 9000).get_seq());
        println!("{}", Pile::new(&initial).do_commands(&cmds, 9001).get_seq());
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Pile {
    pile: Vec<Vec<u8>>,
}

impl Pile {
    fn new(initial: &Vec<String>) -> Self {
        let count = (initial.last().unwrap().len() + 1) / 4;
        Pile {
            pile: (0..count)
                .map(|i| {
                    initial
                        .iter()
                        .rev()
                        .skip(1)
                        .map(|s| s.as_bytes()[i * 4 + 1])
                        .filter(|c| c != &b' ')
                        .collect()
                })
                .collect(),
        }
    }

    fn do_commands(&mut self, cmds: &Vec<String>, vsn: usize) -> &mut Self {
        for c in cmds {
            let mut split_c = c.split(' ').filter_map(|s| s.parse::<usize>().ok());
            let n = split_c.next().unwrap() as u32;
            let from = split_c.next().unwrap() - 1;
            let to = split_c.next().unwrap() - 1;
            match vsn {
                9000 => self.move_rev_n(from, to, n),
                9001 => self.move_n(from, to, n),
                _ => panic!("unknown crane version"),
            }
        }
        self
    }

    fn get_seq(&self) -> String {
        self.pile
            .iter()
            .map(|v| *v.last().unwrap() as char)
            .collect()
    }

    fn move_n(&mut self, from: usize, to: usize, n: u32) {
        let mut stack: Vec<u8> = Vec::new();
        for _ in 0..n {
            stack.push(self.pile[from].pop().unwrap());
        }
        stack.reverse();
        self.pile[to].append(&mut stack);
    }

    fn move_rev_n(&mut self, from: usize, to: usize, n: u32) {
        for _ in 0..n {
            self.move_n(from, to, 1);
        }
    }
}

fn splitify<I, T>(input: I) -> (Vec<String>, Vec<String>)
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let mut initial: Vec<String> = Vec::new();
    let mut cmds: Vec<String> = Vec::new();
    let mut swapped = false;
    for l in input {
        if l.as_ref().is_empty() {
            swapped = true;
        } else if swapped {
            cmds.push(l.as_ref().to_string());
        } else {
            initial.push(l.as_ref().to_string());
        }
    }
    (initial, cmds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitify() {
        let data = "a\nb\nc\n\nd\ne".lines();
        let (s, t) = splitify(data);
        assert_eq!(s, vec!["a", "b", "c"]);
        assert_eq!(t, vec!["d", "e"]);
    }

    #[test]
    fn test_pile() {
        let data: Vec<String> = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "
            .lines()
            .map(|s| String::from(s))
            .collect();
        let mut p = Pile::new(&data);
        assert_eq!(
            p.pile,
            vec![vec![b'Z', b'N'], vec![b'M', b'C', b'D'], vec![b'P']]
        );
        assert_eq!(p.get_seq(), "NDP");

        let cmds: Vec<String> = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .lines()
            .map(|s| String::from(s))
            .collect();
        p.do_commands(&cmds, 9000);
        assert_eq!(p.get_seq(), "CMZ");

        let mut p2 = Pile::new(&data);
        p2.do_commands(&cmds, 9001);
        assert_eq!(p2.get_seq(), "MCD");
    }

    #[test]
    fn test_move() {
        let mut p = Pile {
            pile: vec![vec![1, 2], vec![3, 4], vec![5, 6]],
        };
        p.move_n(1, 2, 1);
        assert_eq!(p.pile, vec![vec![1, 2], vec![3], vec![5, 6, 4]]);
        p.move_n(2, 0, 2);
        assert_eq!(p.pile, vec![vec![1, 2, 6, 4], vec![3], vec![5]]);
        p.move_rev_n(0, 2, 2);
        assert_eq!(p.pile, vec![vec![1, 2], vec![3], vec![5, 4, 6]]);
    }
}
