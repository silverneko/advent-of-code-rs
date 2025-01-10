use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::iter::successors;

struct TestCase {
    orbits: HashMap<String, String>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let orbits = reader
            .lines()
            .map(|e| e.unwrap().split_once(')').map(|(a, b)| (b.to_owned(), a.to_owned())).unwrap())
            .collect();
        Self { orbits }
    }

    fn dfs<'a>(&'a self, u: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(&v) = memo.get(&u) {
            v
        } else {
            let v = 1 + self.dfs(&self.orbits[u], memo);
            memo.insert(u, v);
            v
        }
    }

    fn part1(&self) -> usize {
        let mut memo = HashMap::from([("COM", 0)]);
        self.orbits.keys().map(|u| self.dfs(u, &mut memo)).sum()
    }

    fn part2(&self) -> usize {
        let you: Vec<_> =
            successors(Some("YOU"), |&u| self.orbits.get(u).map(|s| s.as_str())).collect();
        let san: Vec<_> =
            successors(Some("SAN"), |&u| self.orbits.get(u).map(|s| s.as_str())).collect();
        let ca = you.iter().rev().zip(san.iter().rev()).position(|(a, b)| a != b).unwrap();
        you.len() + san.len() - 2 * ca - 2
    }
}

fn main() {
    let data = TestCase::parse(stdin().lock());
    dbg!(data.part1(), data.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"
        .trim_ascii();
        assert_eq!(TestCase::parse(input.as_bytes()).part1(), 42);
    }

    #[test]
    fn test_part2() {
        let input = r"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
"
        .trim_ascii();
        assert_eq!(TestCase::parse(input.as_bytes()).part2(), 4);
    }
}
