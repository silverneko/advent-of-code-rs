use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

fn mid<T: Copy>(s: &[T]) -> T {
    s[s.len() / 2]
}

fn main() {
    let mut rules: HashSet<(i32, i32)> = HashSet::new();

    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        /*
        let [a, b] = line
            .split("|")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .expect("parsed number list")
            .try_into()
            .expect("two numbers");
        rules.insert((a, b));
        */
        rules.insert(line.split("|").map(|s| s.parse::<i32>().unwrap()).collect_tuple().unwrap());
    }

    let cmp = |a: &i32, b: &i32| {
        if a == b {
            Ordering::Equal
        } else if rules.contains(&(*a, *b)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    let mut mid_sum = 0;
    let mut mid_sum2 = 0;
    while let Some(line) = lines.next() {
        let mut nums = line
            .split(",")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .expect("parsed number list");

        if nums.is_sorted_by(|a, b| cmp(a, b) == Ordering::Less) {
            mid_sum += mid(&nums);
        } else {
            nums.sort_by(cmp);
            mid_sum2 += mid(&nums)
        }
    }
    println!("{mid_sum},{mid_sum2}");
}
