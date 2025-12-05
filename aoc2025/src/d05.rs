use clap::Args;
use itertools::Itertools;
use std::io::{BufRead, stdin};

/// Day 5: Cafeteria
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase(Vec<(u64, u64)>, Vec<u64>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(Result::unwrap);
        let ranges = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|s| {
                let (a, b) = s.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .sorted()
            .collect();
        let query = lines.map(|s| s.parse().unwrap()).collect();
        Self(ranges, query)
    }

    fn part1(&self) -> Vec<u64> {
        self.1
            .iter()
            .copied()
            .filter(|&v| self.0.iter().any(|&(a, b)| (a..=b).contains(&v)))
            .collect()
    }

    fn part2(&self) -> Vec<(u64, u64)> {
        self.0.iter().copied().fold(Vec::new(), |mut acc, a| {
            // Triangle inequality
            if let Some(b) = acc.last_mut()
                && let m = (b.0.min(a.0), b.1.max(a.1))
                && (m.1 - m.0 + 1) <= (b.1 - b.0 + 1) + (a.1 - a.0 + 1)
            {
                *b = m;
            } else {
                acc.push(a)
            }
            acc
        })
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("part1 = {}", t.part1().len());
        println!("part2 = {}", t.part2().iter().map(|r| r.1 - r.0 + 1).sum::<u64>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part1(), vec![5, 11, 17]);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part2(), vec![(3, 5), (10, 20)]);
    }
}
