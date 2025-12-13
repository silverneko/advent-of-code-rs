use clap::Args;
use itertools::Itertools;
use std::convert::Infallible;
use std::io::{BufRead, stdin};
use std::str::FromStr;

/// Day 10: Factory
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct MachineConfig {
    state: u32,
    buttons: Vec<u32>,
    joltage: Vec<i32>,
}

impl FromStr for MachineConfig {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.split_whitespace().collect::<Vec<_>>();
        let [state, buttons @ .., jolt] = l.as_slice() else { panic!() };
        let state =
            state.trim_matches(['[', ']']).chars().enumerate().fold(0, |acc, (idx, c)| match c {
                '.' => acc,
                '#' => acc | (1 << idx),
                _ => panic!(),
            });
        let buttons = buttons
            .iter()
            .map(|s| {
                s.trim_matches(['(', ')'])
                    .split(',')
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .fold(0, |acc, b| acc | (1 << b))
            })
            .collect();
        let joltage =
            jolt.trim_matches(['{', '}']).split(',').map(str::parse).map(Result::unwrap).collect();
        Ok(Self { state, buttons, joltage })
    }
}

impl MachineConfig {
    fn find_solutions(&self, state: u32) -> impl Iterator<Item = Vec<usize>> {
        // NOTE: Assuming Itertools::powerset() generates sequence from small to large set size.
        (0..self.buttons.len()).powerset().filter(move |seq| {
            seq.iter().copied().fold(0, |acc, idx| acc ^ self.buttons[idx]) == state
        })
    }

    fn find_joltage_solution(&self, jolt: &[i32]) -> Option<usize> {
        if jolt.iter().all(|&e| e == 0) {
            return Some(0);
        }
        let state = jolt
            .iter()
            .copied()
            .enumerate()
            .fold(0, |acc, (idx, j)| if j & 1 == 1 { acc | 1 << idx } else { acc });
        self.find_solutions(state)
            .filter_map(|seq| {
                let mut jolt = jolt.to_owned();
                for (i, j) in jolt.iter_mut().enumerate() {
                    for b in seq.iter().map(|&s| self.buttons[s]) {
                        if b & (1 << i) != 0 {
                            *j -= 1;
                        }
                    }
                    assert!(*j & 1 == 0);
                    *j /= 2;
                }
                if jolt.iter().all(|&j| j >= 0)
                    && let Some(jj) = self.find_joltage_solution(&jolt)
                {
                    Some(seq.len() + 2 * jj)
                } else {
                    None
                }
            })
            .min()
    }
}

#[derive(Debug)]
struct TestCase(Vec<MachineConfig>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        Self(reader.lines().map(|l| l.unwrap().parse().unwrap()).collect())
    }

    fn part1(&self) -> Vec<Vec<usize>> {
        self.0.iter().map(|v| v.find_solutions(v.state).next().unwrap()).collect()
    }

    fn part2(&self) -> Vec<usize> {
        self.0.iter().map(|v| v.find_joltage_solution(&v.joltage).unwrap()).collect()
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        dbg!(t.part1().iter().map(Vec::len).sum::<usize>());
        dbg!(t.part2().iter().sum::<usize>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part1(), vec![vec![1, 3], vec![2, 3, 4], vec![1, 2]]);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part2(), vec![10, 12, 11]);
    }
}
