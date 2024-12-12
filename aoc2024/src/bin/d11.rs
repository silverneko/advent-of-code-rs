use std::collections::HashMap;

fn main() {
    let input: HashMap<u64, u64> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|e| (e.parse::<u64>().unwrap(), 1))
        .collect();

    let state25 = sim_n(&input, 25);
    let state75 = sim_n(&state25, 50);

    println!(
        "{ans1},{ans2}",
        ans1 = state25.values().sum::<u64>(),
        ans2 = state75.values().sum::<u64>()
    );
}

fn sim_one_step(set: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut nset = HashMap::new();
    for (n, weight) in set.into_iter() {
        if n == 0 {
            *nset.entry(1).or_default() += weight;
        } else {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let (a, b) = s.split_at(s.len() / 2);
                *nset.entry(a.parse().unwrap()).or_default() += weight;
                *nset.entry(b.parse().unwrap()).or_default() += weight;
            } else {
                *nset.entry(n * 2024).or_default() += weight;
            }
        }
    }
    nset
}

fn sim_n(set: &HashMap<u64, u64>, n: usize) -> HashMap<u64, u64> {
    let mut nset = set.clone();
    for _ in 0..n {
        nset = sim_one_step(nset);
    }
    nset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = HashMap::from([(125, 1), (17, 1)]);
        let ans = sim_n(&input, 25);
        assert_eq!(ans.values().sum::<u64>(), 55312);
    }
}
