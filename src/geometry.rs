//! This provides very basic support for 2D geometry.
//!
//! Since there is a large body of 2D l-system examples, this
//! module makes it much easier to implement interpretations
//! for these examples and see your output.

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub};

/// Represents a point in a 2D Euclidean space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }
}


/// A vector in 2-space. This represents a size and direction.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64
}

impl Vector {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    #[inline]
    pub fn up() -> Self {
        Vector::new(0.0, 1.0)
    }

    #[inline]
    pub fn zero() -> Self {
        Vector::new(0.0, 0.0)
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn rotate(&self, degrees: f64) -> Self {
        let cos = degrees.to_radians().cos();
        let sin = degrees.to_radians().sin();

        Vector::new(
            cos * self.x() - sin * self.y(),
            sin * self.x() + cos * self.y()
        )
    }

}

impl Default for Vector {
    #[inline]
    fn default() -> Self {
        Vector::zero()
    }
}



impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y())
    }
}


impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x();
        self.y += rhs.y();
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::Vector;

    #[test]
    fn testing_rotations() {
        let up = Vector::up();
        let one = up.rotate(90.0);

        assert_eq!(up, Vector::new(0.0, 1.0));

        assert!((one.x - -1.0).abs() < 0.001);
        assert!((one.y - 0.0).abs() < 0.001);
    }
}