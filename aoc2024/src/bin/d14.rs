use itertools::Itertools;
use regex::Regex;
use std::io::{stdin, BufRead, Cursor};

struct Robot((i64, i64), (i64, i64));

struct TestCase {
    w: i64,
    h: i64,
    robots: Vec<Robot>,
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Robot> {
    let parse_re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let cap = parse_re.captures(&line).unwrap();
            let parsed = cap
                .iter()
                .skip(1)
                .map(|e| e.unwrap().as_str().parse().unwrap())
                .collect::<Vec<_>>();
            Robot((parsed[0], parsed[1]), (parsed[2], parsed[3]))
        })
        .collect()
}

fn solve(data: &TestCase) -> u64 {
    let &TestCase { w, h, ref robots } = data;
    let mut q: [u64; 4] = [0; 4];
    for &Robot(p, v) in robots.iter() {
        let (p1, p2) = p;
        let (v1, v2) = v;
        let x = (p1 + v1 * 100).rem_euclid(w);
        let y = (p2 + v2 * 100).rem_euclid(h);
        q[0] += if x < w / 2 && y < h / 2 { 1 } else { 0 };
        q[1] += if x < w / 2 && y > h / 2 { 1 } else { 0 };
        q[2] += if x > w / 2 && y < h / 2 { 1 } else { 0 };
        q[3] += if x > w / 2 && y > h / 2 { 1 } else { 0 };
    }
    q.iter().product()
}

fn print_tree(data: &TestCase, steps: usize) {
    let steps = steps as i64;
    let &TestCase { w, h, ref robots } = data;
    let mut canvas = vec![vec![' '; w as usize]; h as usize];
    for &Robot(p, v) in robots.iter() {
        let (p1, p2) = p;
        let (v1, v2) = v;
        let x = (p1 + v1 * steps).rem_euclid(w);
        let y = (p2 + v2 * steps).rem_euclid(h);
        canvas[y as usize][x as usize] = '#';
    }

    let found = (0..h as usize - 2)
        .cartesian_product(0..w as usize - 2)
        .any(|(i, j)| {
            canvas[i..i + 3]
                .iter()
                .flat_map(|c| c[j..j + 3].iter())
                .all(|&e| e == '#')
        });
    if !found {
        return;
    }
    println!("Step {steps}:");
    for row in canvas.iter() {
        for col in row.iter() {
            print!("{col}");
        }
        println!();
    }
}

fn main() {
    let test_case = TestCase {
        w: 101,
        h: 103,
        robots: parse_input(stdin().lock()),
    };
    println!("{:?}", solve(&test_case));

    for n in 0..101 * 103 {
        print_tree(&test_case, n);
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
";
        let input_reader = Cursor::new(input.trim());
        let test_case = TestCase {
            w: 11,
            h: 7,
            robots: parse_input(input_reader),
        };
        assert_eq!(solve(&test_case), 12);
    }
}
