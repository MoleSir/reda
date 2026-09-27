use std::ops::{Add, Mul, Sub};
use num_traits::{Num, NumCast, Zero};
use crate::{base::{Angle, Vec2}, shape::{Point, Polygon, Rect}};

pub trait CoordType: Num + Copy + PartialOrd + Sized {}
impl<T: Num + Copy + PartialOrd> CoordType for T {}

pub trait BoundingBox<T> {
    fn bounding_box(&self) -> Rect<T>;
}

pub trait TryBoundingBox<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>>;
}

/// Translate the geometrical object
pub trait Translate<T> {
    fn translate(&self, v: Vec2<T>) -> Self;
}

/// Scale the geometrical shape. Scaling center is the origin `(0, 0)`.
pub trait Scale<T> {
    fn scale(&self, factor: T) -> Self;
}

/// Rotate by a integer multiple of 90 degrees.
pub trait Rotate<T> {
    fn rotate(&self, a: Angle) -> Self;
}

/// Mirror at the x or y axis.
pub trait Mirror<T> {
    fn mirror_x(&self) -> Self;
    fn mirror_y(&self) -> Self;
}

/// Transform the geometrical object by transforming each point of it.
pub trait MapPoint<T> {
    fn transform<F>(&self, transformate: F) -> Self
    where 
        F: Fn(Point<T>) -> Point<T>;
}

impl<S, T> Scale<T> for S 
where 
    T: Copy + Mul<Output = T>,
    S: MapPoint<T>,
{
    fn scale(&self, factor: T) -> Self {
        self.transform(|p| p * factor)
    }
}

impl<S, T> Translate<T> for S
where
    T: Copy + Add<Output = T>,
    S: MapPoint<T>,
{
    fn translate(&self, v: Vec2<T>) -> S {
        self.transform(|p: Point<T>| p + v)
    }
}

impl<S, T> Rotate<T> for S
where
    T: Copy + Zero + Sub<Output = T>,
    S: MapPoint<T>,
{
    fn rotate(&self, a: Angle) -> S {
        self.transform(|p: Point<T>| match a {
            Angle::R0 => p,
            Angle::R90 => Point::new(T::zero() - p.y, p.x),
            Angle::R180 => Point::zero() - p.vec2(),
            Angle::R270 => Point::new(p.y, T::zero() - p.x),
        })
    }
}

impl<S, T> Mirror<T> for S 
where 
    T: Copy + Zero + Sub<Output = T>,
    S: MapPoint<T>
{
    fn mirror_x(&self) -> Self {
        self.transform(|p| Point::new(p.x, T::zero() - p.y))
    }

    fn mirror_y(&self) -> Self {
        self.transform(|p| Point::new(T::zero() - p.x, p.y))
    }
}

pub trait Cast<Src, Dst>
where 
    Src: NumCast,
    Dst: NumCast,
{
    type Output;

    fn try_cast(&self) -> Option<Self::Output>;
    fn cast(&self) -> Self::Output {
        self.try_cast().unwrap()
    }
}

pub trait ToPolygon<T> {
    fn to_polygon(&self) -> Polygon<T>;
}