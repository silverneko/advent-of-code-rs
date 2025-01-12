use itertools::{chain, Itertools};
use std::io::stdin;
use std::iter::once;

fn part1(n: &usize) -> bool {
    let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    digits.is_sorted() && digits.windows(2).any(|w| w[0] == w[1])
}

fn part2(n: &usize) -> bool {
    let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    digits.is_sorted()
        && chain!(once(99), digits, once(99))
            .tuple_windows()
            .any(|(a, b, c, d)| a != b && b == c && c != d)
}

fn main() {
    let input: String = stdin().lines().next().unwrap().unwrap();
    let (a, b) = input.split('-').map(|e| e.parse().unwrap()).collect_tuple().unwrap();
    dbg!((a..=b).filter(part1).count());
    dbg!((a..=b).filter(part2).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(&111111));
        assert!(!part1(&223450));
        assert!(!part1(&123789));
    }

    #[test]
    fn test_part2() {
        assert!(part2(&112233));
        assert!(!part2(&123444));
        assert!(part2(&111122));
    }
}
