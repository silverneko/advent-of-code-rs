use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
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

    fn step(&self, t: isize) -> Vec<Point> {
        let (h, w) = (self.h as isize, self.w as isize);
        self.robots
            .iter()
            .map(|(p, v)| {
                let q = p + v * t;
                (q.0.rem_euclid(w), q.1.rem_euclid(h)).into()
            })
            .collect()
    }

    fn solve(&self) -> u64 {
        let (h, w) = (self.h as isize, self.w as isize);
        self.step(100)
            .into_iter()
            .fold([0; 4], |mut quad, Point(x, y)| {
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
        for Point(x, y) in self.step(t) {
            canvas[(y, x)] = '#';
        }
        canvas
    }
}

fn entropy<T: Eq + Hash>(samples: &[T]) -> f64 {
    let hist: HashMap<_, usize> = samples.iter().counts();
    let sum = hist.values().sum::<usize>() as f64;
    let pdf = hist.into_values().map(|v| v as f64 / sum);
    pdf.map(|p| -p * p.log2()).sum()
}

/*
 * Let mx = has minimum entropy w.r.t. x axis
 * Let my = has minimum entropy w.r.t. y axis
 * Find t such that both x and y have minimum entropy.
 * Known:
 *   h and w are primes
 *   t = mx (mod w)
 *   t = my (mod h)
 *   t = t (mod w*h)
 * Which means:
 *   t = mx + a * w
 *   mx + a * w = my (mod h)
 *   a = (my - mx) * w^-1 (mod h)
 * Thus:
 *   k = w^-1 (mod h)
 *   t = mx + (my - mx) * k * w (mod w*h)
 */
fn main() {
    let data = TestCase::parse(103, 101, stdin().lock());
    println!("Part1: {}", data.solve());

    let (h, w) = (data.h as isize, data.w as isize);
    let (_, mx) = (0..w)
        .map(|t| (entropy(&data.step(t).into_iter().map(|Point(x, _)| x).collect::<Vec<_>>()), t))
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .unwrap();
    let (_, my) = (0..h)
        .map(|t| (entropy(&data.step(t).into_iter().map(|Point(_, y)| y).collect::<Vec<_>>()), t))
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .unwrap();
    let k = (0..h).find(|x| x * w % h == 1).expect("modulus inverse of w must exist");
    let t = (mx + (my - mx) * k * w).rem_euclid(h * w);
    println!("Step {t}:\n{}", data.draw(t));
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
