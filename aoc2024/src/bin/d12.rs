use std::io::{BufRead, Cursor};

fn parse_input(text: &str) -> Vec<Vec<u8>> {
    Cursor::new(text.trim())
        .lines()
        .map(|line| line.unwrap().into())
        .collect()
}

/*
 * 123
 * 0.4
 * 765
 */
fn neighbor((x, y): (usize, usize), d: usize, grid: &Vec<Vec<u8>>) -> Option<(usize, usize, u8)> {
    static D: [(isize, isize); 8] = [
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];
    let (dx, dy) = D[d];
    let nx = x.checked_add_signed(dx)?;
    let ny = y.checked_add_signed(dy)?;
    Some((nx, ny, *grid.get(nx)?.get(ny)?))
}

fn corners(s: (usize, usize), grid: &Vec<Vec<u8>>) -> u64 {
    let mut cc = 0;
    for (na, da, nb) in [(0, 1, 2), (2, 3, 4), (4, 5, 6), (6, 7, 0)] {
        let na = neighbor(s, na, grid).map(|t| t.2).unwrap_or(0);
        let da = neighbor(s, da, grid).map(|t| t.2).unwrap_or(0);
        let nb = neighbor(s, nb, grid).map(|t| t.2).unwrap_or(0);
        let t = grid[s.0][s.1];
        if t != na && t != nb {
            cc += 1;
        } else if t == na && t == nb && t != da {
            cc += 1;
        }
    }
    cc
}

fn dfs(
    (x, y): (usize, usize),
    grid: &Vec<Vec<u8>>,
    visited: &mut Vec<Vec<bool>>,
) -> (u64, u64, u64) {
    visited[x][y] = true;
    let mut area = 1;
    let mut peri = 0;
    let mut side = corners((x, y), grid);
    for d in [0, 2, 4, 6] {
        if let Some((nx, ny, tt)) = neighbor((x, y), d, grid) {
            if tt == grid[x][y] {
                if !visited[nx][ny] {
                    let (a, p, c) = dfs((nx, ny), grid, visited);
                    area += a;
                    peri += p;
                    side += c;
                }
            } else {
                peri += 1;
            }
        } else {
            peri += 1;
        }
    }
    (area, peri, side)
}

fn solve(grid: &Vec<Vec<u8>>) -> (u64, u64) {
    let h = grid.len();
    let w = grid.first().unwrap().len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }
            let (area, peri, side) = dfs((i, j), grid, &mut visited);
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
