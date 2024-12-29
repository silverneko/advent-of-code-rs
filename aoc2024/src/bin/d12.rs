use itertools::{iproduct, Itertools};
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

fn parse_input(reader: impl BufRead) -> Grid<u8> {
    reader.lines().map(|line| line.unwrap().into()).collect::<Vec<_>>().into()
}

fn corners(s: Point, grid: &Grid<u8>) -> usize {
    let t = grid[s];
    Direction::cardinals()
        .into_iter()
        .circular_tuple_windows()
        .filter(|(a, b)| {
            // [a][d]
            // [s][b]
            let ta = grid.get(s + a).copied().unwrap_or_default();
            let tb = grid.get(s + b).copied().unwrap_or_default();
            let td = grid.get(s + a + b).copied().unwrap_or_default();
            (t != ta && t != tb) || (t == ta && t == tb && t != td)
        })
        .count()
}

fn dfs(s: Point, grid: &Grid<u8>, visited: &mut Grid<bool>) -> (usize, usize, usize) {
    if visited[s] {
        return (0, 0, 0);
    }
    visited[s] = true;
    Direction::cardinals().into_iter().fold((1, 0, corners(s, grid)), |(area, peri, side), d| {
        let ns = s + d;
        if grid.get(ns).copied() == Some(grid[s]) {
            let (a, p, c) = dfs(ns, grid, visited);
            (area + a, peri + p, side + c)
        } else {
            (area, peri + 1, side)
        }
    })
}

fn solve(grid: &Grid<u8>) -> (usize, usize) {
    let mut visited: Grid<bool> = Grid::new(false, grid.h, grid.w);
    iproduct!(0..grid.h, 0..grid.w).fold((0, 0), |(sum1, sum2), p| {
        let (area, peri, side) = dfs(p.into(), grid, &mut visited);
        (sum1 + area * peri, sum2 + area * side)
    })
}

fn main() {
    println!("{:?}", solve(&parse_input(stdin().lock())));
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
"
        .trim();
        assert_eq!(solve(&parse_input(input.as_bytes())), (140, 80));
    }

    #[test]
    fn test_sample_medium() {
        let input = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
        .trim();
        assert_eq!(solve(&parse_input(input.as_bytes())), (772, 436));
    }

    #[test]
    fn test_sample_e() {
        let input = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
        .trim();
        assert_eq!(solve(&parse_input(input.as_bytes())).1, 236);
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
"
        .trim();
        assert_eq!(solve(&parse_input(input.as_bytes())).1, 368);
    }
}
