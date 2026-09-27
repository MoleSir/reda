use std::{fmt, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};
use num_traits::{Float, NumCast, Zero};
use crate::base::Orientation;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Zero> Vec2<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new($x, $y)
    };
}

impl<T: Copy> From<Vec2<T>> for (T, T) {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        (v.x, v.y)
    }
}

impl<T: Copy> From<&Vec2<T>> for (T, T) {
    #[inline]
    fn from(v: &Vec2<T>) -> Self {
        (v.x, v.y)
    }
}

impl<T> Vec2<T> {
    /// Create a new vec2 with `x` and `y` coordinates.
    /// # Examples
    /// ```
    /// use reda_geometry::base::Vec2;
    /// let a = Vec2::new(2, 3);
    /// assert_eq!(a.x, 2);
    /// assert_eq!(a.y, 3);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T: Copy + Zero + PartialOrd + Sub<Output = T>> Vec2<T> {
    /// Get 1-norm of vec2, i.e. the sum of the absolute values of its components.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::base::Vec2;
    /// let a = Vec2::new(-2, 3);
    /// assert_eq!(a.norm1(), 5);
    /// ```
    pub fn norm1(&self) -> T {
        let xabs = if self.x < T::zero() {
            T::zero() - self.x
        } else {
            self.x
        };

        let yabs = if self.y < T::zero() {
            T::zero() - self.y
        } else {
            self.y
        };

        xabs + yabs
    }
}

impl<T: Copy + Zero + PartialOrd + Mul<Output = T> + Sub<Output = T>> Vec2<T> {
    /// Check if `other` is oriented clockwise or counter-clockwise respective to `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::base::Vec2;
    /// use reda_geometry::base::Orientation;
    ///
    /// let a = Vec2::new(1, 0);
    /// let b = Vec2::new(1, 1);
    /// let c = Vec2::new(1, -1);
    /// let d = Vec2::new(2, 0);
    ///
    /// assert_eq!(a.orientation_of(b), Orientation::CounterClockWise);
    /// assert_eq!(a.orientation_of(c), Orientation::ClockWise);
    /// assert_eq!(a.orientation_of(d), Orientation::Straight);
    /// ```
    pub fn orientation_of(&self, other: Self) -> Orientation {
        let p = self.cross_prod(other);

        if p < T::zero() {
            Orientation::ClockWise
        } else if p > T::zero() {
            Orientation::CounterClockWise
        } else {
            Orientation::Straight
        }
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Vec2<T> {
    /// Calculate cross product.
    ///
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::base::Vec2;
    ///
    /// let a = Vec2::new(2, 0);
    /// let b = Vec2::new(0, 2);
    ///
    /// assert_eq!(a.cross_prod(b), 4);
    /// assert_eq!(b.cross_prod(a), -4);
    /// ```
    #[inline]
    pub fn cross_prod(&self, other: Self) -> T {
        self.x * other.y - other.x * self.y
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Vec2<T> {
    /// Get squared 2-norm of vec2.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::base::Vec2;
    /// let a = Vec2::new(2, 3);
    /// assert_eq!(a.norm2_squared(), 2*2+3*3);
    /// ```
    #[inline]
    pub fn norm2_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Calculate scalar product.
    ///
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::base::Vec2;
    ///
    /// let a = Vec2::new(1, 2);
    /// let b = Vec2::new(3, 4);
    ///
    /// assert_eq!(a.dot(b), 1*3 + 2*4);
    /// ```
    pub fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}


impl<T> Vec2<T>
where
    T: Copy + Float + Mul<Output = T> + Add<Output = T>,
{
    /// Get 2-norm of vec2 (length of vec2).
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::base::Vec2;
    /// let a = Vec2::new(2.0, 3.0);
    /// let norm2 = a.norm2();
    /// let norm2_sq = norm2 * norm2;
    /// let expected = a.norm2_squared();
    /// assert!(norm2_sq < expected + 1e-12);
    /// assert!(norm2_sq > expected - 1e-12);
    /// ```
    #[inline]
    pub fn norm2(&self) -> T {
        (self.norm2_squared()).sqrt()
    }

    /// Return a vec2 with the same direction but length 1.
    ///
    /// # Panics
    /// Panics if the vec2 has length 0.
    #[inline]
    pub fn normalized(&self) -> Self {
        *self / self.norm2()
    }
}

impl<T: Copy + NumCast> Vec2<T> {
    /// Calculate length of vec2.
    ///
    /// Similar to `Vec2::norm2` but does potentially return another data type for the length.
    ///
    /// # Examples
    /// ```
    /// use reda_geometry::base::Vec2;
    /// let a = Vec2::new(2.0, 3.0);
    /// let length: f64 = a.length();
    /// let norm2_sq = length * length;
    /// let expected = a.norm2_squared();
    /// assert!(norm2_sq < expected + 1e-12);
    /// assert!(norm2_sq > expected - 1e-12);
    /// ```
    #[inline]
    pub fn length<F: Float>(&self) -> F {
        let x = F::from(self.x).unwrap();
        let y = F::from(self.y).unwrap();

        x.hypot(y)
    }
}

impl<T> Vec2<T>
where
    T: Copy + NumCast,
{
    /// Convert vec2 into a vec2 with floating point data type.
    #[inline]
    pub fn to_float<F>(&self) -> Vec2<F>
    where
        F: Float + NumCast,
    {
        Vec2 {
            x: F::from(self.x).unwrap(),
            y: F::from(self.y).unwrap(),
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Vec2 subtraction.
impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Scalar multiplication.
impl<T, M> Mul<M> for Vec2<T>
where
    T: Mul<M, Output = T>,
    M: Copy,
{
    type Output = Self;

    fn mul(self, rhs: M) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// In-place scalar multiplication.
impl<T, M> MulAssign<M> for Vec2<T>
where
    T: Copy + MulAssign<M>,
    M: Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: M) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Scalar division.
impl<T, D> Div<D> for Vec2<T>
where
    T: Copy + Div<D, Output = T>,
    D: Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: D) -> Self {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Assigning scalar division.
impl<T, D> DivAssign<D> for Vec2<T>
where
    T: Copy + DivAssign<D>,
    D: Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: D) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/// Compute the sum of all vec2s in the iterator.
/// If the iterator is empty, (0, 0) is returned.
impl<T> std::iter::Sum for Vec2<T>
where
    T: Zero + Add<Output = T>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, v| acc + v)
    }
}


impl<T: Copy> From<(T, T)> for Vec2<T> {
    #[inline]
    fn from(coords: (T, T)) -> Self {
        Vec2::new(coords.0, coords.1)
    }
}

impl<'a, T: Copy> From<&'a (T, T)> for Vec2<T> {
    #[inline]
    fn from(coords: &'a (T, T)) -> Self {
        Vec2::new(coords.0, coords.1)
    }
}

impl<'a, T: Copy> From<&'a Vec2<T>> for Vec2<T> {
    #[inline]
    fn from(v: &'a Vec2<T>) -> Self {
        Vec2::new(v.x, v.y)
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    #[inline]
    fn from(coords: [T; 2]) -> Self {
        Vec2::new(coords[0], coords[1])
    }
}


impl<T> fmt::Debug for Vec2<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", &self.x, &self.y)
    }
}

impl<T> fmt::Display for Vec2<T>
where
    T: fmt::Display + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

