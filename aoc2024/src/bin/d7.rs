fn backtrack(res: i64, s: &[i64]) -> (bool, bool) {
    assert!(!s.is_empty());
    let a = s.first().unwrap().clone();
    if s.len() == 1 {
        return if res == a { (true, true) } else { (false, false) };
    }
    let mut r1 = false;
    let mut r2 = false;
    if res >= a {
        let (b1, b2) = backtrack(res - a, &s[1..]);
        r1 |= b1;
        r2 |= b2;
    }
    if res % a == 0 {
        let (b1, b2) = backtrack(res / a, &s[1..]);
        r1 |= b1;
        r2 |= b2;
    }
    if let Some(prefix) = res.to_string().strip_suffix(&a.to_string()) {
        if !prefix.is_empty() {
            let (_, b2) = backtrack(prefix.parse::<i64>().unwrap(), &s[1..]);
            r2 |= b2;
        }
    }
    (r1, r2)
}

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in std::io::stdin().lines().map(|e| e.unwrap()) {
        let (res, args) = line.split_once(": ").unwrap();
        let res = res.parse::<i64>().unwrap();
        let args: Vec<i64> =
            args.split_whitespace().rev().map(|s| s.parse::<i64>().unwrap()).collect();
        let (b1, b2) = backtrack(res, &args);
        if b1 {
            sum1 += res;
        }
        if b2 {
            sum2 += res;
        }
    }
    println!("{sum1},{sum2}");
}
