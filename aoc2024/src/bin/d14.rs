use itertools::iproduct;
use regex::Regex;
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

struct TestCase {
    h: usize,
    w: usize,
    robots: Vec<(Point, Direction)>,
}

impl TestCase {
    fn parse(h: usize, w: usize, reader: impl BufRead) -> Self {
        let parse_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let robots = reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let cap = parse_re.captures(&line).unwrap();
                let parsed = cap
                    .iter()
                    .skip(1)
                    .map(|e| e.unwrap().as_str().parse().unwrap())
                    .collect::<Vec<isize>>();
                ((parsed[0], parsed[1]).into(), (parsed[2], parsed[3]).into())
            })
            .collect();
        Self { h, w, robots }
    }

    fn step(&self, (p, v): (Point, Direction), t: isize) -> Point {
        let q = p + v * t;
        (q.0.rem_euclid(self.w as isize), q.1.rem_euclid(self.h as isize)).into()
    }

    fn solve(&self) -> u64 {
        let (h, w) = (self.h as isize, self.w as isize);
        self.robots
            .iter()
            .fold([0; 4], |mut quad, &robot| {
                let Point(x, y) = self.step(robot, 100);
                quad[0] += if x < w / 2 && y < h / 2 { 1 } else { 0 };
                quad[1] += if x < w / 2 && y > h / 2 { 1 } else { 0 };
                quad[2] += if x > w / 2 && y < h / 2 { 1 } else { 0 };
                quad[3] += if x > w / 2 && y > h / 2 { 1 } else { 0 };
                quad
            })
            .into_iter()
            .product()
    }

    fn draw(&self, t: isize) -> Grid<char> {
        let mut canvas = Grid::new(' ', self.h, self.w);
        for &robot in self.robots.iter() {
            let Point(x, y) = self.step(robot, t);
            canvas[(y, x)] = '#';
        }
        canvas
    }
}

fn entropy(img: &Grid<char>) -> f64 {
    let hist: [usize; 1 << 4] = iproduct!(0..img.h - 1, 0..img.w - 1)
        .map(|(x, y)| {
            iproduct!(x..x + 2, y..y + 2).fold(0, |acc, ij| (img[ij] == '#') as usize | acc << 1)
        })
        .fold([0; 1 << 4], |mut hist, x| {
            hist[x] += 1;
            hist
        });
    let total = hist.iter().sum::<usize>() as f64;
    let pdf = hist.into_iter().filter(|&v| v > 0).map(|v| v as f64 / total);
    pdf.map(|p| -p * p.log2()).sum()
}

fn print_preview(img: &Grid<char>) {
    for x in (0..img.h).step_by(4) {
        for y in (0..img.w).step_by(4) {
            let s = iproduct!(x..x + 4, y..y + 4).filter(|&ij| img.get(ij) == Some(&'#')).count();
            let t = match s {
                0..=1 => ' ',
                2 => '.',
                3.. => '#',
            };
            print!("{t}");
        }
        println!();
    }
}

fn main() {
    let test_case = TestCase::parse(103, 101, stdin().lock());
    println!("Part1: {}", test_case.solve());
    let mut min_e = f64::MAX;
    for t in 0..101 * 103 {
        let img = test_case.draw(t);
        let e = entropy(&img);
        if min_e > e {
            min_e = e;
            println!("Step {t}, entropy {e:.8}");
            print_preview(&img);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let input = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
        .trim_ascii();
        assert_eq!(TestCase::parse(7, 11, input.as_bytes()).solve(), 12);
    }
}
