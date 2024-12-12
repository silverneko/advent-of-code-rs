#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
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
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn get(&self, Point(x, y): Point) -> Option<&T> {
        self.0
            .get(usize::try_from(x).ok()?)?
            .get(usize::try_from(y).ok()?)
    }
}

impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, point: Point) -> &Self::Output {
        self.get(point).unwrap()
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        self.0.index(idx)
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
