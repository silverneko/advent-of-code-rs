use std::io;

fn main() {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let mut words = line.split_ascii_whitespace();
        list_a.push(words.next().unwrap().parse().unwrap());
        list_b.push(words.next().unwrap().parse().unwrap());
    }
    list_a.sort();
    list_b.sort();
    let mut diff = 0;
    /*
    for (a, b) in list_a.iter().zip(list_b) {
        diff += (a - b).abs();
    }
    */
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
