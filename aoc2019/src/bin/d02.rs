use itertools::iproduct;
use std::io::stdin;
use utils::Intcode;

fn part1(program: &Intcode, a: isize, b: isize) -> isize {
    let mut program = program.clone();
    program.code[1] = a;
    program.code[2] = b;
    program.run([]).last();
    program.code[0]
}

fn part2(program: &Intcode, needle: isize) -> isize {
    let (a, b) = iproduct!(0..=99, 0..=99).find(|&(a, b)| part1(program, a, b) == needle).unwrap();
    100 * a + b
}

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    dbg!(part1(&program, 12, 2), part2(&program, 19690720));
}
