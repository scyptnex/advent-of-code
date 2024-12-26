use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

#[derive(Default)]
pub struct Filesys {
    data: Vec<String>,
}

impl StructuredProblem for Filesys {
    fn ingest(&mut self, f: File) {
        self.data = BufReader::new(f).lines().map(|s| s.unwrap()).collect();
    }
    fn solve_1(&self) -> Box<dyn Display> {
        let fs = parse(&self.data);
        Box::new(fs.interior_by_threshold(100000))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        let fs = parse(&self.data);
        Box::new(fs.cleanify(70000000, 30000000))
    }
}

#[derive(Debug)]
struct FsFile {
    size: usize,
}

impl FsFile {
    fn new(ls_line: &Vec<&str>) -> Self {
        FsFile {
            size: ls_line[0].parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct FsDir {
    name: String,
    f_contents: Vec<FsFile>,
    d_contents: Vec<Box<FsDir>>,
}

impl FsDir {
    fn new(n: String) -> Self {
        FsDir {
            name: n,
            f_contents: Vec::default(),
            d_contents: Vec::default(),
        }
    }

    fn interior_by_threshold(&self, threshold: usize) -> usize {
        let sub_size = self
            .d_contents
            .iter()
            .map(|d| d.interior_by_threshold(threshold))
            .sum();
        let s = self.size();
        if s < threshold {
            return sub_size + s;
        }
        sub_size
    }

    fn cleanify(&self, total: usize, desired: usize) -> usize {
        let free = total - self.size();
        let needed = desired - free;
        self.smallest_greater_than(needed).unwrap()
    }

    fn smallest_greater_than(&self, needed: usize) -> Option<usize> {
        let s = self.size();
        if s < needed {
            return None;
        }
        self.d_contents
            .iter()
            .filter_map(|d| d.smallest_greater_than(needed))
            .min()
            .or(Some(s))
    }

    fn parse(&mut self, input: &Vec<String>, start: usize) -> usize {
        let mut cur = start;
        while cur < input.len() {
            let sp: Vec<&str> = input[cur].split(" ").collect();
            if sp[0] != "$" {
                self.add_item(&sp);
                cur += 1;
            } else if sp[1] == "ls" {
                cur += 1;
            } else if sp[2] == ".." {
                return cur + 1;
            } else if sp[2] == "/" {
                if self.name != "/" {
                    return cur;
                } else {
                    cur += 1;
                }
            } else {
                cur = self.recurse(sp[2]).parse(input, cur + 1);
            }
        }
        cur
    }

    fn add_item(&mut self, ls_line: &Vec<&str>) {
        if ls_line[0] == "dir" {
            self.d_contents
                .push(Box::new(FsDir::new(ls_line[1].to_string())))
        } else {
            self.f_contents.push(FsFile::new(&ls_line))
        }
    }

    fn size(&self) -> usize {
        self.f_contents.iter().map(|c| c.size).sum::<usize>()
            + self.d_contents.iter().map(|d| d.size()).sum::<usize>()
    }

    fn recurse(&mut self, dir: &str) -> &mut FsDir {
        self.d_contents
            .iter_mut()
            .find(|d| d.name == dir)
            .unwrap()
            .as_mut()
    }
}

fn parse(data: &Vec<String>) -> FsDir {
    let mut root = FsDir::new("/".to_string());
    root.parse(data, 0);
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let fs = Filesys {
            data: "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
                .lines()
                .map(|s| String::from(s))
                .collect(),
        };
        let t = parse(&fs.data);
        assert_eq!(t.interior_by_threshold(100000), 95437);
        assert_eq!(t.cleanify(70000000, 30000000), 24933642);
    }
}
