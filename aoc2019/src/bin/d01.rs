use std::io::stdin;

fn fuel(m: i32) -> i32 {
    m / 3 - 2
}

fn fuel2(m: i32) -> i32 {
    std::iter::successors(Some(fuel(m)), |&n| match fuel(n) {
        s @ 1.. => Some(s),
        _ => None,
    })
    .sum()
}

fn main() {
    let m: Vec<i32> = stdin().lines().map(|e| e.unwrap().parse().unwrap()).collect();
    let ans1: i32 = m.iter().map(|&e| fuel(e)).sum();
    let ans2: i32 = m.iter().map(|&e| fuel2(e)).sum();
    dbg!(ans1, ans2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
        assert_eq!(fuel2(100756), 50346);
    }
}
