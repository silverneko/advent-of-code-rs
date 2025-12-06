use clap::Args;
use itertools::Itertools;
use std::io::{BufRead, stdin};

/// Day 6: Trash Compactor
#[derive(Args)]
pub struct Main {}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!(),
        }
    }
}

impl Op {
    fn apply(&self, args: impl IntoIterator<Item = u64>) -> u64 {
        match self {
            Self::Add => args.into_iter().sum(),
            Self::Mul => args.into_iter().product(),
        }
    }
}

fn transpose<T>(
    iters: impl Iterator<Item = impl IntoIterator<Item = T>>,
) -> impl Iterator<Item = Vec<T>> {
    let mut args = iters.map(IntoIterator::into_iter).collect::<Vec<_>>();
    std::iter::from_fn(move || args.iter_mut().map(Iterator::next).collect::<Option<Vec<_>>>())
}

fn collect_number(it: impl Iterator<Item = char>) -> u64 {
    it.collect::<String>().trim().parse().unwrap()
}

#[derive(Debug)]
struct TestCase(Vec<(Op, Vec<Vec<char>>)>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();
        let ops = lines.pop().unwrap().split_whitespace().map(Op::from).collect::<Vec<_>>();
        let args_t = transpose(lines.iter().map(|l| l.chars()))
            .chunk_by(|v| v.iter().all(|&c| c == ' '))
            .into_iter()
            .filter_map(|(blank, vv)| if blank { None } else { Some(vv.collect()) })
            .collect::<Vec<_>>();
        Self(std::iter::zip(ops, args_t).collect())
    }

    fn part1(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|(op, vv)| {
                op.apply(transpose(vv.clone().into_iter()).map(|v| collect_number(v.into_iter())))
            })
            .collect()
    }

    fn part2(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|(op, vv)| op.apply(vv.iter().map(|v| collect_number(v.iter().copied()))))
            .collect()
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("part1 = {}", t.part1().iter().sum::<u64>());
        println!("part2 = {}", t.part2().iter().sum::<u64>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
    .trim_ascii_start();

    #[test]
    fn test_part1() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part1(), vec![33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part2(), vec![8544, 625, 3253600, 1058]);
    }
}
