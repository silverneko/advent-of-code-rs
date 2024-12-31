use anyhow::{Context, Result};
use itertools::{izip, Itertools};
use std::collections::HashMap;
use std::io::{stdin, BufRead};

struct TestCase {
    strs: Vec<String>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Result<Self> {
        let strs = reader.lines().collect::<Result<_, _>>()?;
        Ok(Self { strs })
    }

    fn solve(&self) -> Result<usize> {
        let cc: Vec<HashMap<_, _>> = self.strs.iter().map(|s| s.chars().counts()).collect();
        let c2 = cc.iter().filter(|e| e.values().any(|&c| c == 2)).count();
        let c3 = cc.iter().filter(|e| e.values().any(|&c| c == 3)).count();
        Ok(c2 * c3)
    }

    fn solve2(&self) -> Result<String> {
        let (s1, s2) = self
            .strs
            .iter()
            .tuple_combinations()
            .find(|(s1, s2)| {
                izip!(s1.chars(), s2.chars()).map(|(c1, c2)| (c1 != c2) as u32).sum::<u32>() == 1
            })
            .context("find edit distance 1 pair")?;
        Ok(izip!(s1.chars(), s2.chars()).filter(|(c1, c2)| c1 == c2).map(|c| c.0).collect())
    }
}

fn main() -> Result<()> {
    let data = TestCase::parse(stdin().lock())?;
    println!("{:?}", data.solve());
    println!("{:?}", data.solve2());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() -> Result<()> {
        let input = r"
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes())?.solve()?, 12);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = r"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes())?.solve2()?, "fgij");
        Ok(())
    }
}
