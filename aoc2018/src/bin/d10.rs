use anyhow::Result;
use itertools::{chain, Itertools};
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

struct TestCase {
    points: Vec<(Point, Direction)>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Result<Self> {
        let points = reader
            .lines()
            .map(|e| {
                let s = e?;
                let t: Vec<_> = s.split(['<', ',', '>']).collect();
                let [x, y, dx, dy] =
                    [t[1], t[2], t[4], t[5]].map(|e| e.trim().parse::<isize>().unwrap());
                Ok((Point::new(x, -y).counter_rotate(), Direction::new(dx, -dy).counter_rotate()))
            })
            .collect::<Result<_>>()?;
        Ok(Self { points })
    }

    fn solve(&self) {
        let tx = self
            .points
            .iter()
            .tuple_combinations()
            .filter(|((_, d1), (_, d2))| d1.0 != d2.0)
            .map(|((p1, d1), (p2, d2))| (p1.0 - p2.0) / (d2.0 - d1.0));
        let ty = self
            .points
            .iter()
            .tuple_combinations()
            .filter(|((_, d1), (_, d2))| d1.1 != d2.1)
            .map(|((p1, d1), (p2, d2))| (p1.1 - p2.1) / (d2.1 - d1.1));
        let (_, t) = chain![tx, ty].counts().into_iter().map(|(t, f)| (f, t)).max().unwrap();
        let s: Vec<_> = self.points.iter().map(|(p, d)| p + d * t).collect();
        let (min_x, max_x) = s.iter().map(|p| p.0).minmax().into_option().unwrap();
        let (min_y, max_y) = s.iter().map(|p| p.1).minmax().into_option().unwrap();
        let mut grid = Grid::new(' ', (3 + max_x - min_x) as usize, (3 + max_y - min_y) as usize);
        for p in s {
            grid[p - Point(min_x, min_y) + Point(1, 1)] = '#';
        }
        println!("{t}\n{grid}");
    }
}

fn main() -> Result<()> {
    TestCase::parse(SAMPLE_INPUT.as_bytes())?.solve();
    TestCase::parse(stdin().lock())?.solve();
    Ok(())
}

const SAMPLE_INPUT: &str = r"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
"
.trim_ascii();
