use itertools::Itertools;
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

#[derive(Clone)]
struct TestCase {
    grid: Grid<u8>,
    moves: Vec<Point>,
}

fn parse_input<R: BufRead>(reader: R) -> TestCase {
    let mut lines = reader.lines().map(|e| e.unwrap());
    let grid: Grid<u8> = lines
        .by_ref()
        .take_while(|e| !e.is_empty())
        .map(|e| e.into())
        .collect::<Vec<Vec<u8>>>()
        .into();
    let moves = lines
        .take_while(|e| !e.is_empty())
        .flat_map(|e| e.into_bytes())
        .map(|b| match b {
            b'^' => Direction::UP,
            b'v' => Direction::DOWN,
            b'>' => Direction::RIGHT,
            b'<' => Direction::LEFT,
            _ => panic!("unexpected direction: {b:?}"),
        })
        .collect();
    TestCase {
        grid: grid,
        moves: moves,
    }
}

impl TestCase {
    fn upscale(mut self) -> Self {
        self.grid = self
            .grid
            .buf
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|b| match b {
                        b'#' => [b'#', b'#'],
                        b'.' => [b'.', b'.'],
                        b'O' => [b'[', b']'],
                        b'@' => [b'@', b'.'],
                        t @ _ => panic!("unexpected token: {t:?}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into();
        self
    }
}

fn try_move(s: Point, d: Point, grid: &mut Grid<u8>) -> bool {
    let n = s + d;
    let can_move = match grid[n] {
        b'#' => false,
        b'.' => true,
        b'O' => try_move(n, d, grid),
        b'[' => try_move(n + Point(0, 1), d, grid) && try_move(n, d, grid),
        b']' => try_move(n + Point(0, -1), d, grid) && try_move(n, d, grid),
        t @ _ => panic!("unexpected token: {t:?}"),
    };
    if can_move {
        grid[n] = grid[s];
        grid[s] = b'.';
    }
    can_move
}

fn solve(data: TestCase) -> u64 {
    let TestCase { mut grid, moves } = data;
    println!("Initial state:\n{grid}");

    let mut s: Point = (0..grid.h)
        .cartesian_product(0..grid.w)
        .find(|&p| grid[p] == b'@')
        .unwrap()
        .into();

    for &d in moves.iter() {
        let mut try_grid = grid.clone();
        if try_move(s, d, &mut try_grid) {
            grid = try_grid;
            s = s + d;
        }
        // DEBUG - print small samples only
        if moves.len() < 30 {
            println!("Move {d:?}:\n{grid}");
        }
    }

    (0..grid.h)
        .cartesian_product(0..grid.w)
        .filter(|&p| match grid[p] {
            b'O' | b'[' => true,
            _ => false,
        })
        .map(|(i, j)| (i * 100 + j) as u64)
        .sum()
}

fn main() {
    let test_case = parse_input(stdin().lock());
    println!(
        "{:?},{:?}",
        solve(test_case.clone()),
        solve(test_case.upscale())
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sample_small() {
        let input = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
        .trim();
        let reader = Cursor::new(input);
        assert_eq!(solve(parse_input(reader)), 2028);
    }

    #[test]
    fn test_sample_large() {
        let input = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        .trim();
        let reader = Cursor::new(input);
        assert_eq!(solve(parse_input(reader.clone())), 10092);
        assert_eq!(solve(parse_input(reader).upscale()), 9021);
    }

    #[test]
    fn test_sample_wide() {
        let input = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"
        .trim();
        let reader = Cursor::new(input);
        assert_eq!(solve(parse_input(reader).upscale()), 618);
    }
}
