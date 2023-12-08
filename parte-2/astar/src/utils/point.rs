//! 2D Vector implementation

use std::ops::{Add, Neg};

/// 2D Vector
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Point<T>(pub T, pub T);

impl Add<Point<i8>> for Point<usize> {
    type Output = Option<Self>;
    fn add(self, other: Point<i8>) -> Self::Output {
        Some(Self(
            self.0.checked_add_signed(other.0.into())?,
            self.1.checked_add_signed(other.1.into())?,
        ))
    }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, -self.1)
    }
}
