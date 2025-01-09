use itertools::iproduct;
use std::io::{stdin, BufRead};
use utils::{Direction, Point};

// (start, end, accu length w.r.t start)
type Segment = (Point, Point, usize);

struct TestCase {
    wirea: Vec<Segment>,
    wireb: Vec<Segment>,
}

impl TestCase {
    fn parse(reader: impl BufRead) -> Self {
        let lines = reader.lines().map(|e| e.unwrap());
        let mut wires = lines.take(2).map(|e| {
            e.split(',')
                .scan((Point(0, 0), 0), |st, x| {
                    let (u, d) = *st;
                    let n = x[1..].parse::<isize>().unwrap();
                    let v = u + match x.chars().nth(0).unwrap() {
                        'R' => Direction::from((n, 0)),
                        'U' => Direction::from((0, n)),
                        'L' => Direction::from((-n, 0)),
                        'D' => Direction::from((0, -n)),
                        d => panic!("Unexpected direction {d}"),
                    };
                    *st = (v, d + n as usize);
                    Some((u, v, d))
                })
                .collect::<Vec<Segment>>()
        });
        Self { wirea: wires.next().unwrap(), wireb: wires.next().unwrap() }
    }

    fn solve(&self) -> (usize, usize) {
        let (inter, dist): (Vec<Point>, Vec<usize>) =
            iproduct!(self.wirea.iter(), self.wireb.iter())
                .filter_map(|((p, q, d1), (r, s, d2))| {
                    if cross(p - s, r - s) * cross(q - s, r - s) < 0
                        && cross(r - q, p - q) * cross(s - q, p - q) < 0
                    {
                        let x = if p.0 == q.0 { p.0 } else { r.0 };
                        let y = if p.1 == q.1 { p.1 } else { r.1 };
                        let i = Point(x, y);
                        let d = d1 + l1_norm(i - p) + d2 + l1_norm(i - r);
                        Some((i, d))
                    } else {
                        None
                    }
                })
                .unzip();
        (inter.into_iter().map(l1_norm).min().unwrap(), dist.into_iter().min().unwrap())
    }
}

fn cross(a: Point, b: Point) -> isize {
    a.0 * b.1 - a.1 * b.0
}

fn l1_norm(a: Point) -> usize {
    a.0.unsigned_abs() + a.1.unsigned_abs()
}

fn main() {
    let data = TestCase::parse(stdin().lock());
    dbg!(data.solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample0() {
        let input = "R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(TestCase::parse(input.as_bytes()).solve(), (6, 30));
    }

    #[test]
    fn test_sample1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(TestCase::parse(input.as_bytes()).solve(), (159, 610));
    }

    #[test]
    fn test_sample2() {
        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(TestCase::parse(input.as_bytes()).solve(), (135, 410));
    }
}
