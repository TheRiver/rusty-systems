//! This provides very basic support for 2D geometry.
//!
//! Since there is a large body of 2D l-system examples, this
//! module makes it much easier to implement interpretations
//! for these examples and see your output.
//!
//! 
//! <div class="warning">
//! 
//! Note that this is not meant to be a *complete* or *performant* implementation
//! of 2D geometry and transformations. It does not even have an implementation of
//! a transformation matrix — if you need transformation matrices, your needs have moved
//! beyond the ability of this crate, and you should look elsewhere. More full-featured
//! alternatives include [nalgebra][nalgebra], and if you specifically want
//! a computer graphics related package focusing on 2D and 3D operations using
//! homogenous coordinates, consider their [nalgebra-glm][nalgebra-glm] crate.
//! 
//! </div>
//!
//! [nalgebra]: https://nalgebra.org/
//! [nalgebra-glm]: https://nalgebra.org/docs/user_guide/nalgebra_glm

use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
use std::ops::{Add, Div, Index, Neg, Sub};
use std::slice::Iter;
use std::vec::IntoIter;

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

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x() / rhs, self.y() / rhs)
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

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// A path is a sequence of points. These can represent
/// a line.
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

    #[inline]
    pub fn get(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, Point> {
        self.points.iter()
    }

    /// Returns the starting point of the path.
    #[inline]
    pub fn get_start(&self) -> Option<&Point> {
        self.points.first()
    }
}

impl Index<usize> for Path {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl Default for Path {
    fn default() -> Self {
        Path::new()
    }
}

impl IntoIterator for Path {
    type Item = Point;
    type IntoIter = IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

/// Create a [`Path`] from a collection of [`Point`] objects.
impl FromIterator<Point> for Path {
    fn from_iter<T: IntoIterator<Item=Point>>(iter: T) -> Self {
        Path { points: iter.into_iter().collect() }
    }
}

/// Offset all points in the path by the given vector.
impl Add<Vector> for Path {
    type Output = Path;

    fn add(self, rhs: Vector) -> Self::Output {
        self.into_iter().map(|p| p + rhs).collect()
    }
}

/// The bounds of a geometric object, defined as a square. The object is contained in these bounds.
#[derive(Debug, Clone, Default)]
pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    /// The "center of mass" of object.
    pub com: Point
}

impl BoundingBox {
    #[inline]
    pub fn width(&self) -> f64 {
        (self.max_x - self.min_x).abs()
    }

    #[inline]
    pub fn height(&self) -> f64 {
        (self.max_y - self.min_y).abs()
    }

    /// The center of the box.
    #[inline]
    pub fn center(&self) -> Point {
        Point::new((self.max_x + self.min_x) / 2.0, (self.max_y + self.min_y) / 2.0)
    }

    /// Returns a bounding box set up to have its values updated.
    fn initial_infinite() -> Self {
        BoundingBox {
            min_x: f64::INFINITY,
            max_x: f64::NEG_INFINITY,
            min_y: f64::INFINITY,
            max_y: f64::NEG_INFINITY,
            ..Default::default()
        }
    }

    /// Returns a bounding box that only contains 0.
    ///
    /// This is a synonym for [`BoundingBox::default`].
    pub fn zero() -> Self {
        Self::default()
    }
}

/// Represents an item that potentially has bounds.
///
/// Note that empty collections, such as a [`Path`] that
/// contains no points, will return [`None`].
pub trait Bounds {
    fn bounds(&self) -> Option<BoundingBox>;
}

impl Bounds for Path {
    /// Returns the bounds for the path.

    fn bounds(&self) -> Option<BoundingBox> {
        let mut bounds = BoundingBox::initial_infinite();

        let mut center = Point::zero();

        for point in &self.points {
            center = center + *point;

            if point.x < bounds.min_x {
                bounds.min_x = point.x;
            }
            if point.x > bounds.max_x {
                bounds.max_x = point.x;
            }
            if point.y < bounds.min_y {
                bounds.min_y = point.y;
            }
            if point.y > bounds.max_y {
                bounds.max_y = point.y;
            }
        }

        bounds.com = center / self.len() as f64;

        Some(bounds)
    }
}

impl Bounds for Vec<Path> {
    fn bounds(&self) -> Option<BoundingBox> {
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        let mut center = Point::zero();
        let mut len = 0_f64;

        for path in self {
            len += path.len() as f64;

            for point in &path.points {
                center = center + *point;

                if point.x < min_x {
                    min_x = point.x;
                }
                if point.x > max_x {
                    max_x = point.x;
                }
                if point.y < min_y {
                    min_y = point.y;
                }
                if point.y > max_y {
                    max_y = point.y;
                }
            }
        }

        if min_x.is_infinite() {
            return None
        }

        Some(BoundingBox {
            min_x,
            max_x,
            min_y,
            max_y,
            com: center / len
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_rotations() {
        let up = Vector::up();
        let one = up.rotate(90.0);

        assert_eq!(up, Vector::new(0.0, 1.0));

        assert!((one.x - -1.0).abs() < 0.001);
        assert!((one.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn bounding_box() {
        assert_eq!(BoundingBox::zero().width(), 0.0);
        assert_eq!(BoundingBox::zero().height(), 0.0);
        assert_eq!(BoundingBox { min_y: -12.0, max_y: 100.0, ..Default::default()}.height(), 112.0);
        assert_eq!(BoundingBox { min_x: -10.0, max_x: 100.0, ..Default::default()}.width(), 110.0);

        let b = BoundingBox { min_x: -22.0, max_x:-2.0, min_y: 100.0, max_y: 101.0, ..BoundingBox::default()};
        assert_eq!(b.width(), 20.0);
        assert_eq!(b.height(), 1.0);
        assert_eq!(b.center().x, -12.0);
        assert_eq!(b.center().y, 100.5);
    }
}