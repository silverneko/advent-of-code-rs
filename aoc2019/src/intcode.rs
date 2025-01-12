use std::cell::RefCell;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::mpsc;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct Intcode {
    pub code: Vec<isize>,

    pc: usize,
    ra: isize,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Param {
    Imme(isize),
    Addr(isize),
    Rela(isize),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Instruction {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Input(Param),
    Output(Param),
    Jit(Param, Param),
    Jif(Param, Param),
    Tlt(Param, Param, Param),
    Teq(Param, Param, Param),
    Rela(Param),
    Halt,
}

impl Instruction {
    fn parse(code: &[isize]) -> Self {
        let mut params = code.iter().skip(1).copied().scan(code[0] / 100, |st, p| {
            let param = match *st % 10 {
                0 => Param::Addr(p),
                1 => Param::Imme(p),
                2 => Param::Rela(p),
                m => panic!("Unexpected parameter mode {m}"),
            };
            *st /= 10;
            Some(param)
        });
        let mut param = || params.next().unwrap();
        match code[0] % 100 {
            1 => Self::Add(param(), param(), param()),
            2 => Self::Mul(param(), param(), param()),
            3 => Self::Input(param()),
            4 => Self::Output(param()),
            5 => Self::Jit(param(), param()),
            6 => Self::Jif(param(), param()),
            7 => Self::Tlt(param(), param(), param()),
            8 => Self::Teq(param(), param(), param()),
            9 => Self::Rela(param()),
            99 => Self::Halt,
            o => panic!("Unexpected opcode {o}"),
        }
    }

    fn width(&self) -> usize {
        match self {
            Self::Add(..) | Self::Mul(..) | Self::Tlt(..) | Self::Teq(..) => 4,
            Self::Jit(..) | Self::Jif(..) => 3,
            Self::Input(..) | Self::Output(..) | Self::Rela(..) => 2,
            Self::Halt => 1,
        }
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.split(',').map(|e| e.trim().parse()).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { code, ..Default::default() })
    }
}

impl Intcode {
    pub fn parse(s: &str) -> Self {
        s.parse().unwrap()
    }

    fn get(&self, p: Param) -> isize {
        match p {
            Param::Imme(a) => a,
            Param::Addr(a) => self.code.get(a as usize).copied().unwrap_or(0),
            Param::Rela(a) => self.code.get((self.ra + a) as usize).copied().unwrap_or(0),
        }
    }

    fn get_or_resize(&mut self, idx: usize) -> &mut isize {
        if idx >= self.code.len() {
            self.code.resize(idx + 1, 0);
        }
        &mut self.code[idx]
    }

    fn get_mut(&mut self, p: Param) -> &mut isize {
        match p {
            Param::Imme(_) => panic!("write operand cannot be an immediate value {p:?}"),
            Param::Addr(a) => self.get_or_resize(a as usize),
            Param::Rela(a) => self.get_or_resize((self.ra + a) as usize),
        }
    }

    pub fn halted(&self) -> bool {
        matches!(Instruction::parse(&self.code[self.pc..]), Instruction::Halt)
    }

    fn step(&mut self, mut input: impl Iterator<Item = isize>) -> Option<isize> {
        loop {
            let instruction = Instruction::parse(&self.code[self.pc..]);
            match instruction {
                Instruction::Add(a, b, c) => *self.get_mut(c) = self.get(a) + self.get(b),
                Instruction::Mul(a, b, c) => *self.get_mut(c) = self.get(a) * self.get(b),
                Instruction::Input(a) => *self.get_mut(a) = input.next().expect("input exhausted"),
                Instruction::Output(a) => {
                    self.pc += instruction.width();
                    return Some(self.get(a));
                }
                Instruction::Jit(a, b) => {
                    if self.get(a) != 0 {
                        self.pc = self.get(b) as usize;
                        continue;
                    }
                }
                Instruction::Jif(a, b) => {
                    if self.get(a) == 0 {
                        self.pc = self.get(b) as usize;
                        continue;
                    }
                }
                Instruction::Tlt(a, b, c) => *self.get_mut(c) = (self.get(a) < self.get(b)).into(),
                Instruction::Teq(a, b, c) => *self.get_mut(c) = (self.get(a) == self.get(b)).into(),
                Instruction::Rela(a) => self.ra += self.get(a),
                Instruction::Halt => return None,
            }
            self.pc += instruction.width();
        }
    }

    pub fn run<'a>(
        &'a mut self,
        input: impl IntoIterator<Item = isize> + 'a,
    ) -> impl Iterator<Item = isize> + 'a {
        let mut input = input.into_iter();
        std::iter::from_fn(move || self.step(input.by_ref()))
    }

    pub fn deferred_run(&mut self) -> Deferred<'_> {
        Deferred::run(self)
    }
}

