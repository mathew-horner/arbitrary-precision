use std::fmt;

use crate::Digit;

mod conversions;
mod ops;

/// Integer with an arbitrary number of digits
///
/// Digits are stored in least significant first order. For example:
/// `Integer(vec![0, 0, 1])` is equivalent to `100`
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Integer(Vec<Digit>);

impl Integer {
    pub fn from_iter(iter: impl Iterator<Item = impl Into<Digit>>) -> Self {
        let mut num = Self(iter.map(Into::into).collect());
        num.trim_leading_zeros();
        num
    }

    pub fn digits(&self) -> usize {
        self.0.len()
    }

    pub fn zero() -> Self {
        Self(vec![Digit(0)])
    }

    pub fn one() -> Self {
        Self(vec![Digit(1)])
    }

    pub fn pow(self, exponent: usize) -> Integer {
        if exponent == 0 {
            return Integer::one();
        }
        let mut power = self.clone();
        for _ in 1..exponent {
            power = power * self.clone();
        }
        power
    }

    fn empty() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, digit: impl Into<Digit>) {
        self.0.push(digit.into());
    }

    fn pad_to(&mut self, length: usize) {
        for _ in 0..length - self.digits() {
            self.0.push(Digit(0));
        }
    }

    fn trim_leading_zeros(&mut self) {
        while self.0.last() == Some(&Digit(0)) {
            self.0.pop();
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, digit) in self.0.iter().enumerate().rev() {
            write!(f, "{}", digit.0)?;
            if idx != 0 && idx % 3 == 0 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}
