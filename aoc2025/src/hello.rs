use clap::Args;
use std::io::{BufRead, read_to_string, stdin};

/// Day 999: Hello template
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase();

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let _ = read_to_string(reader).unwrap();
        Self()
    }
}

impl Main {
    pub fn run(&self) {
        let _t = TestCase::parse(stdin().lock());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = br"
"
    .trim_ascii();

    #[test]
    fn test_part1() {
        let _t = TestCase::parse(SAMPLE_INPUT);
    }

    #[test]
    fn test_part2() {
        let _t = TestCase::parse(SAMPLE_INPUT);
    }
}
