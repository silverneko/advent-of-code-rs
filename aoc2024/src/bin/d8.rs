use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use utils::{Gcd, Grid, Point};

fn main() {
    let grid: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line| line.map(|e| e.into()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let grid = Grid::from(grid);

    let mut ant: HashMap<u8, Vec<Point>> = HashMap::new();
    let mut ans1: HashSet<Point> = HashSet::new();
    let mut ans2: HashSet<Point> = HashSet::new();

    for i in 0..grid.h {
        for j in 0..grid.w {
            match grid[(i, j)] {
                b'.' => {}
                a @ (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9') => {
                    ant.entry(a).or_default().push(Point::from((i, j)));
                }
                a @ _ => panic!("{a:?}"),
            }
        }
    }

    for (_, v) in ant {
        for pair in v.iter().combinations(2) {
            let [&pa, &pb] = pair.try_into().unwrap();
            let d = pb - pa;

            // part 1
            for p in [pa - d, pb + d] {
                if let Some(_) = grid.get(p) {
                    ans1.insert(p);
                }
            }

            // part 2
            let Point(x, y) = d;
            let g = x.gcd(y);
            let dv = d / g;

            let mut p = pa;
            while let Some(_) = grid.get(p) {
                ans2.insert(p);
                p = p - dv;
            }
            p = pb;
            while let Some(_) = grid.get(p) {
                ans2.insert(p);
                p = p + dv;
            }
        }
    }

    println!();
    for i in 0..grid.h {
        for j in 0..grid.w {
            if ans2.contains(&Point::from((i, j))) {
                print!("#");
            } else {
                print!("{}", std::str::from_utf8(&[grid[(i, j)]]).unwrap());
            }
        }
        println!();
    }

    println!("{},{}", ans1.len(), ans2.len());
}
