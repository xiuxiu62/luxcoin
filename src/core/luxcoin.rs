use std::{
    fmt::{self, Display},
    iter::Sum,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Luxcoin(i64);

impl Luxcoin {
    pub fn new(amount: i64) -> Self {
        Self(amount)
    }
}

impl Add for Luxcoin {
    type Output = Luxcoin;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Luxcoin {
    type Output = Luxcoin;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sum<Luxcoin> for Luxcoin {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Self::new(0);
        iter.for_each(|lc| sum = sum + lc);
        sum
    }
}

impl From<i64> for Luxcoin {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<i32> for Luxcoin {
    fn from(value: i32) -> Self {
        Self::new(value as i64)
    }
}

impl Display for Luxcoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} LUX", self.0)
    }
}
