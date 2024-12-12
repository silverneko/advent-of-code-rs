pub trait Gcd {
    fn gcd(self, rhs: Self) -> Self;
}

macro_rules! impl_gcd {
    ($($unsigned:tt $signed:tt),*) => {$(
        impl Gcd for $unsigned {
            fn gcd(self, b: Self) -> Self {
                if self == 0 {
                    b
                } else {
                    (b % self).gcd(self)
                }
            }
        }

        impl Gcd for $signed {
            fn gcd(self, b: Self) -> Self {
                self.unsigned_abs().gcd(b.unsigned_abs()) as Self
            }
        }
    )*};
}

impl_gcd! {usize isize, u32 i32, u64 i64}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_i32() {
        assert_eq!(1i32.gcd(13), 1);
        assert_eq!(24i32.gcd(13), 1);
        assert_eq!(40i32.gcd(15), 5);
        assert_eq!((-36i32).gcd(15), 3);
        assert_eq!((-360i32).gcd(-40), 40);
        assert_eq!(48i32.gcd(-64), 16);
    }
}
