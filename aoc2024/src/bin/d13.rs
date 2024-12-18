use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy)]
struct TestCase {
    a: (i64, i64),
    b: (i64, i64),
    c: (i64, i64),
}

fn ext_gcd(
    (a1, b1, s1): (i64, i64, i64),
    (a2, b2, s2): (i64, i64, i64),
) -> ((i64, i64, i64), (i64, i64, i64)) {
    if s1 == 0 {
        ((a1, b1, s1), (a2, b2, s2))
    } else {
        let t = s2 / s1;
        assert_eq!(s2 % s1, s2 - t * s1);
        ext_gcd((a2 - t * a1, b2 - t * b1, s2 % s1), (a1, b1, s1))
    }
}

// Solve (x, y) for ax + by = c
fn solve(data: TestCase) -> Option<(i64, i64)> {
    let TestCase { a: (x1, y1), b: (x2, y2), c: (x, y) } = data;
    let d = x2 * y1 - x1 * y2;
    assert_ne!(d, 0);
    let A = (y * x2 - x * y2) / d;
    let B = (x * y1 - y * x1) / d;
    if A >= 0 && B >= 0 && (A * x1 + B * x2) == x && (A * y1 + B * y2) == y {
        Some((A, B))
    } else {
        None
    }
}

fn main() {
    let parse_re = Regex::new(r"^\D*(\d+)\D+(\d+)\D*$").unwrap();
    let test_cases: Vec<TestCase> = std::io::stdin()
        .lines()
        .map(|e| e.unwrap())
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let a = chunk.next().unwrap();
            let b = chunk.next().unwrap();
            let c = chunk.next().unwrap();
            let a = parse_re.captures(&a).unwrap();
            let b = parse_re.captures(&b).unwrap();
            let c = parse_re.captures(&c).unwrap();
            TestCase {
                a: (a[1].parse().unwrap(), a[2].parse().unwrap()),
                b: (b[1].parse().unwrap(), b[2].parse().unwrap()),
                c: (c[1].parse().unwrap(), c[2].parse().unwrap()),
            }
        })
        .collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for data in test_cases {
        if let Some((a, b)) = solve(data) {
            sum1 += 3 * a + b;
        }
        let data_large =
            TestCase { c: (data.c.0 + 10000000000000, data.c.1 + 10000000000000), ..data };
        if let Some((a, b)) = solve(data_large) {
            sum2 += 3 * a + b;
        }
    }
    println!("{sum1},{sum2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let input = TestCase { a: (94, 34), b: (22, 67), c: (8400, 5400) };
        assert_eq!(solve(input), Some((80, 40)));
    }

    #[test]
    fn test_sample2() {
        let input = TestCase { a: (26, 66), b: (67, 21), c: (12748, 12176) };
        assert_eq!(solve(input), None);
    }

    #[test]
    fn test_sample3() {
        let input = TestCase { a: (17, 86), b: (84, 37), c: (7870, 6450) };
        assert_eq!(solve(input), Some((38, 86)));
    }

    #[test]
    fn test_sample4() {
        let input = TestCase { a: (69, 23), b: (27, 71), c: (18641, 10279) };
        assert_eq!(solve(input), None);
    }
}
