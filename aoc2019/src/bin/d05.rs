use std::io::stdin;
use utils::Intcode;

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    for x in [1, 5] {
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
    fn test_io_program() {
        // Times input number by three
        assert_intcode!("3,9,1002,9,3,9,4,9,99,0", &[33], &[99]);
    }

    #[test]
    fn test_teq() {
        assert_intcode!("3,9,8,9,10,9,4,9,99,-1,8", &[1], &[0]);
        assert_intcode!("3,9,8,9,10,9,4,9,99,-1,8", &[8], &[1]);
        assert_intcode!("3,3,1108,-1,8,3,4,3,99", &[1], &[0]);
        assert_intcode!("3,3,1108,-1,8,3,4,3,99", &[8], &[1]);
    }

    #[test]
    fn test_tlt() {
        assert_intcode!("3,9,7,9,10,9,4,9,99,-1,8", &[7], &[1]);
        assert_intcode!("3,9,7,9,10,9,4,9,99,-1,8", &[8], &[0]);
        assert_intcode!("3,3,1107,-1,8,3,4,3,99", &[-1], &[1]);
        assert_intcode!("3,3,1107,-1,8,3,4,3,99", &[10], &[0]);
    }

    #[test]
    fn test_jump() {
        assert_intcode!("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &[0], &[0]);
        assert_intcode!("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &[1], &[1]);
        assert_intcode!("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &[0], &[0]);
        assert_intcode!("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &[-1], &[1]);
    }

    #[test]
    fn test_large() {
        let program = r"
3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
";
        assert_intcode!(&program, &[-1], &[999]);
        assert_intcode!(&program, &[8], &[1000]);
        assert_intcode!(&program, &[10], &[1001]);
    }
}
