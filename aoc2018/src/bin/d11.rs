use itertools::iproduct;

fn power_level((x, y): (i32, i32), serial: i32) -> i32 {
    (((x + 10) * y + serial) * (x + 10) / 100) % 10 - 5
}

type Grid = [[i32; 301]; 301];

fn prefix_sum(serial: i32) -> Grid {
    let mut s: Grid = [[0; 301]; 301];
    for (x, y) in iproduct!(1..s.len(), 1..s.len()) {
        s[x][y] =
            s[x - 1][y] + s[x][y - 1] - s[x - 1][y - 1] + power_level((x as i32, y as i32), serial);
    }
    s
}

fn rectangle_sum(psum: &Grid, p1: (usize, usize), p2: (usize, usize)) -> i32 {
    psum[p2.0][p2.1] - psum[p2.0][p1.1] - psum[p1.0][p2.1] + psum[p1.0][p1.1]
}

fn find_max_square_k(psum: &Grid, k: usize) -> (i32, (usize, usize)) {
    iproduct!(0..psum.len() - k, 0..psum.len() - k)
        .map(|(x, y)| (rectangle_sum(psum, (x, y), (x + k, y + k)), (x + 1, y + 1)))
        .max()
        .unwrap()
}

fn find_max_square(psum: &Grid) -> (i32, (usize, usize, usize)) {
    (0..psum.len())
        .map(|k| {
            let (p, (x, y)) = find_max_square_k(psum, k);
            (p, (x, y, k))
        })
        .max()
        .unwrap()
}

fn main() {
    let psum = prefix_sum(8561);
    let ans1 = find_max_square_k(&psum, 3).1;
    let ans2 = find_max_square(&psum).1;
    println!("{},{}", ans1.0, ans1.1);
    println!("{},{},{}", ans2.0, ans2.1, ans2.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_power() {
        assert_eq!(power_level((3, 5), 8), 4);
    }

    #[test]
    fn test_sample_42() {
        let psum = prefix_sum(42);
        assert_eq!(find_max_square_k(&psum, 3), (30, (21, 61)));
        assert_eq!(find_max_square(&psum), (119, (232, 251, 12)));
    }

    #[test]
    fn test_sample_18() {
        let psum = prefix_sum(18);
        assert_eq!(find_max_square(&psum), (113, (90, 269, 16)));
    }
}
