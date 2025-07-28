use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Neg, Sub};

/// A minimal implementation of a 2-element vector for this project.
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Constructs a new vector with the given x and y values.
    pub fn new (x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}


/// Implements vector addition.
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}


/// Implements negation of vectors using the unary '-'.
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

/// Implements vector subtraction.
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

/// Implements multiplication of a vector with a scalar value.
impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2::new(
            self.x * rhs,
            self.y * rhs,
        )
    }
}

/// Implements multiplication of a scalar value with a vector.
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

/// Provides vector formatting and printing.
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
