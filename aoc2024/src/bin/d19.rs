use std::io::{stdin, BufRead};

struct TestCase {
    fragments: Vec<String>,
    patterns: Vec<String>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(|e| e.unwrap());
        let fragments = lines.next().unwrap().split(',').map(|s| s.trim().to_owned()).collect();
        let patterns = lines.skip(1).map(|s| s.to_owned()).collect();
        Self { fragments, patterns }
    }

    fn assemble(&self, pattern: &str) -> usize {
        // dp[i] = can assemble pat[..i]
        let mut dp = vec![0; pattern.len() + 1];
        dp[0] = 1;
        for i in 0..pattern.len() {
            for fragment in &self.fragments {
                if pattern[i..].starts_with(fragment) {
                    dp[i + fragment.len()] += dp[i];
                }
            }
        }
        dp[pattern.len()]
    }

    fn solve(&self) -> (usize, usize) {
        self.patterns
            .iter()
            .map(|pat| self.assemble(pat))
            .fold((0, 0), |acc, x| (acc.0 + if x > 0 { 1 } else { 0 }, acc.1 + x))
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
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).solve(), (6, 16));
    }
}
