use std::io::stdin;
use utils::Intcode;

fn feed_input(program: &Intcode, input: &str) {
    for n in program.clone().run(input.bytes().map(|b| b.into())) {
        if n < 256 && (n as u8).is_ascii() {
            print!("{}", n as u8 as char);
        } else {
            println!("Answer {n}");
            break;
        }
    }
}

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();

    let part1_input = "
NOT C J
AND D J
NOT A T
OR T J
WALK
".trim_start();
    feed_input(&program, part1_input);
    // ABCDEFGHI
    //    *   *
    //    **   *
    //    ***
    let part2_input = "
NOT A T
OR T J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT D T
OR F T
OR I T
AND E T
OR H T
AND T J
RUN
".trim_start();
    feed_input(&program, part2_input);
}
