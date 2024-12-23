use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdin, BufRead};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Node(u8, u8);

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&[self.0, self.1]).unwrap())
    }
}

type Graph = HashMap<Node, Vec<Node>>;

struct TestCase {
    graph: Graph,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut graph = Graph::new();
        for line in reader.lines() {
            let s = line.unwrap();
            let s = s.as_bytes();
            assert_eq!(s.len(), 5);
            assert_eq!(s[2], b'-');
            assert_ne!(s[0..=1], s[3..=4]);
            let v = Node(s[0], s[1]);
            let u = Node(s[3], s[4]);
            if v < u {
                graph.entry(v).or_default().push(u);
            } else {
                graph.entry(u).or_default().push(v);
            }
        }
        for (_, v) in graph.iter_mut() {
            v.sort();
        }
        Self { graph }
    }

    fn can_add_to_strong_set(&self, u: &Node, ss: &[Node]) -> bool {
        ss.iter().all(|v| self.graph[v].contains(u))
    }

    fn find_strong_sets(&self) -> Vec<Vec<Vec<Node>>> {
        let Self { ref graph } = self;
        let mut strong_sets: Vec<Vec<Vec<Node>>> =
            vec![graph.keys().sorted().map(|e| vec![*e]).collect()];
        loop {
            let mut stronger_sets = Vec::new();
            for s in strong_sets.last().unwrap().iter() {
                let v = s.last().unwrap();
                if graph.contains_key(v) {
                    for u in graph[v].iter() {
                        if self.can_add_to_strong_set(u, s) {
                            let mut stronger = s.clone();
                            stronger.push(*u);
                            stronger_sets.push(stronger);
                        }
                    }
                }
            }
            if stronger_sets.is_empty() {
                break;
            }
            strong_sets.push(stronger_sets);
        }
        strong_sets
    }
}

fn main() {
    let strong_sets = TestCase::parse(stdin().lock()).find_strong_sets();
    let triples = &strong_sets[2];
    let strongest = strong_sets.last().unwrap();
    println!("{}", triples.iter().filter(|s| s.iter().any(|b| b.0 == b't')).count());
    println!("{}", strongest[0].iter().join(","));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        .trim();
        let strong_sets = TestCase::parse(input.as_bytes()).find_strong_sets();
        let triples = &strong_sets[2];
        assert_eq!(
            triples.iter().map(|s| s.iter().join(",")).join("\n"),
            r"
aq,cg,yn
aq,vc,wq
co,de,ka
co,de,ta
co,ka,ta
de,ka,ta
kh,qp,ub
qp,td,wh
tb,vc,wq
tc,td,wh
td,wh,yn
ub,vc,wq
"
            .trim()
        );
        let strongest = strong_sets.last().unwrap();
        assert_eq!(strongest.len(), 1);
        assert_eq!(strongest[0].iter().join(","), "co,de,ka,ta");
    }
}
