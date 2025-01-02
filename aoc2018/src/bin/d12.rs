use itertools::Itertools;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestCase {
    state: HashSet<i64>,
    rules: HashSet<String>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(|e| e.unwrap());
        let state = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if c == '#' { Some(idx as i64) } else { None })
            .collect();
        let rules = lines
            .skip(1)
            .filter_map(|e| {
                let (mask, b) = e.split_once(" => ")?;
                if b == "#" {
                    Some(mask.to_owned())
                } else {
                    None
                }
            })
            .collect();
        Self { state, rules }
    }

    fn step(&mut self) {
        let (lb, rb) = self.state.iter().copied().minmax().into_option().unwrap();
        let (lb, rb) = (lb - 4, rb + 4);
        let next_state = (lb..=rb)
            .map(|k| if self.state.contains(&k) { (k, '#') } else { (k, '.') })
            .collect::<Vec<_>>()
            .windows(5)
            .filter_map(|v| {
                let (i, c): (Vec<i64>, String) = v.iter().copied().unzip();
                if self.rules.contains(&c) {
                    Some(i[2])
                } else {
                    None
                }
            })
            .collect();
        self.state = next_state;
    }

    fn solve(&self) -> (i64, i64) {
        let sum: Vec<_> = (0..200)
            .scan(self.clone(), |state, _| {
                let s = state.state.iter().sum();
                state.step();
                Some(s)
            })
            .collect();
        let dsum: Vec<_> = sum.windows(2).map(|a| a[1] - a[0]).collect();
        let ddsum: Vec<_> = dsum.windows(2).map(|a| a[1] - a[0]).collect();
        println!("sum:\n{sum:?}");
        println!("derivative:\n{dsum:?}");
        println!("2nd derivative:\n{ddsum:?}");
        let p = ddsum.windows(5).position(|w| w.iter().all(|&e| e == 0)).unwrap();
        let d = dsum[p];
        macro_rules! test_equation {
            ($x:literal) => { assert_eq!(sum[p + $x], sum[p] + d * $x) };
            ($x:literal $($xs:literal)+) => { test_equation!{$x}; test_equation!{$($xs)+} };
        }
        test_equation![1 2 3 4 5 6 7 8];
        println!();
        println!("Sequence converges at generation {p}, step size {d}");
        println!("S[k] = S[{p}] + {d} x (k - {p}) for k >= {p}");
        println!();
        (sum[20], sum[p] + d * (500_0000_0000 - p as i64))
    }
}

fn main() {
    println!("{:?}", TestCase::parse(stdin().lock()).solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).solve().0, 325);
    }
}
