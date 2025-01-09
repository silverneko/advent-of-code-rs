use itertools::{chain, izip, Itertools};
use std::io::{read_to_string, stdin};
use std::iter::once;
use std::sync::mpsc;
use utils::Intcode;

fn run_setting(program: &Intcode, seq: [isize; 5]) -> isize {
    let mut amp = vec![program.clone(); seq.len()];
    let init: Box<dyn Iterator<Item = isize>> = Box::new(once(0));
    izip!(amp.iter_mut(), seq)
        .fold(init, |st, (a, s)| Box::new(a.run(chain![once(s), st])))
        .last()
        .unwrap()
}

fn run_feedback_setting(program: &Intcode, seq: [isize; 5]) -> isize {
    let mut amp = vec![program.clone(); seq.len()];
    let (tx, rx) = mpsc::channel();
    let init: Box<dyn Iterator<Item = isize>> = Box::new(chain![once(0), rx]);
    izip!(amp.iter_mut(), seq)
        .fold(init, |st, (a, s)| Box::new(a.run(chain![once(s), st])))
        .inspect(|&x| tx.send(x).unwrap())
        .last()
        .unwrap()
}

fn main() {
    let program: Intcode = read_to_string(stdin()).unwrap().parse().unwrap();
    let ans1 = (0..=4)
        .permutations(5)
        .map(|s| run_setting(&program, s.try_into().unwrap()))
        .max()
        .unwrap();
    let ans2 = (5..=9)
        .permutations(5)
        .map(|s| run_feedback_setting(&program, s.try_into().unwrap()))
        .max()
        .unwrap();
    dbg!(ans1, ans2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_setting() {
        assert_eq!(
            run_setting(
                &Intcode::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
                [4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            run_setting(
                &Intcode::parse(
                    "3,23,3,24,1002,24,10,24,1002,23,-1,23,
101,5,23,23,1,24,23,23,4,23,99,0,0"
                ),
                [0, 1, 2, 3, 4]
            ),
            54321
        );
        assert_eq!(
            run_setting(
                &Intcode::parse(
                    "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
                ),
                [1, 0, 4, 3, 2]
            ),
            65210
        );
    }

    #[test]
    fn test_feedback_setting() {
        assert_eq!(
            run_feedback_setting(
                &Intcode::parse(
                    "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                ),
                [9, 8, 7, 6, 5]
            ),
            139629729
        );
        assert_eq!(
            run_feedback_setting(
                &Intcode::parse(
                    "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                ),
                [9, 7, 8, 5, 6]
            ),
            18216
        );
    }
}
