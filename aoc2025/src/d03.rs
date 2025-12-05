use clap::Args;
use std::io::{BufRead, stdin};

/// Day 3: Lobby
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase {
    banks: Vec<Vec<u64>>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let banks = reader
            .lines()
            .map(|s| s.unwrap())
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as _).collect())
            .collect();
        Self { banks }
    }

    fn solve_one(w: u32, seq: &[u64]) -> u64 {
        seq.iter().copied().fold(0, |acc, v| {
            acc.max(
                (0..w)
                    .map(|idx| {
                        let b = 10u64.pow(idx);
                        let lhs = acc - (acc % (b * 10));
                        let rhs = (acc % b) * 10 + v;
                        lhs + rhs
                    })
                    .max()
                    .unwrap(),
            )
        })
    }

    fn part1(&self) -> Vec<u64> {
        self.banks.iter().map(|b| Self::solve_one(2, b)).collect()
    }

    fn part2(&self) -> Vec<u64> {
        self.banks.iter().map(|b| Self::solve_one(12, b)).collect()
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("part1 = {}", t.part1().into_iter().sum::<u64>());
        println!("part2 = {}", t.part2().into_iter().sum::<u64>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
987654321111111
811111111111119
234234234234278
818181911112111
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part1(), vec![98, 89, 78, 92]);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part2(), vec![987654321111, 811111111119, 434234234278, 888911112111]);
    }
}
