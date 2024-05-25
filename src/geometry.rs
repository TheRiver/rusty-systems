//! This provides very basic support for 2D geometry.
//!
//! Since there is a large body of 2D l-system examples, this
//! module makes it much easier to implement interpretations
//! for these examples and see your output.
//!
//! Note that this is not meant to be a *complete* or *performant* implementation
//! of 2D geometry and transformations. It does not even have an implementation of
//! a transformation matrix — if you need transformation matrices, your needs have moved
//! beyond the ability of this crate, and you should look elsewhere. More full-featured
//! alternatives include [nalgebra][nalgebra], and if you specifically want
//! a computer graphics related package focusing on 2D and 3D operations using
//! homogenous coordinates, consider their [nalgebra-glm][nalgebra-glm] crate.
//!
//! [nalgebra]: https://nalgebra.org/
//! [nalgebra-glm]: https://nalgebra.org/docs/user_guide/nalgebra_glm

use std::fmt::{Display, Formatter};
use std::ops::{Add, Neg, Sub};

/// Represents an immutable point in 2-space. 
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
    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
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


/// An immutable vector in 2-space. This represents *size* and *direction*.
/// See [`Point`]
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
    pub fn down() -> Self {
        Vector::new(0.0, -1.0)
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

    /// Returns a vector rotated by the given degrees.
    ///
    /// Here is an example of how to make use of rotations. Note
    /// that rust has built in support for converting between degrees and radians.
    /// See [`f64::to_radians`] and [`f64::to_degrees`].
    ///
    /// ```
    /// use rusty_systems::geometry::Vector;
    /// let up = Vector::up();
    /// let left = up.rotate(90.0);
    ///
    /// assert!((up - Vector::new(0.0, 1.0)).norm() < 0.001);   // The up vector is (0, 1)
    /// assert!((left - Vector::new(-1.0, 0.0)).norm() < 0.001) // rotated by 90º, it points (-1, 0)
    /// ```
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

impl Default for Point {
    #[inline]
    fn default() -> Self {
        Point::zero()
    }
}

impl From<Point> for Vector {
    #[inline]
    fn from(value: Point) -> Self {
        Vector::new(value.x, value.y)
    }
}

impl From<Vector> for Point {
    #[inline]
    fn from(value: Vector) -> Self {
        Point::new(value.x, value.y)
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

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


pub struct Edge {
    pub from: Point,
    pub to: Point
}

impl Edge {
    #[inline]
    pub fn new(from: Point, to: Point) -> Self {
        Edge { from, to }
    }
    
    #[inline]
    pub fn zero() -> Self {
        Edge::new(Point::zero(), Point::zero())
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Path { points: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    #[inline]
    pub fn push<T: Into<Point>>(&mut self, point: T) {
        self.points.push(point.into())
    }
}

impl Default for Path {
    fn default() -> Self {
        Path::new()
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