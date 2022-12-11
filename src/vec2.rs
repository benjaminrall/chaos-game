use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Index, IndexMut};
use chaos_game::random_range;

/// Object to represent a column vector in 2 dimensions
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Returns a zero vector
    pub fn zero() -> Vec2 {
        Vec2 { x: 0., y: 0. }
    }

    /// Returns a vector with x and y 1
    pub fn one() -> Vec2 {
        Vec2 { x: 1., y: 1. }
    }

    /// Constructs a new vector with given x and y values
    pub fn new (x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    /// Constructs a vector with random components between a given range
    pub fn random(min: f64, max: f64) -> Vec2 {
        Vec2 {
            x: random_range(min, max),
            y: random_range(min, max),
        }
    }

    /// Returns a random vector in a unit circle
    pub fn random_in_unit_circle() -> Vec2 {
        loop {
            let p = Vec2::random(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    /// Returns a random unit vector
    pub fn random_unit_vector() -> Vec2 {
        Vec2::random_in_unit_circle().unit()
    }

    /// Returns the squared length of a vector
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length of a vector
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    /// Returns the dot product of two vectors
    pub fn dot(u: &Vec2, v: &Vec2) -> f64 {
        u.x * v.x + u.y * v.y
    }

    /// Returns the unit vector
    pub fn unit(self) -> Vec2 {
        self / self.length()
    }

    /// Returns true if all components of the vector are near zero
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s
    }
}

/// Allows vectors to be cloned directly by constructing a vector with the same x and y
impl Clone for Vec2 {
    fn clone(&self) -> Self {
        Vec2::new(self.x, self.y)
    }
}

impl Copy for Vec2 { }

/// Implements vector addition
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}

/// Implements vector addition with the += operator
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Implements negation of vectors using the unary '-'
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

/// Implements vector subtraction
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

/// Implements vector subtraction with the -= operator
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}

/// Implements element-wise multiplication of two vectors
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2::new(
            self.x * rhs.x,
            self.y * rhs.y,
        )
    }
}

/// Implements multiplication of a vector with a scalar value
impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2::new(
            self.x * rhs,
            self.y * rhs,
        )
    }
}

/// Implements multiplication of a scalar value with a vector
impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

/// Implements multiplication of a vector by a scalar value with the *= operator
impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Implements division of a vector by a scalar value
impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

/// Implements division of a vector by a scalar value with the /= operator
impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

/// Implements indexing a vector to access elements
impl Index<usize> for Vec2 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Attempted to access an out of range Vec3 index.")
        }
    }
}

/// Implements indexing a vector to modify elements
impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Attempted to access an out of range Vec3 index.")
        }
    }
}

/// Provides vector formatting and printing
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}