use itertools::iproduct;
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

#[derive(Clone)]
struct TestCase {
    grid: Grid<char>,
    moves: Vec<Point>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let mut lines = reader.lines().map(|e| e.unwrap());
        let grid = lines
            .by_ref()
            .take_while(|e| !e.is_empty())
            .map(|e| e.chars().collect())
            .collect::<Vec<_>>()
            .into();
        let moves = lines
            .take_while(|e| !e.is_empty())
            .flat_map(|e| e.chars().collect::<Vec<_>>())
            .map(|b| match b {
                '^' => Direction::UP,
                'v' => Direction::DOWN,
                '>' => Direction::RIGHT,
                '<' => Direction::LEFT,
                t => panic!("unexpected direction: {t:?}"),
            })
            .collect();
        Self { grid, moves }
    }

    fn upscale(&self) -> Self {
        let grid = self
            .grid
            .buf
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|&b| match b {
                        '#' => ['#', '#'],
                        '.' => ['.', '.'],
                        'O' => ['[', ']'],
                        '@' => ['@', '.'],
                        t => panic!("unexpected token: {t:?}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into();
        Self { grid, moves: self.moves.clone() }
    }
}

fn try_move(s: Point, d: Direction, grid: &mut Grid<char>) -> bool {
    let n = s + d;
    let can_move = match grid[n] {
        '#' => false,
        '.' => true,
        'O' => try_move(n, d, grid),
        '[' => try_move(n + Direction::RIGHT, d, grid) && try_move(n, d, grid),
        ']' => try_move(n + Direction::LEFT, d, grid) && try_move(n, d, grid),
        t => panic!("unexpected token: {t:?}"),
    };
    if can_move {
        grid[n] = grid[s];
        grid[s] = '.';
    }
    can_move
}

fn solve(data: &TestCase) -> u64 {
    let TestCase { grid, moves } = data;
    println!("Initial state:\n{grid}");

    let s: Point = iproduct!(0..grid.h, 0..grid.w).find(|&p| grid[p] == '@').unwrap().into();
    let (grid, _) = moves.iter().fold((grid.clone(), s), |(grid, s), &d| {
        let mut try_grid = grid.clone();
        let (grid, s) = if try_move(s, d, &mut try_grid) { (try_grid, s + d) } else { (grid, s) };
        // DEBUG - print small samples only
        if moves.len() < 30 {
            println!("Move {d:?}:\n{grid}");
        }
        (grid, s)
    });
    println!("End state:\n{grid}");
    iproduct!(0..grid.h, 0..grid.w)
        .filter(|&p| matches!(grid[p], 'O' | '['))
        .map(|(i, j)| (i * 100 + j) as u64)
        .sum()
}

fn main() {
    let test_case = TestCase::parse(stdin().lock());
    println!("{:?},{:?}", solve(&test_case), solve(&test_case.upscale()));
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(solve(&TestCase::parse(input.as_bytes())), 2028);
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
        assert_eq!(solve(&TestCase::parse(input.as_bytes())), 10092);
        assert_eq!(solve(&TestCase::parse(input.as_bytes()).upscale()), 9021);
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
        assert_eq!(solve(&TestCase::parse(input.as_bytes()).upscale()), 618);
    }
}
