use clap::Args;
use itertools::Itertools;
use std::io::{BufRead, stdin};

/// Day 2: Gift Shop
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase {
    ranges: Vec<(usize, usize)>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let ranges = reader
            .split(b',')
            .map(Result::unwrap)
            .map(|v| {
                let (a, b) = str::from_utf8(&v).unwrap().trim().split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        Self { ranges }
    }

    fn solve(&self, times: impl IntoIterator<Item = usize> + Clone) -> Vec<usize> {
        (1..1_00000)
            .flat_map(|v| {
                times
                    .clone()
                    .into_iter()
                    .map(move |n| format!("{v}").repeat(n))
                    .filter(|s| s.len() <= 10)
                    .map(|s| s.parse().unwrap())
            })
            .filter(|v| self.ranges.iter().any(|&(a, b)| (a..=b).contains(&v)))
            .sorted()
            .dedup()
            .collect()
    }

    fn part1(&self) -> Vec<usize> {
        self.solve([2])
    }

    fn part2(&self) -> Vec<usize> {
        self.solve(2..=10)
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("part1 = {}", t.part1().into_iter().sum::<usize>());
        println!("part2 = {}", t.part2().into_iter().sum::<usize>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part1(), vec![11, 22, 99, 1010, 222222, 446446, 38593859, 1188511885]);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(
            t.part2(),
            vec![
                11, 22, 99, 111, 999, 1010, 222222, 446446, 565656, 38593859, 824824824,
                1188511885, 2121212121
            ]
        );
    }
}
