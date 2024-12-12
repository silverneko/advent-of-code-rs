use std::io;
use std::iter::zip;

fn is_increasing_decreasing(v: &[i32]) -> bool {
    for r in [(1..=3), (-3..=-1)] {
        if zip(v, &v[1..]).map(|(a, b)| b - a).all(|d| r.contains(&d)) {
            return true;
        }
    }
    false
}

fn main() {
    let mut total = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let v: Vec<i32> = line
            .split_ascii_whitespace()
            .map(str::parse::<i32>)
            .collect::<Result<Vec<i32>, _>>()
            .unwrap();

        if is_increasing_decreasing(&v) {
            total += 1;
        } else {
            for i in 0..v.len() {
                let mut vv = Vec::with_capacity(v.len() - 1);
                vv.extend_from_slice(&v[0..i]);
                vv.extend_from_slice(&v[i + 1..]);
                if is_increasing_decreasing(&vv) {
                    total += 1;
                    break;
                }
            }
        }
    }
    println!("{total}");
}
