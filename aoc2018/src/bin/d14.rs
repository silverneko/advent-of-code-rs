struct State {
    p: usize,
    q: usize,
    nums: Vec<u8>,
}

impl State {
    fn step(&mut self) {
        let s = self.nums[self.p] + self.nums[self.q];
        if s >= 10 {
            self.nums.push(s / 10);
        }
        self.nums.push(s % 10);
        self.p = (self.p + self.nums[self.p] as usize + 1) % self.nums.len();
        self.q = (self.q + self.nums[self.q] as usize + 1) % self.nums.len();
    }

    fn score(&self, t: usize) -> String {
        self.nums[t..].iter().take(10).map(|d| d.to_string()).collect()
    }
}

fn main() {
    let input = 635041;
    let input2 = [6, 3, 5, 0, 4, 1];
    let mut state = State { p: 0, q: 1, nums: vec![3, 7] };
    let pos = loop {
        state.step();
        if state.nums.ends_with(&input2) {
            break state.nums.len() - 6;
        }
        let ss = &state.nums[..state.nums.len() - 1];
        if ss.ends_with(&input2) {
            break ss.len() - 6;
        }
    };
    println!("{},{pos}", state.score(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let mut state = State { p: 0, q: 1, nums: vec![3, 7] };
        while state.nums.len() < 2030 {
            state.step();
        }
        assert_eq!(state.score(5), "0124515891");
        assert_eq!(state.score(9), "5158916779");
        assert_eq!(state.score(18), "9251071085");
        assert_eq!(state.score(2018), "5941429882");
    }
}
