use itertools::{iproduct, izip, Itertools};
use std::io::{stdin, BufRead};

#[derive(Debug)]
struct TestCase {
    locks: Vec<[usize; 5]>,
    keys: Vec<[usize; 5]>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let lines = reader.lines().map(|e| e.unwrap());
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for (_, c) in lines.chunk_by(|e| !e.is_empty()).into_iter().filter(|(k, _)| *k) {
            let rows: Vec<String> = c.collect();
            let h: [_; 5] = (0..5)
                .map(|x| rows.iter().filter(|r| r.chars().nth(x).unwrap() == '#').count() - 1)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            if rows[0] == "#####" {
                locks.push(h);
            } else {
                keys.push(h);
            }
        }
        Self { locks, keys }
    }

    fn solve(&self) -> usize {
        iproduct!(self.locks.iter(), self.keys.iter())
            .filter(|&(l, k)| izip!(l, k).all(|(a, b)| (a + b) <= 5))
            .count()
    }
}

fn main() {
    println!("{}", TestCase::parse(stdin().lock()).solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).solve(), 3);
    }
}
