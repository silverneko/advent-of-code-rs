use rustyline::{error::ReadlineError, DefaultEditor};
use std::env;
use std::fs::read_to_string;
use std::sync::mpsc;
use utils::{BatchLines, Intcode};

fn main() {
    let argv1 = env::args().nth(1).expect("argv[1] must be the intcode program");
    let mut program: Intcode = read_to_string(argv1).unwrap().parse().unwrap();
    let (tx, rx) = mpsc::channel();

    let input = r"
south
south
take tambourine
north
north
west
south
take polygon
north
east
north
west
take boulder
east
north
take manifold
north
take hologram
south
west
take fuel cell
south
east
south
take fixed point
north
west
north
north
take wreath
east
east
drop tambourine
drop fuel cell
drop hologram
drop wreath
inv
north
";
    /*
    drop polygon
    drop manifold
    drop boulder
    drop fixed point
    */
    for c in input.bytes() {
        tx.send(c.into()).unwrap();
    }
    std::thread::spawn(move || {
        let mut rl = DefaultEditor::new().unwrap();
        loop {
            match rl.readline("> ") {
                Ok(line) => {
                    for c in line.bytes() {
                        tx.send(c.into()).unwrap();
                    }
                    tx.send(b'\n'.into()).unwrap();
                }
                Err(ReadlineError::Interrupted) => {}
                e => {
                    println!("Error {e:?}");
                    break;
                }
            }
        }
    });

    let output = program.run(rx);
    for line in output.map(|b| b as u8 as char).batch_lines() {
        println!("{line}");
    }
}
