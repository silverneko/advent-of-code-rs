use itertools::iproduct;
use std::io::stdin;
use std::ops::Range;
use utils::{Grid, Intcode};

fn plot(program: &Intcode, xs: Range<usize>, ys: Range<usize>) {
    let mut grid = Grid::new(' ', ys.end - ys.start, xs.end - xs.start);
    for (x, y) in iproduct!(xs.clone(), ys.clone()) {
        grid[(y - ys.start, x - xs.start)] = if is_pulled(program, x, y) { '#' } else { '.' };
    }
    println!("Plot ({xs:?}) x ({ys:?}):");
    println!("{grid}");
}

fn is_pulled(program: &Intcode, x: usize, y: usize) -> bool {
    match program.clone().run([x as isize, y as isize]).next().unwrap() {
        0 => false,
        1 => true,
        err => panic!("Unexpected output {err}"),
    }
}

fn part1(program: &Intcode) -> usize {
    iproduct!(0..50, 0..50).filter(|&(x, y)| is_pulled(program, x, y)).count()
}

fn part2(program: &Intcode, x: usize, y: usize) -> usize {
    // plot(program, x..x + 10, y..y + 10);
    let test_x = (0..100).all(|d| is_pulled(program, x + d, y));
    let test_y = (0..100).all(|d| is_pulled(program, x, y + d));
    if test_x && test_y {
        return dbg!(x * 10000 + y);
    }
    let move_x = !test_y && is_pulled(program, x + 1, y);
    let move_y = !test_x && is_pulled(program, x, y + 1);
    assert!(move_x || move_y);
    part2(program, x + move_x as usize, y + move_y as usize)
}

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    plot(&program, 120..140, 100..120);
    dbg!(part1(&program), part2(&program, 120, 100));
}
