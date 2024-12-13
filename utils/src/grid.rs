#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x as isize, y as isize)
    }
}

pub enum Direction {}

impl Direction {
    pub const UP: Point = Point(-1, 0);
    pub const DOWN: Point = Point(1, 0);
    pub const RIGHT: Point = Point(0, 1);
    pub const LEFT: Point = Point(0, -1);
}

macro_rules! impl_add {
    ($tr:ident, $fn:ident, $op:tt, $lhs:ty, $rhs:ty, $ot:ident) => {
        impl std::ops::$tr<$rhs> for $lhs {
                    type Output = $ot;

                    fn $fn(self, rhs: $rhs) -> Self::Output {
                        $ot::new(self.0 $op rhs.0, self.1 $op rhs.1)
                    }
                }
    };
}

macro_rules! impl_add_x {
    ($tr:ident, $fn:ident, $op:tt) => {
        impl_add! {$tr, $fn, $op, Point, Point, Point}
        impl_add! {$tr, $fn, $op, Point, &Point, Point}
        impl_add! {$tr, $fn, $op, &Point, Point, Point}
        impl_add! {$tr, $fn, $op, &Point, &Point, Point}
    };
}

impl_add_x! {Add, add, +}
impl_add_x! {Sub, sub, -}

macro_rules! impl_mul {
    ($tr:ident, $fn:ident, $op:tt, $lhs:ty, $rhs:ty, $ot:ident) => {
        impl std::ops::$tr<$rhs> for $lhs {
                    type Output = $ot;

                    fn $fn(self, rhs: $rhs) -> Self::Output {
                        $ot::new(self.0 $op rhs, self.1 $op rhs)
                    }
                }
    };
}

macro_rules! impl_mul_x {
    ($tr:ident, $fn:ident, $op:tt) => {
        impl_mul! {$tr, $fn, $op, Point, isize, Point}
        impl_mul! {$tr, $fn, $op, Point, &isize, Point}
        impl_mul! {$tr, $fn, $op, &Point, isize, Point}
        impl_mul! {$tr, $fn, $op, &Point, &isize, Point}
    };
}

impl_mul_x! {Mul, mul, *}
impl_mul_x! {Div, div, /}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub h: usize,
    pub w: usize,
    pub buf: Vec<Vec<T>>,
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(fill: T, height: usize, width: usize) -> Self {
        Self::from(vec![vec![fill; width]; height])
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(buf: Vec<Vec<T>>) -> Self {
        let h = buf.len();
        let w = buf.first().map(|e| e.len()).unwrap_or(0);
        assert!(buf.iter().all(|e| e.len() == w));
        Self {
            h: h,
            w: w,
            buf: buf,
        }
    }
}

impl<T> Grid<T> {
    pub fn get<P: Into<Point>>(&self, point: P) -> Option<&T> {
        let Point(x, y) = point.into();
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        self.buf.get(x)?.get(y)
    }

    pub fn get_mut<P: Into<Point>>(&mut self, point: P) -> Option<&mut T> {
        let Point(x, y) = point.into();
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        self.buf.get_mut(x)?.get_mut(y)
    }
}

impl<T, P: Into<Point>> std::ops::Index<P> for Grid<T> {
    type Output = T;

    fn index(&self, point: P) -> &Self::Output {
        self.get(point.into()).unwrap()
    }
}

impl<T, P: Into<Point>> std::ops::IndexMut<P> for Grid<T> {
    fn index_mut(&mut self, point: P) -> &mut Self::Output {
        self.get_mut(point.into()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        assert_eq!(Point(1, 2) + Point(3, 4), Point(4, 6));
        assert_eq!(Point(1, 2) - Point(3, 4), Point(-2, -2));
    }

    #[test]
    fn test_point_mul() {
        assert_eq!(Point(1, 2) * -1, Point(-1, -2));
        assert_eq!(Point(12, 18) / 4, Point(3, 4));
    }
}
