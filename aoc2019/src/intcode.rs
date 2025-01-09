use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct Intcode {
    pub code: Vec<isize>,

    pc: usize,
    ra: isize,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
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

    pub fn input(&mut self, ns: &[isize]) {
        self.inputs.extend(ns);
    }

    pub fn output(&self) -> &[isize] {
        &self.outputs
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

    pub fn run(&mut self) -> &mut Self {
        loop {
            let instruction = Instruction::parse(&self.code[self.pc..]);
            match instruction {
                Instruction::Add(a, b, c) => *self.get_mut(c) = self.get(a) + self.get(b),
                Instruction::Mul(a, b, c) => *self.get_mut(c) = self.get(a) * self.get(b),
                Instruction::Input(a) => *self.get_mut(a) = self.inputs.pop_front().expect("EOF"),
                Instruction::Output(a) => self.outputs.push(self.get(a)),
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
                Instruction::Halt => break,
            }
            self.pc += instruction.width();
        }
        self
    }
}

#[macro_export]
macro_rules! assert_intcode {
    ($program:expr, $input:expr, $expect:expr) => {{
        let mut program = Intcode::parse($program);
        program.input($input);
        program.run();
        assert_eq!(program.output(), $expect);
    }};
}
