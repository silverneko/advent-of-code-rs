use std::io::stdin;
use utils::Intcode;

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    for x in [1, 2] {
        let mut program = program.clone();
        program.input(&[x]);
        program.run();
        println!("Input: {x}");
        println!("Output: {:?}", program.output());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::assert_intcode;

    #[test]
    fn test_quine() {
        let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let expected: Vec<_> = quine.split(",").map(|e| e.parse::<isize>().unwrap()).collect();
        assert_intcode!(quine, &[], &expected);
    }

    #[test]
    fn test_bignum() {
        assert_intcode!("1102,34915192,34915192,7,4,7,99,0", &[], &[1219070632396864]);
        assert_intcode!("104,1125899906842624,99", &[], &[1125899906842624]);
    }
}
