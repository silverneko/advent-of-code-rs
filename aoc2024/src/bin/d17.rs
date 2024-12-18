use itertools::Itertools;
use std::io::{stdin, BufRead};

#[derive(Debug, Clone)]
struct State {
    pos: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    code: Vec<usize>,
}

impl State {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader
            .lines()
            .filter_map(|e| e.unwrap().split_whitespace().last().map(|s| s.to_owned()));
        let reg_a = lines.next().unwrap().parse::<u64>().unwrap();
        let reg_b = lines.next().unwrap().parse::<u64>().unwrap();
        let reg_c = lines.next().unwrap().parse::<u64>().unwrap();
        let code: Vec<usize> =
            lines.next().unwrap().split(',').map(|e| e.parse().unwrap()).collect();
        Self { pos: 0, reg_a, reg_b, reg_c, code }
    }

    fn opcode(&self) -> Option<usize> {
        if self.pos < self.code.len() {
            Some(self.code[self.pos])
        } else {
            None
        }
    }

    fn operand(&self) -> usize {
        self.code[self.pos + 1]
    }

    fn lit(&self) -> u64 {
        self.operand() as u64
    }

    fn combo(&self) -> u64 {
        match self.operand() {
            lit @ 0..=3 => lit as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            t => panic!("unexpected combo operand {t}"),
        }
    }
}

fn solve(mut state: State) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    while let Some(opcode) = state.opcode() {
        match opcode {
            0 => state.reg_a >>= state.combo(),
            1 => state.reg_b ^= state.lit(),
            2 => state.reg_b = state.combo() % 8,
            3 => {
                if state.reg_a != 0 {
                    state.pos = state.lit() as usize;
                    continue;
                }
            }
            4 => state.reg_b ^= state.reg_c,
            5 => output.push((state.combo() % 8) as usize),
            6 => state.reg_b = state.reg_a >> state.combo(),
            7 => state.reg_c = state.reg_a >> state.combo(),
            t => panic!("unexpected opcode {t}"),
        }
        state.pos += 2;
    }
    output
}

/*
Program:
2,4 B=A % 8
1,2 B=B ^ 2
7,5 C=A >> B
1,3 B=B ^ 3
4,4 B=B ^ C
5,5 O=B % 8
0,3 A=A >> 3
3,0 loop if A != 0
 */

fn dfs(s: &State, reg_a: u64) -> Option<u64> {
    let output = solve(State { reg_a, ..s.clone() });
    if output == s.code {
        return Some(reg_a);
    }
    if reg_a == 0 || s.code.ends_with(&output) {
        println!("{reg_a} {output:?}");
        for i in 0..=7 {
            let reg_a = reg_a << 3 | i;
            if reg_a > 0 {
                if let Some(a) = dfs(s, reg_a) {
                    return Some(a);
                }
            }
        }
    }
    None
}

fn main() {
    let state = State::parse(stdin().lock());
    println!("{state:?}");
    println!("{}", solve(state.clone()).iter().join(","));

    let reg_a = dfs(&state, 0).unwrap();
    let output = solve(State { reg_a, ..state.clone() });
    println!("{reg_a} {output:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sample() {
        let input = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        .trim();
        let reader = Cursor::new(input);
        assert_eq!(solve(State::parse(reader)).iter().join(","), "4,6,3,5,6,3,5,2,1,0");
    }
}
