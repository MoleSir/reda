use std::ops::Mul;
use num_traits::{One, Zero};
use crate::{base::{Angle, Vec2}, shape::{CoordType, Point}, traits::{Mirror, Rotate, Scale, Translate}};

pub struct Transform<T> {
    pub mirror: bool,
    pub rotation: Angle,
    pub magnification: T,
    pub displacement: Vec2<T>,
}

impl<T> Transform<T> {
    pub fn new(mirror: bool, rotation: Angle, magnification: T, displacement: Vec2<T>) -> Self {
        Self { 
            mirror,
            rotation,
            magnification,
            displacement, 
        }
    }
}

impl<T: Zero + One> Transform<T> {
    pub fn identity() -> Self {
        Self::translate(Vec2::zero())
    }

    pub fn translate<V: Into<Vec2<T>>>(v: V) -> Self {
        let t = v.into();
        Self::new(false, Angle::R0, T::one(), t)
    }

    pub fn rotate(angle: Angle) -> Self {
        Self::new(false, angle, T::one(), Vec2::zero())
    }

    pub fn rotate_ccw90() -> Self {
        Self::rotate(Angle::R90)
    }

    pub fn rotate_cw90() -> Self {
        Self::rotate(Angle::R270)
    }

    pub fn mirror_x() -> Self {
        Self::new(true, Angle::R0, T::one(), Vec2::zero())
    }

    pub fn mirror_y() -> Self {
        Self::new(true, Angle::R180, T::one(), Vec2::zero())
    }

    pub fn scale(factor: T) -> Self {
        Self::new(false, Angle::R0, factor, Vec2::zero())
    }
}

impl<T> Transform<T>
where
    T: Copy + Mul<Output = T>,
{
    pub fn transform_distance(&self, d: T) -> T {
        d * self.magnification
    }
}

impl<T: CoordType> Transform<T> {
    pub fn rotate_around(angle: Angle, rotation_center: Point<T>) -> Self {
        Self::translate(Point::zero() - rotation_center)
            .then(&Self::rotate(angle))
            .then(&Self::translate(rotation_center))
    }

    pub fn transform_point(&self, p: Point<T>) -> Point<T> {
        if self.mirror { p.mirror_x() } else { p }
            .rotate(self.rotation)
            .scale(self.magnification)
            .translate(self.displacement)
    }

    /// Return a new transformation that is equal to applying
    pub fn then(&self, t: &Self) -> Self {
        let d = t.transform_point(self.displacement.into());
        let r = if t.mirror {
            -self.rotation
        } else {
            self.rotation
        };
        Self {
            mirror: self.mirror ^ t.mirror,
            rotation: r + t.rotation,
            magnification: self.magnification * t.magnification,
            displacement: d.vec2()
        }
    }   
}

impl<T: CoordType> Default for Transform<T> {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod test {
    use crate::{base::Angle, shape::Point, transform::Transform};

    #[test]
    fn test_simple_transform_combine() {
        let t1 = Transform::new(false, Angle::R90, 1, (1, 2).into());
        let t2 = Transform::new(true, Angle::R90, 1, (3, 4).into());
    
        let p = Point::new(10, 11);
        assert_eq!(
            t2.transform_point(t1.transform_point(p)),
            t1.then(&t2).transform_point(p)
        );
        assert_eq!(
            t1.transform_point(t2.transform_point(p)),
            t2.then(&t1).transform_point(p)
        );
    }
}