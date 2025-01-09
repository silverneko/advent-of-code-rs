use itertools::Itertools;
use std::io::stdin;
use utils::Intcode;

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    for x in [1, 2] {
        let mut program = program.clone();
        println!("Input: {x}");
        println!("Output: [{}]", program.run([x]).join(", "));
    }
}
