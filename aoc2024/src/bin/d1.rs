use itertools::Itertools;

fn main() {
    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = std::io::stdin()
        .lines()
        .map(|e| {
            e.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();
    list_a.sort();
    list_b.sort();
    println!("{}", list_a.iter().zip(list_b.iter()).map(|(a, b)| (a - b).abs()).sum::<i32>());

    let mut diff = 0;
    let mut i = 0;
    let mut j = 0;

    while i < list_a.len() {
        while j < list_b.len() && list_b[j] < list_a[i] {
            j += 1;
        }
        let mut pi = i;
        while pi < list_a.len() && list_a[pi] == list_a[i] {
            pi += 1;
        }
        let mut pj = j;
        while pj < list_b.len() && list_b[pj] == list_a[i] {
            pj += 1;
        }
        diff += list_a[i] * (pj - j) as i32 * (pi - i) as i32;
        i = pi;
        j = pj;
    }
    println!("{diff}");
}
