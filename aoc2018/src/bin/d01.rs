use anyhow::{Context, Result};
use itertools::Itertools;
use std::io::{stdin, BufRead};

struct TestCase {
    nums: Vec<isize>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Result<Self> {
        let nums = reader.lines().map(|e| Ok(e?.parse()?)).collect::<Result<_>>()?;
        Ok(Self { nums })
    }

    fn solve(&self) -> Result<(isize, isize)> {
        let dup = self
            .nums
            .iter()
            .cycle()
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .duplicates()
            .next()
            .context("first dup element")?;
        Ok((self.nums.iter().sum(), dup))
    }
}

fn main() -> Result<()> {
    println!("{:?}", TestCase::parse(stdin().lock())?.solve());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() -> Result<()> {
        let input = r"
+1
-2
+3
+1
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes())?.solve()?, (3, 2));
        Ok(())
    }
}
