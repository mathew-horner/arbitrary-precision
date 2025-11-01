//! Convert from fixed-precision numbers to arbitrary-precision [`Integer`]s.

use std::ops::{DivAssign, Rem};

use super::*;

impl<N> From<N> for Integer
where
    N: Copy + DivAssign + From<u8> + PartialOrd + Rem<Output = N>,
    u8: TryFrom<N>,
{
    fn from(mut value: N) -> Self {
        let zero = N::from(0);
        let ten = N::from(10);
        let mut digits = Vec::new();
        while value > zero {
            // A number in the range [0, 10) will always be produced, so u8::try_from(...) is infallible here.
            let Ok(rem) = u8::try_from(value % ten) else {
                unreachable!();
            };
            digits.push(Digit(rem));
            value /= ten;
        }
        Integer(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8() {
        let integer = Integer::from(u8::MAX);
        let expected = Integer::from_iter([5, 5, 2].into_iter());
        assert_eq!(integer, expected);
    }

    #[test]
    fn from_u16() {
        let integer = Integer::from(u16::MAX);
        let expected = Integer::from_iter([5, 3, 5, 5, 6].into_iter());
        assert_eq!(integer, expected);
    }

    #[test]
    fn from_u32() {
        let integer = Integer::from(u32::MAX);
        let expected = Integer::from_iter([5, 9, 2, 7, 6, 9, 4, 9, 2, 4].into_iter());
        assert_eq!(integer, expected);
    }

    #[test]
    fn from_u64() {
        let integer = Integer::from(u64::MAX);
        let expected = Integer::from_iter(
            [5, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1].into_iter(),
        );
        assert_eq!(integer, expected);
    }

    #[test]
    fn from_u128() {
        let integer = Integer::from(u128::MAX);
        let expected = Integer::from_iter(
            [
                5, 5, 4, 1, 1, 2, 8, 6, 7, 1, 3, 4, 7, 0, 6, 4, 7, 3, 3, 6, 4, 3, 6, 4, 8, 3, 9, 0,
                2, 9, 6, 6, 3, 2, 8, 2, 0, 4, 3,
            ]
            .into_iter(),
        );
        assert_eq!(integer, expected);
    }
}
