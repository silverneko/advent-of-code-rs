use std::io::{BufRead, Cursor};
use utils::{Direction, Grid, Point};

fn parse_input(text: &str) -> Grid<u8> {
    let grid: Vec<Vec<u8>> = Cursor::new(text.trim())
        .lines()
        .map(|line| line.unwrap().into())
        .collect();
    grid.into()
}

fn corners(s: Point, grid: &Grid<u8>) -> u64 {
    /*
     * 123
     * 0.4
     * 765
     */
    let D: [Point; 8] = [
        Direction::LEFT,
        Direction::LEFT + Direction::UP,
        Direction::UP,
        Direction::UP + Direction::RIGHT,
        Direction::RIGHT,
        Direction::RIGHT + Direction::DOWN,
        Direction::DOWN,
        Direction::DOWN + Direction::LEFT,
    ];
    let mut cc = 0;
    for (na, da, nb) in [(0, 1, 2), (2, 3, 4), (4, 5, 6), (6, 7, 0)] {
        let na = grid.get(s + D[na]).copied().unwrap_or(0);
        let da = grid.get(s + D[da]).copied().unwrap_or(0);
        let nb = grid.get(s + D[nb]).copied().unwrap_or(0);
        let t = grid[s];
        if t != na && t != nb {
            cc += 1;
        } else if t == na && t == nb && t != da {
            cc += 1;
        }
    }
    cc
}

fn dfs(s: Point, grid: &Grid<u8>, visited: &mut Grid<bool>) -> (u64, u64, u64) {
    visited[s] = true;
    let mut area = 1;
    let mut peri = 0;
    let mut side = corners(s, grid);
    for d in [
        Direction::UP,
        Direction::DOWN,
        Direction::RIGHT,
        Direction::LEFT,
    ] {
        let ns = s + d;
        let nt = grid.get(ns);
        if nt.is_some() && *nt.unwrap() == grid[s] {
            if !visited[ns] {
                let (a, p, c) = dfs(ns, grid, visited);
                area += a;
                peri += p;
                side += c;
            }
        } else {
            peri += 1;
        }
    }
    (area, peri, side)
}

fn solve(grid: &Grid<u8>) -> (u64, u64) {
    let mut visited: Grid<bool> = Grid::new(false, grid.h, grid.w);
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..grid.h {
        for j in 0..grid.w {
            if visited[(i, j)] {
                continue;
            }
            let (area, peri, side) = dfs((i, j).into(), grid, &mut visited);
            sum1 += area * peri;
            sum2 += area * side;
        }
    }
    (sum1, sum2)
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let grid = parse_input(&input);
    println!("{:?}", solve(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_small() {
        let input = r"
AAAA
BBCD
BBCC
EEEC
";
        let grid = parse_input(input);
        assert_eq!(solve(&grid), (140, 80));
    }

    #[test]
    fn test_sample_medium() {
        let input = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        let grid = parse_input(input);
        assert_eq!(solve(&grid), (772, 436));
    }

    #[test]
    fn test_sample_e() {
        let input = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        let grid = parse_input(input);
        assert_eq!(solve(&grid).1, 236);
    }

    #[test]
    fn test_sample_ab() {
        let input = r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let grid = parse_input(input);
        assert_eq!(solve(&grid).1, 368);
    }
}
