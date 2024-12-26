use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::iter::zip;

struct TestCase {
    nums: Vec<usize>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let nums = reader.lines().map(|e| e.unwrap().parse().unwrap()).collect();
        Self { nums }
    }

    fn pack5(nums: &[isize]) -> usize {
        nums.iter().for_each(|&n| assert!((-16..16).contains(&n)));
        nums.iter().fold(0, |acc, &n| (n as usize % 32) | acc << 5)
    }

    fn step(a: usize) -> usize {
        let a = ((a << 6) ^ a) % 16777216;
        let a = ((a >> 5) ^ a) % 16777216;
        ((a << 11) ^ a) % 16777216
    }

    fn solve(&self) -> (usize, usize) {
        // Generate all raw prices.
        let generated_nums: Vec<Vec<usize>> = self
            .nums
            .iter()
            .map(|&n| {
                let mut state = n;
                std::iter::repeat_with(move || {
                    state = Self::step(state);
                    state
                })
                .take(2000)
                .collect()
            })
            .collect();
        let ans1 = generated_nums.iter().map(|v| v.last().unwrap()).sum();

        // A table is a collection of memos.
        // A memo is a mapping from packed 4-seq to price.
        let table: Vec<HashMap<usize, usize>> = generated_nums
            .iter()
            .map(|g| {
                let prices: Vec<_> = g.iter().map(|e| e % 10).collect();
                let deltas: Vec<isize> =
                    prices.windows(2).map(|a| a[1] as isize - a[0] as isize).collect();
                let mut memo = HashMap::new();
                for (p, seq) in zip(prices.iter().skip(4).copied(), deltas.windows(4)) {
                    memo.entry(Self::pack5(seq)).or_insert(p);
                }
                memo
            })
            .collect();
        // Enumerate each candidate 4-seq and find the max sum of prices by table lookup.
        let candidates = table.iter().flat_map(|t| t.keys().copied()).sorted().dedup();
        let ans2 = candidates
            .map(|k| table.iter().map(|m| m.get(&k).copied().unwrap_or(0)).sum())
            .max()
            .unwrap();

        (ans1, ans2)
    }
}

fn main() {
    println!("{:?}", TestCase::parse(stdin().lock()).solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(TestCase::parse("1".as_bytes()).solve().0, 8685429);
    }

    #[test]
    fn test_10() {
        assert_eq!(TestCase::parse("10".as_bytes()).solve().0, 4700978);
    }

    #[test]
    fn test_100() {
        assert_eq!(TestCase::parse("100".as_bytes()).solve().0, 15273692);
    }

    #[test]
    fn test_2024() {
        assert_eq!(TestCase::parse("2024".as_bytes()).solve().0, 8667524);
    }

    #[test]
    fn test_part2() {
        let input = r"
1
2
3
2024
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).solve().1, 23);
    }
}
