use clap::Args;
use std::io::{BufRead, stdin};

/// Day 1: Secret Entrance
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase {
    ops: Vec<isize>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let ops = reader
            .lines()
            .map(|s| match s.unwrap().split_at(1) {
                ("L", v) => -1 * v.parse::<isize>().unwrap(),
                ("R", v) => v.parse().unwrap(),
                _ => unreachable!(),
            })
            .collect();
        Self { ops }
    }

    fn solve(&self) -> (usize, usize) {
        let (mut acc, mut sum1, mut sum2) = (50, 0, 0);
        for &v in self.ops.iter() {
            sum2 += if v > 0 { acc + v } else { (100 - acc) % 100 - v } / 100;
            acc = (acc + v).rem_euclid(100);
            sum1 += if acc == 0 { 1 } else { 0 };
        }
        (sum1, sum2 as _)
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("(part1, part2) = {:?}", t.solve());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        .trim();
        let t = TestCase::parse(input.as_bytes());
        assert_eq!(t.solve(), (3, 6));
    }

    #[test]
    fn test_large() {
        let input = r"R1000".trim();
        let t = TestCase::parse(input.as_bytes());
        assert_eq!(t.solve(), (0, 10));
    }

    #[test]
    fn test_lrlr() {
        let input = r"
L50
L10
R10
L10
R10
"
        .trim();
        let t = TestCase::parse(input.as_bytes());
        assert_eq!(t.solve(), (3, 3));
    }

    #[test]
    fn test_rlrl() {
        let input = r"
L50
R10
L10
R10
L10
"
        .trim();
        let t = TestCase::parse(input.as_bytes());
        assert_eq!(t.solve(), (3, 3));
    }
}
