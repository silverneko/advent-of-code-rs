use clap::Args;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};

/// Day 11: Reactor
#[derive(Args)]
pub struct Main {}

#[derive(Debug)]
struct TestCase(HashMap<String, Vec<String>>);

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        Self(
            reader
                .lines()
                .map(Result::unwrap)
                .map(|l| {
                    let (k, v) = l.split_once(':').unwrap();
                    (k.to_owned(), v.split_whitespace().map(|s| s.to_owned()).collect())
                })
                .collect(),
        )
    }

    fn count_paths(&self, from: &str, to: &str) -> usize {
        let mut queue = VecDeque::from([from]);
        let mut cache = HashMap::from([(from, 1)]);
        let mut ans = 0;
        while let Some(v) = queue.pop_front() {
            let w = cache.remove(v).unwrap();
            if v == to {
                ans += w;
            } else if let Some(edges) = self.0.get(v) {
                for u in edges {
                    let u = u.as_str();
                    if let Some(c) = cache.get_mut(u) {
                        *c += w;
                    } else {
                        queue.push_back(u);
                        cache.insert(u, w);
                    }
                }
            }
        }
        ans
    }

    fn part1(&self) -> usize {
        self.count_paths("you", "out")
    }

    fn part2(&self) -> usize {
        self.count_paths("svr", "dac")
            * self.count_paths("dac", "fft")
            * self.count_paths("fft", "out")
            + self.count_paths("svr", "fft")
                * self.count_paths("fft", "dac")
                * self.count_paths("dac", "out")
    }
}

impl Main {
    pub fn run(&self) {
        let t = TestCase::parse(stdin().lock());
        dbg!(t.part1(), t.part2());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let t = TestCase::parse(
            b"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
            .trim_ascii(),
        );
        assert_eq!(t.part1(), 5);
    }

    #[test]
    fn test_part2() {
        let t = TestCase::parse(
            b"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
            .trim_ascii(),
        );
        assert_eq!(t.part2(), 2);
    }
}
