use itertools::iproduct;
use utils::{Grid, Point};

fn count_xmas(grid: &Grid<char>, p: Point) -> usize {
    iproduct!(-1..=1, -1..=1)
        .map(Point::from)
        .filter(|d| {
            "XMAS".chars().enumerate().all(|(idx, c)| grid.get(p + d * idx as isize) == Some(&c))
        })
        .count()
}

fn count_x_mas(grid: &Grid<char>, p: Point) -> usize {
    [[Point(-1, -1), Point(0, 0), Point(1, 1)], [Point(-1, 1), Point(0, 0), Point(1, -1)]]
        .into_iter()
        .map(|d| d.into_iter().filter_map(|dv| grid.get(p + dv).copied()).collect::<String>())
        // For each of the two diagonals, have to be either "MAS" or its reverse.
        .all(|d| matches!(d.as_str(), "MAS" | "SAM")) as usize
}

fn main() {
    let grid: Grid<char> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<_>>()
        .into();

    let xmas: usize = iproduct!(0..grid.h, 0..grid.w).map(|p| count_xmas(&grid, p.into())).sum();
    let x_mas: usize = iproduct!(0..grid.h, 0..grid.w).map(|p| count_x_mas(&grid, p.into())).sum();

    println!("{xmas},{x_mas}");
}
