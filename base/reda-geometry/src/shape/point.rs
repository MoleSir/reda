use std::cmp::{Ord, Ordering};
use std::fmt;
pub use num_traits::Zero;
use num_traits::{Float, NumCast};
pub use std::ops::Deref;
use std::ops::{Add, AddAssign, DerefMut, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::base::{Orientation2D, Vec2};
use crate::traits::{BoundingBox, Cast, MapPoint, TryBoundingBox};

use super::Rect;

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

/// A point is defined by a x and y coordinate in the euclidean plane.
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct Point<T> {
    /// Store the location as a translation from the origin.
    location: Vec2<T>,
}

impl<T> Deref for Point<T> {
    type Target = Vec2<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.location
    }
}

impl<T> DerefMut for Point<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.location
    }
}

impl<T> From<Vec2<T>> for Point<T> {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        Point { location: v }
    }
}

impl<T: Copy> From<&Vec2<T>> for Point<T> {
    #[inline]
    fn from(v: &Vec2<T>) -> Self {
        Self::from(*v)
    }
}

impl<T> From<Point<T>> for Vec2<T> {
    #[inline]
    fn from(p: Point<T>) -> Self {
        p.location
    }
}

impl<T: Copy> From<&Point<T>> for Vec2<T> {
    #[inline]
    fn from(p: &Point<T>) -> Self {
        p.location
    }
}

impl<T> Point<T> {
    /// Create a new point with `x` and `y` coordinates.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Point {
            location: Vec2::new(x, y),
        }
    }
}

impl<T: Copy> Point<T> {
    #[inline]
    pub fn vec2(&self) -> Vec2<T> {
        self.location
    }

    pub fn get(&self, coord: Orientation2D) -> T {
        match coord {
            Orientation2D::Horizontal => self.x,
            Orientation2D::Vertical => self.y,
        }
    }

    pub fn set(&mut self, coord: Orientation2D, value: T) {
        match coord {
            Orientation2D::Horizontal => self.x = value,
            Orientation2D::Vertical => self.y = value,
        }
    }
}

impl<T: Zero> Point<T> {
    /// Get zero-Point.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::shape::Point;
    ///
    /// let a = Point::zero();
    /// let b = Point::new(0, 0);
    ///
    /// assert_eq!(a, b);
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Vec2::zero().into()
    }

    /// Check if this is the zero-Point.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::shape::Point;
    ///
    /// assert!(Point::<usize>::zero().is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>> Point<T> {
    /// Compute the squared distance to the `other` point.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::shape::Point;
    ///
    /// let a = Point::new(0, 0);
    /// let b = Point::new(2, 0);
    /// assert_eq!(a.distance_sq(&b), 2*2);
    /// ```
    #[inline]
    pub fn distance_sq(self, other: &Point<T>) -> T {
        let diff = self - *other;
        diff.norm2_squared()
    }

    /// Calculate the cross product of the two vec2s defined by three points.
    ///
    /// A positive value implies that `self` → `a` → `b` is counter-clockwise, negative implies
    /// clockwise.
    ///
    /// (`b` - `self`) x (`c` - `b`)
    ///
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::shape::Point;
    ///
    /// let a = Point::new(1,0);
    /// let b = Point::new(1,1);
    /// let c = Point::new(0,1);
    ///
    /// let p = a.cross_prod3(b, c);
    ///
    /// assert_eq!(p, (b-a).cross_prod(c - b));
    /// ```
    #[inline]
    pub fn cross_prod3(&self, b: Point<T>, c: Point<T>) -> T {
        (b.x - self.x) * (c.y - b.y) - (b.y - self.y) * (c.x - b.x)
    }
}

impl<T: Copy + Sub<Output = T> + NumCast> Point<T> {
    /// Compute the Euclidean distance betwen two points.
    #[inline]
    pub fn distance<F: Float>(self, other: &Point<T>) -> F {
        let diff = self - *other;
        diff.length()
    }
}

/// Compare points.
///
/// The ordering is determined by the x-coordinates. If it is the same
/// for both points the y-coordinate is used.
///
/// Point `a` > Point `b` iff `a.x > b.x || (a.x == b.x && a.y > b.y)`.
impl<T: PartialOrd> PartialOrd for Point<T> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&rhs.x) {
            Some(Ordering::Equal) => self.y.partial_cmp(&rhs.y),
            maybe_ordering => maybe_ordering,
        }
    }
}

