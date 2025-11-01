//! Arithmetic operations for [`Integer`]s.

use std::ops::{Add, Mul};

use super::*;

impl Add for Integer {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let length = self.digits().max(rhs.digits());
        self.pad_to(length);
        rhs.pad_to(length);

        let mut carry = Digit(0);

        for idx in 0..length {
            let (sum, carry1) = self.0[idx].plus(rhs.0[idx]);
            let (sum, carry2) = sum.plus(carry);
            self.0[idx] = sum;
            carry = carry1.max(carry2);
        }

        if carry == Digit(1) {
            self.push(1);
        }

        self.trim_leading_zeros();
        self
    }
}

impl Mul for Integer {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut intermediates = Vec::with_capacity(rhs.digits());

        for i in 0..rhs.digits() {
            let mut intermediate = Integer::empty();
            for _ in 0..i {
                intermediate.push(Digit(0));
            }
            let mut carry = Digit(0);

            for j in 0..self.digits() {
                let (digit, carry1) = rhs.0[i].times(self.0[j]);
                let (digit, carry2) = digit.plus(carry);
                intermediate.push(digit);
                // ???: This will never yield a carry, right?
                carry = carry1.plus(carry2).0;
            }

            if carry != Digit(0) {
                intermediate.push(carry);
            }

            intermediates.push(intermediate);
        }

        intermediates
            .into_iter()
            .fold(Integer::zero(), |a, b| a + b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_iter_removes_leading_zeros() {
        let num = Integer::from_iter([0, 5, 1, 0, 0].into_iter());
        let expected = Integer::from_iter([0, 5, 1].into_iter());
        assert_eq!(num, expected);
    }

    #[test]
    fn add_no_carry() {
        let a = Integer::from_iter([3, 6].into_iter());
        let b = Integer::from_iter([6, 3].into_iter());
        let expected = Integer::from_iter([9, 9].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_with_carry() {
        let a = Integer::from_iter([4, 6].into_iter());
        let b = Integer::from_iter([6, 3].into_iter());
        let expected = Integer::from_iter([0, 0, 1].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_more_digits_lhs() {
        let a = Integer::from_iter([2, 9, 4, 5].into_iter());
        let b = Integer::from_iter([9, 9].into_iter());
        let expected = Integer::from_iter([1, 9, 5, 5].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn add_more_digits_rhs() {
        let a = Integer::from_iter([7, 8, 5].into_iter());
        let b = Integer::from_iter([4, 7, 0, 3, 9].into_iter());
        let expected = Integer::from_iter([1, 6, 6, 3, 9].into_iter());
        assert_eq!(a + b, expected);
    }

    #[test]
    fn mul() {
        let a = Integer::from_iter([8, 3, 6, 7].into_iter());
        let b = Integer::from_iter([4, 3, 2].into_iter());
        let expected = Integer::from_iter([2, 9, 2, 7, 8, 7, 1].into_iter());
        assert_eq!(a * b, expected);
    }

    #[test]
    fn pow_zero() {
        let a = Integer::from_iter([2].into_iter());
        let e = 0;
        let expected = Integer::from_iter([1].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_one() {
        let a = Integer::from_iter([2].into_iter());
        let e = 1;
        let expected = Integer::from_iter([2].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_two() {
        let a = Integer::from_iter([2].into_iter());
        let e = 2;
        let expected = Integer::from_iter([4].into_iter());
        assert_eq!(a.pow(e), expected);
    }

    #[test]
    fn pow_big() {
        let a = Integer::from_iter([2].into_iter());
        let e = 30;
        let expected = Integer::from_iter([4, 2, 8, 1, 4, 7, 3, 7, 0, 1].into_iter());
        assert_eq!(a.pow(e), expected);
    }
}
