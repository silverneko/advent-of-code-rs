use regex::Regex;
use std::io;

fn main() {
    let mul_re = Regex::new(r"(do)\(\)|(don't)\(\)|(mul)\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let input = io::read_to_string(io::stdin()).unwrap();

    let mut mul_sum = 0;
    let mut cond_mul_sum = 0;
    let mut enabled = true;

    for cg in mul_re.captures_iter(&input) {
        let tokens: Vec<&str> = cg.iter().filter_map(|e| e).map(|e| e.as_str()).collect();
        match tokens[1] {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                let a: i32 = tokens[2].parse().unwrap();
                let b: i32 = tokens[3].parse().unwrap();
                mul_sum += a * b;
                if enabled {
                    cond_mul_sum += a * b;
                }
            }
            _ => panic!(),
        }
    }
    println!("{mul_sum}\n{cond_mul_sum}");
}
