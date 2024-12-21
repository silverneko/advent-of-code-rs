use itertools::chain;
use std::collections::HashMap;
use std::io::stdin;
use std::iter::{once, repeat, zip};
use utils::Point;

// Only the relative position w.r.t. 'A' is important.
fn keypad_pos(d: char) -> Point {
    match d {
        '7' => Point(0, 0),
        '8' => Point(0, 1),
        '9' => Point(0, 2),
        '4' => Point(1, 0),
        '5' => Point(1, 1),
        '6' => Point(1, 2),
        '1' => Point(2, 0),
        '2' => Point(2, 1),
        '3' => Point(2, 2),
        '0' | '^' => Point(3, 1),
        'A' => Point(3, 2),
        '<' => Point(4, 0),
        'v' => Point(4, 1),
        '>' => Point(4, 2),
        _ => panic!("illegal digit {d}"),
    }
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

[Change direction as less as possible] because expensive.

[Greedily go left] because the '<' key position is the furthest away.

         <      ^   A
  v <<   A ^  > A > A
v<A<AA>>^A<Av>A^AvA^A

       ^        <       A
   <   A  v <   A >>  ^ A
v<<A>>^Av<A<A>>^AvAA<^A>A

[Greedily go down] because the 'v' key position is the second furthest.

     >       v      A
  v  A   <   A >  ^ A
v<A>^Av<<A>>^AvA<^A>A

        v   >   A
  v <   A > A ^ A
v<A<A>>^AvA^A<A>A

[Greedily _don't_ go right / greedily go up] because we rather go '^' and 'v' than '>' and '<',
because left is more expensive than down.

               >                    ^         A
        v      A         <      ^   A     >   A
   < v  A ^  > A  v <<   A >  ^ A > A  v  A ^ A
v<<A>A>^A<Av>A^A<vA<AA>>^AvA<^A>AvA^A<vA>^A<A>A

                 ^                >          A
         <       A        v   >   A      ^   A
  v <<   A >>  ^ A   < v  A > A ^ A   <  A > A
<vA<AA^>>AvAA<^A>Av<<A>A^>AvA^A<A>Av<<A>>AvA^A
 */
fn expand_steps(a: char, b: char) -> Vec<char> {
    let Point(mut y, mut x) = keypad_pos(b) - keypad_pos(a);
    let mut keys = Vec::new();
    #[allow(clippy::collapsible_if)]
    if x < 0 {
        if (a == 'A' && x == -1) || !matches!(a, 'A' | '0' | '^') {
            keys.extend(vec!['<'; -x as usize]);
            x = 0;
        }
    }
    #[allow(clippy::collapsible_if)]
    if y > 0 {
        if (a == '4' && y == 1) || (a == '7' && y <= 2) || !matches!(a, '1' | '4' | '7') {
            keys.extend(vec!['v'; y as usize]);
            y = 0;
        }
    }
    #[allow(clippy::collapsible_if)]
    if y < 0 {
        if a != '<' {
            keys.extend(vec!['^'; -y as usize]);
            y = 0;
        }
    }
    if x > 0 {
        keys.extend(vec!['>'; x as usize]);
    }
    if y > 0 {
        keys.extend(vec!['v'; y as usize]);
    } else {
        keys.extend(vec!['^'; -y as usize]);
    }
    if x < 0 {
        keys.extend(vec!['<'; -x as usize]);
    }
    keys
}

struct TestCase {
    num: usize,
    initial_state: HashMap<(char, char), usize>,
}

impl TestCase {
    fn parse(s: &str) -> TestCase {
        assert!(s.ends_with("A"));
        TestCase {
            num: s.trim_end_matches('A').parse().unwrap(),
            initial_state: zip(chain![once('A'), s.chars()], s.chars()).zip(repeat(1)).collect(),
        }
    }

    fn solve(&self, chain: usize) -> (usize, usize) {
        let end_state = (0..=chain).fold(self.initial_state.clone(), |s, _| {
            let mut next_state = HashMap::new();
            for ((a, b), w) in s {
                let pv = expand_steps(a, b);
                for p in zip(chain![once('A'), pv.clone()], chain![pv, once('A')]) {
                    *next_state.entry(p).or_default() += w;
                }
            }
            next_state
        });
        (end_state.into_values().sum(), self.num)
    }
}

fn main() {
    let data: Vec<TestCase> = stdin().lines().map(|e| TestCase::parse(&e.unwrap())).collect();
    println!("{}", data.iter().map(|e| e.solve(2)).map(|(a, b)| a * b).sum::<usize>());
    println!("{}", data.iter().map(|e| e.solve(25)).map(|(a, b)| a * b).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_029() {
        assert_eq!(TestCase::parse("029A").solve(2), (68, 29));
    }

    #[test]
    fn test_sample_980() {
        assert_eq!(TestCase::parse("980A").solve(2), (60, 980));
    }

    #[test]
    fn test_sample_179() {
        assert_eq!(TestCase::parse("179A").solve(2), (68, 179));
    }

    #[test]
    fn test_sample_456() {
        assert_eq!(TestCase::parse("456A").solve(2), (64, 456));
    }

    #[test]
    fn test_sample_379() {
        assert_eq!(TestCase::parse("379A").solve(2), (64, 379));
    }
}
