use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::ops::{BitAnd, BitOr, BitXor};
use std::str::{from_utf8, FromStr};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Gate {
    AND,
    OR,
    XOR,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "AND" => Ok(Gate::AND),
            "OR" => Ok(Gate::OR),
            "XOR" => Ok(Gate::XOR),
            _ => Err(()),
        }
    }
}

impl Gate {
    fn call<T>(&self, a: T, b: T) -> T
    where
        T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
    {
        match self {
            Self::AND => a & b,
            Self::OR => a | b,
            Self::XOR => a ^ b,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Wire([u8; 3]);

impl Wire {
    fn of(p: char, n: usize) -> Self {
        format!("{}{:02}", p, n).parse().unwrap()
    }

    fn port(&self) -> Option<(u8, usize)> {
        Some((self.0[0], from_utf8(&self.0[1..]).ok()?.parse().ok()?))
    }
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        assert_eq!(s.len(), 3);
        Ok(Self(s.as_bytes().try_into().unwrap()))
    }
}

impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.0).unwrap())
    }
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TestCase {
    state: HashMap<Wire, u8>,
    gates: HashMap<Wire, (Gate, Wire, Wire)>,
    max_x: usize,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(|e| e.unwrap());
        let state = lines
            .by_ref()
            .take_while(|e| !e.is_empty())
            .map(|e| {
                let (w, s) = e.split_once(": ").unwrap();
                (w.parse().unwrap(), s.parse().unwrap())
            })
            .collect();
        let gates: HashMap<Wire, (Gate, Wire, Wire)> = lines
            .map(|e| {
                let t: Vec<_> = e.split_whitespace().collect();
                (
                    t[4].parse().unwrap(),
                    (t[1].parse().unwrap(), t[0].parse().unwrap(), t[2].parse().unwrap()),
                )
            })
            .collect();
        let max_x = gates
            .values()
            .flat_map(|(_, w1, w2)| [w1, w2])
            .filter_map(|w| match w.port() {
                Some((b'x' | b'y', n)) => Some(n),
                _ => None,
            })
            .max()
            .unwrap();
        Self { state, gates, max_x }
    }

    fn get(&mut self, w: &Wire) -> u8 {
        if let Some(&v) = self.state.get(w) {
            return v;
        }
        // Insert junk 0 to avoid loops.
        self.state.insert(*w, 0);
        let v = if let Some(&(gate, w1, w2)) = self.gates.get(w) {
            gate.call(self.get(&w1), self.get(&w2))
        } else {
            0
        };
        self.state.insert(*w, v);
        v
    }

    fn part1(&mut self) -> u64 {
        let mut ans = 0;
        for w in self.gates.clone().keys() {
            if let Some((b'z', n)) = w.port() {
                ans |= (self.get(w) as u64) << n;
            }
        }
        ans
    }

    fn swap(&mut self, a: &Wire, b: &Wire) {
        let pa = self.gates.get_mut(a).unwrap() as *mut _;
        let pb = self.gates.get_mut(b).unwrap() as *mut _;
        unsafe { std::ptr::swap(pa, pb) }
    }

    fn score(&mut self) -> usize {
        let mut score = 0;
        for i in 0..=self.max_x {
            let mut add = 0;
            let mut carry = 0;
            for c in [0, 1] {
                if c == 1 && i == 0 {
                    continue;
                }
                for x in [0, 1] {
                    for y in [0, 1] {
                        self.state = HashMap::from([(Wire::of('x', i), x), (Wire::of('y', i), y)]);
                        if c == 1 {
                            self.state
                                .extend([(Wire::of('x', i - 1), 1), (Wire::of('y', i - 1), 1)]);
                        }
                        if self.get(&Wire::of('z', i)) != x ^ y ^ c {
                            add = 1;
                        }
                        if self.get(&Wire::of('z', i + 1)) != ((x & y) | (y & c) | (x & c)) {
                            carry = 1;
                        }
                    }
                }
            }
            score += add + carry;
        }
        score
    }

    fn find_best_swap(&self) -> (usize, Wire, Wire) {
        let wires = Arc::new(Vec::from_iter(self.gates.keys().copied()));
        let work_queue: Vec<_> = {
            let l = self.gates.len();
            iproduct!(0..l, 0..l).filter(|(i, j)| i < j).collect()
        };
        let results = Arc::new(Mutex::new(Vec::with_capacity(work_queue.len())));
        let work_queue = Arc::new(Mutex::new(work_queue));

        let mut thread_handlers = Vec::new();
        for _ in 0..70 {
            let work_queue = Arc::clone(&work_queue);
            let results = Arc::clone(&results);
            let wires = Arc::clone(&wires);
            let mut data = self.clone();
            thread_handlers.push(thread::spawn(move || loop {
                let Some((i, j)) = work_queue.lock().unwrap().pop() else { break };
                let (w1, w2) = (wires[i], wires[j]);
                data.swap(&w1, &w2);
                let s = data.score();
                data.swap(&w1, &w2);
                results.lock().unwrap().push((s, w1, w2));
            }));
        }
        for t in thread_handlers {
            t.join().unwrap();
        }

        let r = *results.lock().unwrap().iter().min().unwrap();
        r
    }

    fn part2(&mut self) -> String {
        let mut ans = Vec::new();
        loop {
            let score = self.score();
            if score == 0 {
                break;
            }
            let (s, w1, w2) = self.find_best_swap();
            println!(
                "{score:3} -> {s:3}\t{:?}\t{:?}",
                self.gates.get_key_value(&w1).unwrap(),
                self.gates.get_key_value(&w2).unwrap()
            );
            self.swap(&w1, &w2);
            ans.extend([w1, w2]);
        }
        ans.sort();
        ans.into_iter().join(",")
    }
}

fn main() {
    let mut data = TestCase::parse(stdin().lock());
    println!("{}", data.part1());
    println!("{}", data.part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_small() {
        let input = r"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).part1(), 4);
    }

    #[test]
    fn test_sample_large() {
        let input = r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"
        .trim();
        assert_eq!(TestCase::parse(input.as_bytes()).part1(), 2024);
    }
}
