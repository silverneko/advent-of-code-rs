use std::io;

const numbers: &[(u32, &str)] = &[
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
    (0, "0"),
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
];

fn main() {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut a = None;
        let mut b = None;
        for (idx, _) in line.char_indices() {
            let s = &line[idx..];
            for (n, p) in numbers {
                if s.starts_with(p) {
                    a = a.or(Some(n));
                    b = Some(n);
                }
            }
        }
        let a = a.unwrap();
        let b = b.unwrap();
        sum += a * 10 + b;
    }
    println!("{sum}");
}
