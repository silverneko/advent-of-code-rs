use std::collections::VecDeque;

fn bfs(grid: &Vec<Vec<u8>>, s: (usize, usize)) -> (u32, u32) {
    let h = grid.len();
    let w = grid.first().unwrap().len();

    let mut q = VecDeque::from([s]);
    let mut visited: Vec<Vec<u32>> = vec![vec![0; w]; h];
    visited[s.0][s.1] = 1;
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let tx = x.checked_add_signed(i);
            let ty = y.checked_add_signed(j);
            if let (Some(tx), Some(ty)) = (tx, ty) {
                if let Some(&tt) = (|| grid.get(tx)?.get(ty))() {
                    if tt == grid[x][y] + 1 {
                        if visited[tx][ty] == 0 {
                            q.push_back((tx, ty));
                        }
                        visited[tx][ty] += visited[x][y];
                    }
                }
            }
        }
    }
    /*
    let mut cc = 0;
    let mut rating = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'9' && visited[i][j] > 0 {
                cc += 1;
                rating += visited[i][j];
            }
        }
    }
    */
    let f = grid
        .iter()
        .flatten()
        .zip(visited.iter().flatten())
        .filter(|(&t, &r)| t == b'9' && r > 0);
    let cc = f.clone().count() as u32;
    let rating = f.fold(0, |acc, (_, &r)| acc + r);
    (cc, rating)
}

fn main() {
    let grid: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().into())
        .collect();
    let h = grid.len();
    let w = grid.first().unwrap().len();

    let mut ans1 = 0;
    let mut ans2 = 0;

    // part 1
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'0' {
                let aa = bfs(&grid, (i, j));
                ans1 += aa.0;
                ans2 += aa.1;
            }
        }
    }

    println!("{ans1},{ans2}");
}
