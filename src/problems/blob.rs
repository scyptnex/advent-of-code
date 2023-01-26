use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::StructuredProblem;

struct Vxl {
    x: usize,
    y: usize,
    z: usize,
}

impl Vxl {
    fn new(s: &str) -> Self {
        let mut si = s.split(",");
        Vxl::of(
            si.next().unwrap().parse::<usize>().unwrap() + 1,
            si.next().unwrap().parse::<usize>().unwrap() + 1,
            si.next().unwrap().parse::<usize>().unwrap() + 1,
        )
    }

    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Vxl::of(x, y, z)
    }

    fn of(x: usize, y: usize, z: usize) -> Self {
        Vxl { x, y, z }
    }

    fn adj_in_bounds(&self, b: &(usize, usize, usize)) -> Vec<Vxl> {
        let mut r: Vec<Vxl> = Vec::new();
        if self.x > 0 {
            r.push(Vxl::of(self.x - 1, self.y, self.z));
        }
        if self.y > 0 {
            r.push(Vxl::of(self.x, self.y - 1, self.z));
        }
        if self.z > 0 {
            r.push(Vxl::of(self.x, self.y, self.z - 1));
        }
        if self.x + 1 < b.0 {
            r.push(Vxl::of(self.x + 1, self.y, self.z));
        }
        if self.y + 1 < b.1 {
            r.push(Vxl::of(self.x, self.y + 1, self.z));
        }
        if self.z + 1 < b.2 {
            r.push(Vxl::of(self.x, self.y, self.z + 1));
        }
        r
    }
}

type VxlMap = Vec<Vec<Vec<bool>>>;

fn adj(x: usize, y: usize, z: usize) -> [(usize, usize, usize); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn exposed_faces(vm: &VxlMap, x: usize, y: usize, z: usize) -> usize {
    adj(x, y, z)
        .iter()
        .filter(|(xc, yc, zc)| !vm[*xc][*yc][*zc])
        .count()
}

fn surface_area(vm: &VxlMap) -> usize {
    vm.iter()
        .enumerate()
        .map(|(xi, vx)| {
            vx.iter()
                .enumerate()
                .map(|(yi, vy)| {
                    vy.iter()
                        .enumerate()
                        .filter(|(_, p)| **p)
                        .map(|(zi, _)| exposed_faces(vm, xi, yi, zi))
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn fill(hollow: VxlMap) -> VxlMap {
    let mut hollow = hollow;
    let mut filled = hollow.clone();

    let bounds = (hollow.len(), hollow[0].len(), hollow[0][0].len());
    let mut flr = vec![Vxl { x: 0, y: 0, z: 0 }];
    while !flr.is_empty() {
        for f in flr.iter() {
            hollow[f.x][f.y][f.z] = true;
        }
        let nxt_flr: HashSet<(usize, usize, usize)> = flr
            .iter()
            .flat_map(|f| f.adj_in_bounds(&bounds).into_iter())
            .map(|v| (v.x, v.y, v.z))
            .filter(|(x, y, z)| !hollow[*x][*y][*z])
            .collect();
        flr = nxt_flr.into_iter().map(|v| Vxl::from(v)).collect();
    }

    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            for z in 0..bounds.2 {
                if !hollow[x][y][z] {
                    filled[x][y][z] = true;
                }
            }
        }
    }

    filled
}

#[derive(Default)]
pub struct Blob {
    data: Vec<Vxl>,
}

impl Blob {
    fn read<I: Iterator<Item = String>>(&mut self, i: I) {
        self.data = i.map(|s| Vxl::new(&s)).collect();
    }

    fn bounds(&self) -> (usize, usize, usize) {
        self.data.iter().fold((0, 0, 0), |cur, nxt| {
            (
                cur.0.max(nxt.x + 1),
                cur.1.max(nxt.y + 1),
                cur.2.max(nxt.z + 1),
            )
        })
    }

    fn to_vxl(&self) -> VxlMap {
        let (bx, by, bz) = self.bounds();
        let mut v = vec![vec![vec![false; bz + 1]; by + 1]; bx + 1];
        for vxl in self.data.iter() {
            v[vxl.x][vxl.y][vxl.z] = true;
        }
        v
    }
}

impl StructuredProblem for Blob {
    fn ingest(&mut self, f: File) {
        self.read(BufReader::new(f).lines().map(|s| s.unwrap()));
    }
    fn solve_1(&self) -> Box<dyn Display> {
        Box::new(surface_area(&self.to_vxl()))
    }
    fn solve_2(&self) -> Box<dyn Display> {
        Box::new(surface_area(&fill(self.to_vxl())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> Blob {
        let mut t = Blob::default();
        t.read(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
                .lines()
                .map(|s| String::from(s)),
        );
        t
    }

    #[test]
    fn test_answer() {
        let t = data();
        assert_eq!(format!("{}", t.solve_1()), "64");
        assert_eq!(format!("{}", t.solve_2()), "58");
    }

    #[test]
    fn test_bounds() {
        assert_eq!(data().bounds(), (5, 5, 8));
    }

    #[test]
    fn test_voxel() {
        let v = Vxl::new("1,2,3");
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 3);
        assert_eq!(v.z, 4);
    }
}
