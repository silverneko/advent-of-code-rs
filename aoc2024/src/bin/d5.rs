use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

fn mid<T: Copy>(s: &[T]) -> T {
    s[s.len() / 2]
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());

    let rules: HashSet<(i32, i32)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split("|").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap())
        .collect();

    let cmp = |a: &i32, b: &i32| {
        if a == b {
            Ordering::Equal
        } else if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    let (mid_sum, mid_sum2) = lines
        .map(|line| {
            let mut nums = line
                .split(",")
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .expect("parsed number list");

            if nums.is_sorted_by(|a, b| cmp(a, b) == Ordering::Less) {
                (mid(&nums), 0)
            } else {
                nums.sort_by(cmp);
                (0, mid(&nums))
            }
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("{mid_sum},{mid_sum2}");
}