/// Compare points.
///
/// The ordering is determined by the x-coordinates. If it is the same
/// for both points the y-coordinate is used.
///
/// Point `a` > Point `b` iff `a.x > b.x || (a.x == b.x && a.y > b.y)`.
impl<T: Ord> Ord for Point<T> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.x.cmp(&rhs.x) {
            Ordering::Equal => self.y.cmp(&rhs.y),
            ordering => ordering,
        }
    }
}

impl<T: Copy> From<Point<T>> for (T, T) {
    #[inline]
    fn from(p: Point<T>) -> Self {
        (p.x, p.y)
    }
}

impl<T: Copy> From<&Point<T>> for (T, T) {
    #[inline]
    fn from(p: &Point<T>) -> Self {
        (p.x, p.y)
    }
}

impl<T: Copy> From<(T, T)> for Point<T> {
    #[inline]
    fn from(coords: (T, T)) -> Self {
        Point::new(coords.0, coords.1)
    }
}

impl<'a, T: Copy> From<&'a (T, T)> for Point<T> {
    #[inline]
    fn from(coords: &'a (T, T)) -> Self {
        Point::new(coords.0, coords.1)
    }
}

impl<'a, T: Copy> From<&'a Point<T>> for Point<T> {
    #[inline]
    fn from(v: &'a Point<T>) -> Self {
        Point::new(v.x, v.y)
    }
}

impl<T: Copy> From<[T; 2]> for Point<T> {
    #[inline]
    fn from(coords: [T; 2]) -> Self {
        Point::new(coords[0], coords[1])
    }
}

impl<T: Copy> From<Point<T>> for [T; 2] {
    #[inline]
    fn from(p: Point<T>) -> Self {
        [p.x, p.y]
    }
}

impl<T> fmt::Debug for Point<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({:?}, {:?})", self.x, self.y)
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Copy + NumCast> Point<T> {
    /// Convert Point into a Point with floating point data type.
    #[inline]
    pub fn cast_to_float<F: Float + NumCast>(&self) -> Point<F> {
        // TODO: find conversion that does not panic for sure.
        Point::new(F::from(self.x).unwrap(), F::from(self.y).unwrap())
    }
}

impl<T, V> Add<V> for Point<T>
where
    T: Copy + Add<Output = T>,
    V: Into<Point<T>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: V) -> Self {
        let rhs = rhs.into();
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, V> AddAssign<V> for Point<T>
where
    T: Copy + AddAssign<T>,
    V: Into<Vec2<T>>,
{
    #[inline]
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        self.location += rhs;
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Vec2<T>;

    #[inline]
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Sub<Vec2<T>> for Point<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Point<T>;

    #[inline]
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, V> SubAssign<V> for Point<T>
where
    T: Copy + SubAssign<T>,
    V: Into<Vec2<T>>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        self.location -= rhs;
    }
}

impl<T> Neg for Point<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Point::new(-self.x, -self.y)
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<T> for Point<T>
where
    T: Copy + MulAssign<T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.location *= rhs;
    }
}

/// Scalar division.
impl<T> Div<T> for Point<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Copy + Zero + Add<Output = T>> std::iter::Sum for Point<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, p| acc + p)
    }
}

impl<T: Copy> MapPoint<T> for Point<T> {
    fn transform<F>(&self, transformate: F) -> Self
        where 
            F: Fn(Point<T>) -> Point<T> 
    {
        transformate(*self)
    }
}

impl<T: Copy> BoundingBox<T> for Point<T> {
    fn bounding_box(&self) -> Rect<T> {
        Rect {
            lower_left: *self,
            upper_right: *self,
        }
    }
}

impl<T: Copy> TryBoundingBox<T> for Point<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        Some(self.bounding_box())
    }
}

impl<Src: Copy + NumCast, Dst: Copy + NumCast> Cast<Src, Dst> for Point<Src> {
    type Output = Point<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        match (Dst::from(self.x), Dst::from(self.y)) {
            (Some(x), Some(y)) => Some(Point::new(x, y)),
            _ => None,
        }
    }
}