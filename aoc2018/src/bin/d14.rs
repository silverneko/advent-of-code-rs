struct State {
    p: usize,
    q: usize,
    nums: Vec<u8>,
}

impl State {
    fn new() -> Self {
        Self { p: 0, q: 1, nums: vec![3, 7] }
    }

    fn step(self) -> Self {
        let Self { p, q, mut nums } = self;
        let s = nums[p] + nums[q];
        if s >= 10 {
            nums.push(s / 10);
        }
        nums.push(s % 10);
        let p = (p + nums[p] as usize + 1) % nums.len();
        let q = (q + nums[q] as usize + 1) % nums.len();
        Self { p, q, nums }
    }

    fn gen(n: usize) -> String {
        let mut state = Self::new();
        while state.nums.len() < n + 10 {
            state = state.step();
        }
        state.nums[n..].iter().take(10).map(|d| d.to_string()).collect()
    }

    fn find(pat: &str) -> usize {
        let seq: Vec<u8> = pat.chars().map(|c| c.to_string().parse().unwrap()).collect();
        let mut state = Self::new();
        loop {
            for off in [0, 1] {
                let len = state.nums.len() - off;
                let nums = &state.nums[..len];
                if nums.ends_with(&seq) {
                    return len - seq.len();
                }
            }
            state = state.step();
        }
    }
}

fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    println!("{}", State::gen(input.parse().unwrap()));
    println!("{}", State::find(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(State::gen(5), "0124515891");
        assert_eq!(State::gen(9), "5158916779");
        assert_eq!(State::gen(18), "9251071085");
        assert_eq!(State::gen(2018), "5941429882");
    }

    #[test]
    fn test_part2() {
        assert_eq!(State::find("01245"), 5);
        assert_eq!(State::find("51589"), 9);
        assert_eq!(State::find("92510"), 18);
        assert_eq!(State::find("59414"), 2018);
    }
}
