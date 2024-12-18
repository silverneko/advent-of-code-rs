use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, BufRead};
use utils::{Direction, Point};

struct TestCase {
    h: isize,
    w: isize,
    b: HashMap<Point, usize>,
}

impl TestCase {
    fn parse(reader: impl BufRead, h: isize, w: isize) -> Self {
        let b = reader
            .lines()
            .map(|e| {
                e.unwrap()
                    .split(',')
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap()
                    .into()
            })
            .enumerate()
            .map(|e| (e.1, e.0))
            .collect();
        Self { h, w, b }
    }

    fn get_block(&self, n: Point) -> Option<usize> {
        if (0..self.h).contains(&n.0) && (0..self.w).contains(&n.1) {
            Some(self.b.get(&n).copied().unwrap_or(usize::MAX))
        } else {
            None
        }
    }

    fn solve(&self, limit: usize) -> Option<usize> {
        let mut dist: HashMap<Point, usize> = HashMap::from([(Point(0, 0), 0)]);
        let mut bq: VecDeque<Point> = VecDeque::from([Point(0, 0)]);
        while let Some(v) = bq.pop_front() {
            for dv in [
                Direction::UP,
                Direction::DOWN,
                Direction::RIGHT,
                Direction::LEFT,
            ] {
                let n = v + dv;
                if let Some(t) = self.get_block(n) {
                    if t >= limit && !dist.contains_key(&n) {
                        dist.insert(n, dist[&v] + 1);
                        bq.push_back(n);
                    }
                }
            }
        }
        dist.get(&Point(self.h - 1, self.w - 1)).copied()
    }

    fn part2(&self) -> Point {
        let n = (0..self.b.len())
            .collect::<Vec<_>>()
            .partition_point(|&i| self.solve(i).is_some());
        *self.b.iter().find(|(_, &t)| t == n - 1).unwrap().0
    }
}

fn main() {
    let data = TestCase::parse(stdin().lock(), 71, 71);
    println!("{:?}", data.solve(1024));
    println!("{:?}", data.part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sample() {
        let input = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
        .trim();
        let reader = Cursor::new(input);
        let data = TestCase::parse(reader, 7, 7);
        assert_eq!(data.solve(12), Some(22));
        assert_eq!(data.part2(), Point(6, 1));
    }
}
