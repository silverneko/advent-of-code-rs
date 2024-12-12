use itertools::Itertools;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

enum PatrolPath {
    Area(usize),
    Loop,
}

fn find_patrol_path(mut grid: Vec<Vec<u8>>) -> PatrolPath {
    let h = grid.len();
    let w = grid.first().unwrap().len();
    let (mut x, mut y) = (0..h)
        .cartesian_product(0..w)
        .find(|(x, y)| grid[*x][*y] == b'^')
        .expect("Cannot find '^' in input");

    let mut d = 0;
    loop {
        let (dx, dy) = DIRECTIONS[d];
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        let next_tile = (|| {
            grid.get(usize::try_from(nx).ok()?)?
                .get(usize::try_from(ny).ok()?)
        })();
        match next_tile {
            None => {
                grid[x][y] = d as u8;
                break;
            }
            Some(b'#') => {
                d = (d + 1) % 4;
            }
            Some(b'.' | b'^' | 0..4) => {
                if grid[x][y] == d as u8 {
                    return PatrolPath::Loop;
                }
                grid[x][y] = d as u8;
                (x, y) = (nx as usize, ny as usize);
            }
            _ => panic!(),
        }
    }
    PatrolPath::Area(
        grid.iter()
            .flatten()
            .filter(|b| match b {
                0..4 => true,
                _ => false,
            })
            .count(),
    )
}

fn main() {
    let canvas: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().into())
        .collect();

    let PatrolPath::Area(ans1) = find_patrol_path(canvas.clone()) else {
        panic!()
    };

    let h = canvas.len();
    let w = canvas.first().unwrap().len();
    let ans2 = (0..h)
        .cartesian_product(0..w)
        .filter(|(i, j)| canvas[*i][*j] == b'.')
        .map(|(i, j)| {
            let mut grid = canvas.clone();
            grid[i][j] = b'#';
            find_patrol_path(grid)
        })
        .filter(|patrol_path| match patrol_path {
            PatrolPath::Loop => true,
            _ => false,
        })
        .count();

    println!("{ans1},{ans2}");
}