pub struct Deferred<'a> {
    output: Box<RefCell<dyn Iterator<Item = isize> + 'a>>,
    tx: mpsc::Sender<isize>,
}

impl<'a> Deferred<'a> {
    fn run(program: &'a mut Intcode) -> Self {
        let (tx, rx) = mpsc::channel();
        Self { output: Box::new(RefCell::new(program.run(rx))), tx }
    }

    pub fn send(&self, x: isize) {
        self.tx.send(x).unwrap();
    }

    pub fn iter<'b>(&'b self) -> impl Iterator<Item = isize> + 'b + use<'b, 'a> {
        let mut iter = self.output.borrow_mut();
        std::iter::from_fn(move || iter.next())
    }
}

#[macro_export]
macro_rules! assert_intcode {
    ($program:expr, $expect:expr) => {
        itertools::assert_equal(Intcode::parse($program).run(std::iter::empty()), $expect)
    };
    ($program:expr, $input:expr, $expect:expr) => {
        itertools::assert_equal(Intcode::parse($program).run($input), $expect)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_output() {
        // Input one number, time by three, then output it.
        assert_intcode!("3,9,1002,9,3,9,4,9,99,0", [33], [99]);
    }

    #[test]
    fn test_teq() {
        // Test eq 8
        assert_intcode!("3,9,8,9,10,9,4,9,99,-1,8", [1], [0]);
        assert_intcode!("3,9,8,9,10,9,4,9,99,-1,8", [8], [1]);
        assert_intcode!("3,3,1108,-1,8,3,4,3,99", [1], [0]);
        assert_intcode!("3,3,1108,-1,8,3,4,3,99", [8], [1]);
    }

    #[test]
    fn test_tlt() {
        // Test lt 8
        assert_intcode!("3,9,7,9,10,9,4,9,99,-1,8", [7], [1]);
        assert_intcode!("3,9,7,9,10,9,4,9,99,-1,8", [8], [0]);
        assert_intcode!("3,3,1107,-1,8,3,4,3,99", [-1], [1]);
        assert_intcode!("3,3,1107,-1,8,3,4,3,99", [10], [0]);
    }

    #[test]
    fn test_jump() {
        // Jump if true / Jump if false
        assert_intcode!("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", [0], [0]);
        assert_intcode!("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", [1], [1]);
        assert_intcode!("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", [0], [0]);
        assert_intcode!("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", [-1], [1]);
    }

    #[test]
    fn test_large_program() {
        let program = r"
3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
";
        assert_intcode!(program, [-1], [999]);
        assert_intcode!(program, [8], [1000]);
        assert_intcode!(program, [10], [1001]);
    }

    #[test]
    fn test_quine() {
        let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let expect = quine.split(",").map(|e| e.parse::<isize>().unwrap());
        assert_intcode!(quine, expect);
    }

    #[test]
    fn test_bignum() {
        assert_intcode!("1102,34915192,34915192,7,4,7,99,0", [1219070632396864]);
        assert_intcode!("104,1125899906842624,99", [1125899906842624]);
    }
}
