use itertools::iproduct;
use std::io::stdin;
use utils::Intcode;

fn part1(program: &Intcode, a: isize, b: isize) -> isize {
    let mut program = program.clone();
    program.code[1] = a;
    program.code[2] = b;
    program.run();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_program() {
        assert_eq!(Intcode::parse("1,0,0,0,99").run().code, [2, 0, 0, 0, 99]);
        assert_eq!(Intcode::parse("2,3,0,3,99").run().code, [2, 3, 0, 6, 99]);
        assert_eq!(Intcode::parse("2,4,4,5,99,0").run().code, [2, 4, 4, 5, 99, 9801]);
        assert_eq!(Intcode::parse("1,1,1,4,99,5,6,0,99").run().code, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
