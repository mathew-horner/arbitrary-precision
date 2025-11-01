#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Digit(pub(crate) u8);

impl Digit {
    /// Add two digits and return the least significant digit of the sum and
    /// carry.
    pub fn plus(self, other: Digit) -> (Digit, Digit) {
        let mut sum = self.0 + other.0;
        let mut carry = 0;
        if sum >= 10 {
            sum -= 10;
            carry = 1;
        }
        (Digit(sum), Digit(carry))
    }

    /// Multiply two digits and return the least significant digit of the
    /// product and carry.
    pub fn times(self, other: Digit) -> (Digit, Digit) {
        let prod = self.0 * other.0;
        let digit = prod % 10;
        let carry = prod / 10;
        (Digit(digit), Digit(carry))
    }
}

impl From<u8> for Digit {
    fn from(value: u8) -> Self {
        if value >= 10 {
            panic!("{value} is not a digit");
        }
        Self(value)
    }
}
