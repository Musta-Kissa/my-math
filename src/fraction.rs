use std::cmp::PartialEq;
use std::cmp::{self, Ordering};
use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::Copy;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[macro_export]
macro_rules! fr {
    ($num:expr,$den:expr) => {
        MyFraction::new($num, $den)
    };
}

fn gcd(mut n:i128,mut d:i128) -> i128 {
    assert_ne!(d,0,"dvision by zero in gcd");
    n = n.abs();
    d = d.abs();
    while n != 0 {
    if n < d {
      std::mem::swap(&mut n, &mut d);
    }
    n %= d;
  }
  d
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct MyFraction {
    numerator: i128,
    denominator: i128,
}
impl MyFraction {
    pub fn new(num: i128, den: i128) -> Self {
        assert_ne!(den, 0);
        Self {
            numerator: num,
            denominator: den,
        }
        .simplify()
    }
    fn simplify(mut self) -> Self {
        let max_factor: i128 = gcd(self.numerator, self.denominator);

        if self.denominator.is_negative() {
            self.numerator *= -1;
            self.denominator *= -1;
        }
        self.numerator /= max_factor;
        self.denominator /= max_factor;
        assert_ne!(self.denominator, 0);
        self
    }
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
    pub fn as_mixed(&self) -> String {
        let rem = self.numerator % self.denominator;
        if self.denominator == 1 {
            return format!("{}", self.numerator);
        }
        if self.numerator < self.denominator {
            return format!("({}/{})", self.numerator, self.denominator);
        }
        let whole_part = (self.numerator - rem) / self.denominator;
        format!("{whole_part}({rem}/{})", self.denominator)
    }
}

//==========================================================//

impl Sub<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn sub(self, rhs: MyFraction) -> Self::Output {
        if self.denominator == rhs.denominator {
            return Self {
                numerator: self.numerator - rhs.numerator,
                denominator: self.denominator,
            }
            .simplify();
        } else {
            Self {
                numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
            }
            .simplify()
        }
    }
}
macro_rules! implSub {
    ($($type:ty),*) => { $(
        impl Sub<$type> for MyFraction {
            type Output = MyFraction;

            fn sub(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator - rhs as i128 * self.denominator,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Sub<MyFraction> for $type {
            type Output = MyFraction;

            fn sub(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: self as i128 * rhs.denominator - rhs.numerator,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implSub!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Add<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn add(self, rhs: MyFraction) -> Self::Output {
        if self.denominator == rhs.denominator {
            return Self {
                numerator: self.numerator + rhs.numerator,
                denominator: self.denominator,
            }
            .simplify();
        } else {
            Self {
                numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                denominator: self.denominator * rhs.denominator,
            }
            .simplify()
        }
    }
}
macro_rules! implAdd {
    ($($type:ty),*) => { $(
        impl Add<$type> for MyFraction {
            type Output = MyFraction;

            fn add(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator + rhs as i128 * self.denominator,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Add<MyFraction> for $type {
            type Output = MyFraction;

            fn add(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: self as i128 * rhs.denominator + rhs.numerator,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implAdd!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Mul<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn mul(self, rhs: MyFraction) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .simplify()
    }
}
macro_rules! implMul {
    ($($type:ty),*) => { $(
        impl Mul<$type> for MyFraction {
            type Output = MyFraction;

            fn mul(self, rhs: $type) -> Self::Output {
                Self {
                    numerator: self.numerator * rhs as i128,
                    denominator: self.denominator,
                }
                .simplify()
            }
        }
        impl Mul<MyFraction> for $type {
            type Output = MyFraction;

            fn mul(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: rhs.numerator * self as i128,
                    denominator: rhs.denominator,
                }
                .simplify()
            }
        }
        )*
    };
}
implMul!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Div<MyFraction> for MyFraction {
    type Output = MyFraction;

    fn div(self, rhs: MyFraction) -> Self::Output {
        assert_ne!(self.denominator * rhs.numerator, 0);
        Self {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
        .simplify()
    }
}
macro_rules! implDiv {
    ($($type:ty),*) => { $(
        impl Div<MyFraction> for $type {
            type Output = MyFraction;

            fn div(self, rhs: MyFraction) -> Self::Output {
                MyFraction {
                    numerator: (self as i128) * rhs.denominator,
                    denominator: rhs.numerator,
                }
                .simplify()
            }
        }
        impl Div<$type> for MyFraction {
            type Output = MyFraction;

            fn div(self, rhs: $type) -> Self::Output {
                assert_ne!(self.denominator * rhs as i128, 0);
                Self {
                    numerator: self.numerator,
                    denominator: self.denominator * rhs as i128,
                }
                .simplify()
            }
        }
        )*
    };
}
implDiv!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl SubAssign for MyFraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl AddAssign for MyFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl MulAssign for MyFraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    } 
}
impl DivAssign for MyFraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    } 
}
macro_rules! implAssign {
    ($($type:ty),*) => { $(
        impl SubAssign<$type> for MyFraction {
            fn sub_assign(&mut self, rhs: $type) {
                *self = *self - rhs;
            }
        }
        impl AddAssign<$type> for MyFraction {
            fn add_assign(&mut self, rhs: $type) {
                *self = *self + rhs;
            }
        }
        impl MulAssign<$type> for MyFraction {
            fn mul_assign(&mut self, rhs: $type) {
                *self = *self * rhs;
            }
        }
        impl DivAssign<$type> for MyFraction {
            fn div_assign(&mut self, rhs: $type) {
                *self = *self / rhs;
            }
        }
        )*
    };
}
implAssign!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl PartialEq for MyFraction {
    fn eq(&self, other: &Self) -> bool {
        self.denominator == other.denominator && self.numerator == other.numerator
    }
}
impl PartialOrd for MyFraction {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MyFraction {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.denominator == other.denominator {
            return self.numerator.cmp(&other.numerator);
        } else {
            return (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator));
        }
    }
}
macro_rules! implCmp {
    ($($type:ty),*) => { $(
        impl PartialEq<$type> for MyFraction {
            fn eq(&self, other: &$type) -> bool {
                self.denominator == 1 && self.numerator == *other as i128
            }
        }
        impl PartialOrd<$type> for MyFraction {
            fn partial_cmp(&self, other: &$type) -> Option<Ordering> {
                if self.denominator == 1 {
                    return Some(self.numerator.cmp(&(*other as i128)));
                } else {
                    return Some(self.numerator.cmp(&(*other as i128 * self.denominator)));
                }
            }
        }

        impl PartialEq<MyFraction> for $type {
            fn eq(&self, other: &MyFraction) -> bool {
                other.denominator == 1 && other.numerator == *self as i128
            }
        }
        impl PartialOrd<MyFraction> for $type {
            fn partial_cmp(&self, other: &MyFraction) -> Option<Ordering> {
                if other.denominator == 1 {
                    return Some((*self as i128).cmp(&other.numerator));
                } else {
                    return Some(((*self as i128) * other.denominator).cmp(&other.numerator));
                }
            }
        }
        )*
    };
}
implCmp!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

macro_rules! implFrom {
    ($($type:ty),*) => { $(
        impl From<$type> for MyFraction {
            fn from(value: $type) -> Self {
                MyFraction {
                    numerator: value as i128,
                    denominator: 1,
                }
            }
        }
        )*
    };
}
implFrom!(usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

//==========================================================//

impl Display for MyFraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)?;
        } else {
            write!(f, "({}/{})", self.numerator, self.denominator)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_print {
        ($left:expr, $op:tt, $right:expr,$($type:ty),*) => {
            assert!($left $op $right, "{} {} {}", $left, stringify!($op), $right);
        };
    }

    #[test]
    fn test() {
        assert_eq!(fr!(10, 2), fr!(5, 1));
        assert_print!(fr!(152541421414141124124124, 152541421414141124124124),==,1,
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

        assert_print!(fr!(10, 2),==,fr!(10, 2),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(fr!(10, 2),>,fr!(10, 3),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(fr!(10, 3),<,fr!(10, 2),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

        assert_print!(fr!(10, 2),==,5,
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(fr!(10, 2),>,4,
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(fr!(10, 2),<,6,
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

        assert_print!(5,==,fr!(10,2),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(2,<,fr!(10, 3),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
        assert_print!(6,>,fr!(10, 3),
            usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

        assert_eq!(fr!(7, 2).as_mixed(), "3(1/2)".to_string());
        assert_eq!(fr!(6, 2).as_mixed(), "3".to_string());
        assert_eq!(fr!(1, 6).as_mixed(), "(1/6)".to_string());

        assert_eq!(fr!(1, 3).as_f64(), 1. / 3.);

        println!("{}", fr!(21231, 421).as_mixed());
    }
}

