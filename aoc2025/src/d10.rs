use clap::Args;
use itertools::Itertools;
use std::collections::HashMap;
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
    /// Generates all possible button combinations, grouped by the end state that it produces.
    fn generate_states(&self) -> HashMap<u32, Vec<Vec<u32>>> {
        self.buttons.iter().copied().powerset().fold(HashMap::new(), |mut hash_map, seq| {
            let state = seq.iter().copied().fold(0, |acc, b| acc ^ b);
            if let Some(v) = hash_map.get_mut(&state) {
                v.push(seq);
                v.sort_by(|a, b| a.len().cmp(&b.len()));
            } else {
                hash_map.insert(state, vec![seq]);
            }
            hash_map
        })
    }
}

fn solve_joltage(state_map: &HashMap<u32, Vec<Vec<u32>>>, jolt: &[i32]) -> Option<usize> {
    if jolt.iter().all(|&e| e == 0) {
        return Some(0);
    }
    let state = jolt
        .iter()
        .copied()
        .enumerate()
        .fold(0, |acc, (idx, j)| if j & 1 == 1 { acc | 1 << idx } else { acc });
    state_map
        .get(&state)?
        .iter()
        .filter_map(|seq| {
            let mut jolt = jolt.to_owned();
            for idx in 0..jolt.len() {
                jolt[idx] -= seq.iter().copied().filter(|b| b & (1 << idx) != 0).count() as i32;
                assert!(jolt[idx] & 1 == 0);
                jolt[idx] /= 2;
            }
            if jolt.iter().all(|&j| j >= 0) {
                Some(seq.len() + 2 * solve_joltage(state_map, &jolt)?)
            } else {
                None
            }
        })
        .min()
}

#[derive(Debug)]
struct TestCase(Vec<MachineConfig>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        Self(reader.lines().map(|l| l.unwrap().parse().unwrap()).collect())
    }

    fn part1(&self) -> Vec<Vec<u32>> {
        self.0.iter().map(|v| v.generate_states()[&v.state].first().unwrap().clone()).collect()
    }

    fn part2(&self) -> Vec<usize> {
        self.0.iter().map(|v| solve_joltage(&v.generate_states(), &v.joltage).unwrap()).collect()
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
        assert_eq!(
            t.part1(),
            vec![vec![0b1010, 0b1100], vec![0b10001, 0b111, 0b11110], vec![0b11001, 0b110111]]
        );
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(SAMPLE_INPUT);
        assert_eq!(t.part2(), vec![10, 12, 11]);
    }
}
