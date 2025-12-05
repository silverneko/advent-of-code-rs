use clap::Args;
use itertools::iproduct;
use std::io::{BufRead, read_to_string, stdin};

use crate::grid::{Grid, Point};

/// Day 4: Printing Department
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase(Grid<char>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        Self(read_to_string(reader).unwrap().parse().unwrap())
    }

    fn solve(&self) -> impl Iterator<Item = Grid<char>> {
        std::iter::successors(Some(self.0.clone()), |st| {
            let mut res = st.clone();
            for p in st.indices().filter(|&p| st[p] == '@') {
                let c = iproduct!(-1..=1, -1..=1)
                    .map(Point::from)
                    .filter(|&d| matches!(st.get(p + d), Some(&'@')))
                    .count();
                if c < 5 {
                    res[p] = 'x';
                }
            }
            if res == *st { None } else { Some(res) }
        })
        .skip(1)
    }

    fn part1(&self) -> Grid<char> {
        self.solve().next().unwrap()
    }

    fn part2(&self) -> Grid<char> {
        self.solve().last().unwrap()
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        println!("part1 = {}", t.part1().iter().copied().filter(|&c| c == 'x').count());
        println!("part2 = {}", t.part2().iter().copied().filter(|&c| c == 'x').count());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        const EXPECTED: &str = r"
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
"
        .trim_ascii();
        let t = TestCase::parse(SAMPLE_INPUT);
        let res = t.part1();
        assert_eq!(
            res,
            EXPECTED.parse::<Grid<_>>().unwrap(),
            "Actual:\n{res}\nExpected:\n{EXPECTED}"
        );
    }

    #[test]
    fn test_part2() {
        const EXPECTED: &str = r"
..xx.xxxx.
xxx.x.x.xx
xxxxx.x.xx
x.xx@@..x.
xx.@@@@.xx
.xx@@@@@.x
.x.@.@.@@x
x.x@@.@@@x
.xx@@@@@x.
x.x.@@@.x.
"
        .trim_ascii();
        let t = TestCase::parse(SAMPLE_INPUT);
        let res = t.part2();
        assert_eq!(
            res,
            EXPECTED.parse::<Grid<_>>().unwrap(),
            "Actual:\n{res}\nExpected:\n{EXPECTED}"
        );
    }
}
