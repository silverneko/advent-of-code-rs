use itertools::iproduct;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, BufRead};
use utils::{Grid, Point};

fn bfs(grid: &Grid<char>, st: Point) -> Grid<usize> {
    let mut bq = VecDeque::from([st]);
    let mut dist: Grid<usize> = Grid::new(usize::MAX, grid.h, grid.w);
    dist[st] = 0;
    while let Some(v) = bq.pop_front() {
        for dv in [Point::UP, Point::DOWN, Point::RIGHT, Point::LEFT] {
            let n = v + dv;
            if matches!(grid.get(n), Some('.' | 'S' | 'E')) && dist[n] > dist[v].saturating_add(1) {
                dist[n] = dist[v] + 1;
                bq.push_back(n);
            }
        }
    }
    dist
}

fn l1_norm(v: Point) -> usize {
    v.0.unsigned_abs() + v.1.unsigned_abs()
}

struct TestCase {
    grid: Grid<char>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let grid = reader
            .lines()
            .map(|e| e.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into();
        Self { grid }
    }

    fn solve(&self, max_shortcut_len: usize) -> HashMap<usize, usize> {
        let grid = &self.grid;
        let start = iproduct!(0..grid.h, 0..grid.w).find(|&p| grid[p] == 'S').unwrap().into();
        let end = iproduct!(0..grid.h, 0..grid.w).find(|&p| grid[p] == 'E').unwrap().into();
        let ds = bfs(grid, start);
        let de = bfs(grid, end);
        let se_dist = ds[end];
        assert_ne!(se_dist, usize::MAX);

        let mut ans = HashMap::new();
        for v in iproduct!(0..grid.h, 0..grid.w).map(Point::from) {
            for u in iproduct!(0..grid.h, 0..grid.w).map(Point::from) {
                let shortcut_len = l1_norm(u - v);
                let nd = shortcut_len.saturating_add(ds[v]).saturating_add(de[u]);
                if shortcut_len <= max_shortcut_len && nd < se_dist {
                    *ans.entry(se_dist - nd).or_default() += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let data = TestCase::parse(stdin().lock());
    let ans1: usize = data.solve(2).into_iter().filter(|&(d, _)| d >= 100).map(|(_, n)| n).sum();
    let ans2: usize = data.solve(20).into_iter().filter(|&(d, _)| d >= 100).map(|(_, n)| n).sum();
    println!("{ans1},{ans2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn get_input() -> &'static str {
        r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
        .trim()
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            TestCase::parse(Cursor::new(get_input())).solve(2),
            HashMap::from([
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1),
            ])
        );
    }

    #[test]
    fn test_part2() {
        let data = TestCase::parse(Cursor::new(get_input()));
        assert_eq!(
            HashMap::from_iter(data.solve(20).into_iter().filter(|&(d, _)| d >= 50)),
            HashMap::from([
                (50, 32),
                (52, 31),
                (54, 29),
                (56, 39),
                (58, 25),
                (60, 23),
                (62, 20),
                (64, 19),
                (66, 12),
                (68, 14),
                (70, 12),
                (72, 22),
                (74, 4),
                (76, 3),
            ])
        );
    }
}
