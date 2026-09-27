use num_traits::NumCast;

use crate::shape::Point;
use crate::base::Vec2;
use crate::traits::{BoundingBox, Cast, CoordType, MapPoint, TryBoundingBox};
pub use crate::base::Angle;
pub use crate::base::{Contains, Side};

use super::Rect;

/// An edge (line segment) is represented by its starting point and end point.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Edge<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T: Copy> Edge<T> {
    pub fn new(start: impl Into<Point<T>>, end: impl Into<Point<T>>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Return the same edge but with the two points swapped.
    pub fn reversed(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }
}

impl<T: PartialEq> Edge<T> {
    /// Check if edge is degenerate.
    /// An edge is degenerate if start point == end point.
    pub fn is_degenerate(&self) -> bool {
        self.start == self.end
    }

    pub fn is_rectilinear(&self) -> bool {
        !self.is_degenerate() && (self.start.x == self.end.x || self.start.y == self.end.y)
    }

    pub fn is_horizontal(&self) -> bool {
        !self.is_degenerate() && self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        !self.is_degenerate() && self.start.x == self.end.x
    }
}

impl<T: CoordType> Edge<T> {
    /// Returns the vec2 from `self.start` to `self.end`.
    pub fn vec2(&self) -> Vec2<T> {
        self.end - self.start
    }

    /// Tells on which side of the edge a point is.
    ///
    /// # Panics
    /// Panics if the edge is degenerate.
    ///
    /// Returns `Side::Left` if the point is on the left side,
    /// `Side::Right` if the point is on the right side
    /// or `Side::Center` if the point lies exactly on the line.
    pub fn side_of(&self, point: Point<T>) -> Side {
        assert!(!self.is_degenerate());

        let a = self.vec2();
        let b = point - self.start;

        // Handle rectilinear cases.
        if a.x.is_zero() {
            let side = if point.x < self.start.x {
                Side::Left
            } else if point.x > self.start.x {
                Side::Right
            } else {
                Side::Center
            };

            // Maybe flip the side depending on the orientation of the edge.
            let side = if self.start.y < self.end.y {
                side
            } else {
                side.other()
            };

            return side;
        } else if a.y.is_zero() {
            let side = if point.y < self.start.y {
                Side::Right
            } else if point.y > self.start.y {
                Side::Left
            } else {
                Side::Center
            };

            // Maybe flip the side depending on the orientation of the edge.
            let side = if self.start.x < self.end.x {
                side
            } else {
                side.other()
            };

            return side;
        }

        let area = b.cross_prod(a);

        if area.is_zero() {
            Side::Center
        } else if area < T::zero() {
            Side::Left
        } else {
            Side::Right
        }
    }

    /// Test if point lies on the edge.
    /// Includes start and end points of edge.
    pub fn contains_point(&self, point: Point<T>) -> Contains {
        if self.start == point || self.end == point {
            Contains::OnBounds
        } else if self.is_degenerate() {
            Contains::No
        } else {
            let a = self.start - point;
            let b = self.end - point;
            // Check if the triangle point-start-end has zero area and that a and b have opposite directions.
            // If a and b have opposite directions then point is between start and end.
            if a.cross_prod(b).is_zero() && a.dot(b) <= T::zero() {
                Contains::WithinBounds
            } else {
                Contains::No
            }
        }
    }

    /// Test if point lies on the line defined by the edge.
    pub fn line_contains_point(&self, point: Point<T>) -> bool {
        if self.is_degenerate() {
            self.start == point
        } else {
            let l = self.vec2();
            let b = point - self.start;

            l.cross_prod(b).is_zero()
        }
    }

    /// if two edges are parallel.
    pub fn is_parallel(&self, other: &Self) -> bool {
        if self.is_degenerate() || other.is_degenerate() {
            false
        } else {
            let a = self.vec2();
            let b = other.vec2();

            if a.x.is_zero() {
                // Both vertical?
                b.x.is_zero()
            } else if a.y.is_zero() {
                // Both horizontal?
                b.y.is_zero()
            } else if b.x.is_zero() || b.y.is_zero() {
                false
            } else {
                a.cross_prod(b).is_zero()
            }
        }
    }

    /// if two edges are collinear, i.e. are on the same line.
    pub fn is_collinear(&self, other: &Self) -> bool {
        if self.is_degenerate() || other.is_degenerate() {
            false
        } else {
            let v = self.vec2();
            let a = other.start - self.start;
            let b = other.end - self.start;

            if self.start.x == self.end.x {
                // Self is vertical.
                // The edges are collinear iff `other` has the same x coordinates as `self`.
                other.start.x == self.start.x && other.end.x == self.start.x
            } else if self.start.y == self.end.y {
                // `self` is horizontal
                other.start.y == self.start.y && other.end.y == self.start.y
            } else if other.start.x == other.end.x {
                self.start.x == other.start.x && self.end.x == other.start.x
            } else if other.start.y == other.end.y {
                // `other` is horizontal
                self.start.y == other.start.y && self.end.y == other.start.y
            } else {
                v.cross_prod(a).is_zero() && v.cross_prod(b).is_zero()
            }
        }
    }

    /// Test edges for coincidence.
    /// Two edges are coincident if they are oriented the same way
    /// and share more than one point (implies that they must be parallel).
    pub fn is_coincident(&self, other: &Self) -> bool {
        // TODO: use is_collinear
        // TODO: approximate version for floating point types

        let self_start_in_other = other.contains_point(self.start).inclusive_bounds();
        let self_end_in_other = other.contains_point(self.end).inclusive_bounds();
        let other_start_in_self = self.contains_point(other.start).inclusive_bounds();
        let other_end_in_self = self.contains_point(other.end).inclusive_bounds();

        let share_more_than_one_point = self.end != other.start
            && self.start != other.end
            && (self_start_in_other || other_start_in_self)
            && (other_end_in_self || self_end_in_other);

        // Sharing more than one point should imply that the edges are parallel.
        debug_assert!(if share_more_than_one_point {
            self.is_parallel(other)
        } else {
            true
        });

        let oriented_the_same_way = self.vec2().dot(other.vec2()) > T::zero();

        share_more_than_one_point && oriented_the_same_way
    }

    /// Test if two edges are approximately parallel.
    /// To be used for float coordinates.
    /// Inspired by algorithm on page 241 of "Geometric Tools for Computer Graphics".
    pub fn is_parallel_approx(&self, other: &Self, epsilon_squared: T) -> bool {
        if self.is_degenerate() || other.is_degenerate() {
            false
        } else {
            let d1 = self.vec2();
            let d2 = other.vec2();

            let len1_sqr = d1.norm2_squared();
            let len2_sqr = d2.norm2_squared();

            let cross = d1.cross_prod(d2);
            let cross_sqr = cross * cross;

            // TODO: require square tolerance form caller?
            cross_sqr <= len1_sqr * len2_sqr * epsilon_squared
        }
    }

    /// Test if two edges are approximately collinear, i.e. are on the same line.
    /// Inspired by algorithm on page 241 of "Geometric Tools for Computer Graphics".
    pub fn is_collinear_approx(&self, other: &Self, epsilon_squared: T) -> bool {
        if self.is_degenerate() || other.is_degenerate() {
            false
        } else {
            let d1 = self.vec2();
            let d2 = other.vec2();

            let len1_sqr = d1.norm2_squared();
            let len2_sqr = d2.norm2_squared();

            let cross = d1.cross_prod(d2);
            let cross_sqr = cross * cross;

            let approx_parallel = cross_sqr <= len1_sqr * len2_sqr * epsilon_squared;

            if approx_parallel {
                let e = other.start - self.start;
                let len_e_sqrt = e.norm2_squared();
                let cross = e.cross_prod(d1);
                let cross_sqr = cross * cross;
                cross_sqr <= len1_sqr * len_e_sqrt * epsilon_squared
            } else {
                false
            }
        }
    }

    /// Test if lines defined by the edges intersect.
    /// If the lines are collinear they are also considered intersecting.
    pub fn lines_intersect_approx(&self, other: &Self, epsilon_squared: T) -> bool {
        !self.is_parallel_approx(other, epsilon_squared)
            || self.is_collinear_approx(other, epsilon_squared)
    }

    /// Test if this edge is crossed by the line defined by the other edge.
    ///
    /// Returns `WithinBounds` if start and end point of this edge lie on different sides
    /// of the line defined by the `other` edge or `OnBounds` if at least one of the points
    /// lies on the line.
    pub fn crossed_by_line(&self, other: &Self) -> Contains {
        // TODO: Handle degenerate cases.
        let side1 = other.side_of(self.start);

        if side1 == Side::Center {
            Contains::OnBounds
        } else {
            let side2 = other.side_of(self.end);

            if side2 == Side::Center {
                Contains::OnBounds
            } else if side1 == side2 {
                Contains::No
            } else {
                Contains::WithinBounds
            }
        }
    }

    /// Test if lines defined by the edges intersect.
    /// If the lines are collinear they are also considered intersecting.
    pub fn lines_intersect(&self, other: &Self) -> bool {
        !self.is_parallel(other) || self.is_collinear(other)
    }
}

impl<T: Copy> From<Edge<T>> for (Point<T>, Point<T>) {
    fn from(e: Edge<T>) -> Self {
        (e.start, e.end)
    }
}

impl<T: Copy> From<&Edge<T>> for (Point<T>, Point<T>) {
    fn from(e: &Edge<T>) -> Self {
        (e.start, e.end)
    }
}

impl<T: CoordType> From<(Point<T>, Point<T>)> for Edge<T> {
    fn from(points: (Point<T>, Point<T>)) -> Self {
        Edge::new(points.0, points.1)
    }
}

impl<T: CoordType> From<[Point<T>; 2]> for Edge<T> {
    fn from(points: [Point<T>; 2]) -> Self {
        Edge::new(points[0], points[1])
    }
}

impl<T: Copy> MapPoint<T> for Edge<T> {
    fn transform<F>(&self, transformate: F) -> Self
        where 
            F: Fn(Point<T>) -> Point<T> 
    {
        Self {
            start: transformate(self.start),
            end: transformate(self.end)
        }    
    }
}

impl<T: Copy + PartialOrd> BoundingBox<T> for Edge<T> {
    fn bounding_box(&self) -> Rect<T> {
        Rect::new(self.start, self.end)
    }
}

impl<T: Copy + PartialOrd> TryBoundingBox<T> for Edge<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        Some(self.bounding_box())
    }
}

impl<Src: Copy + NumCast, Dst: Copy + NumCast> Cast<Src, Dst> for Edge<Src> {
    type Output = Edge<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        match (self.start.try_cast(), self.end.try_cast()) {
            (Some(s), Some(e)) => Some(Edge::new(s, e)),
            _ => None,
        }
    }
}



#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use crate::shape::Edge;
    use crate::shape::Point;

    #[test]
    fn test_is_parallel() {
        let e1 = Edge::new((1, 2), (3, 4));
        let e2 = Edge::new((1 - 10, 2 - 20), (5 - 10, 6 - 20));
        let e3 = Edge::new((1 - 10, 2 - 20), (5 - 10, 6 - 20 + 1));

        assert!(e1.is_parallel(&e2));
        assert!(!e1.is_parallel(&e3));

        assert!(e1.is_parallel_approx(&e2, 0));
        assert!(!e1.is_parallel_approx(&e3, 0));

        assert!(e1.is_parallel_approx(&e3, 1));
    }

    #[test]
    fn test_is_collinear() {
        let e1 = Edge::new((0, 0), (1, 2));
        let e2 = Edge::new((10, 20), (100, 200));
        assert!(e1.is_collinear(&e2));
        assert!(e2.is_collinear(&e1));
        assert!(e1.is_collinear_approx(&e2, 0));
        assert!(e2.is_collinear_approx(&e1, 0));

        // Not collinear.
        let e1 = Edge::new((0i64, 0), (1, 2));
        let e2 = Edge::new((10, 20), (1000, 2001));
        assert!(!e1.is_collinear(&e2));
        assert!(!e2.is_collinear(&e1));
        assert!(!e1.is_collinear_approx(&e2, 0));
        assert!(!e2.is_collinear_approx(&e1, 0));

        assert!(e1.is_collinear_approx(&e2, 1));
        assert!(e2.is_collinear_approx(&e1, 1));
    }

    #[test]
    fn test_line_contains_point() {
        let e1 = Edge::new((1, 2), (5, 6));
        let p0 = Point::new(1, 2);
        let p1 = Point::new(3, 4);
        let p2 = Point::new(5, 6);
        let p3 = Point::new(6, 7);
        let p4 = Point::new(0, 1);
        let p5 = Point::new(0, 0);

        assert!(e1.line_contains_point(p0));
        assert!(e1.line_contains_point(p1));
        assert!(e1.line_contains_point(p2));
        assert!(e1.line_contains_point(p3));
        assert!(e1.line_contains_point(p4));
        assert!(!e1.line_contains_point(p5));

        let e1 = Edge::new((0, 0), (1, 1));
        assert!(e1.line_contains_point(Point::new(2, 2)));
        assert!(e1.line_contains_point(Point::new(-1, -1)));
    }

    #[test]
    fn test_contains() {
        let e1 = Edge::new((1, 2), (5, 6));
        let p0 = Point::new(1, 2);
        let p1 = Point::new(3, 4);
        let p2 = Point::new(5, 6);
        let p3 = Point::new(0, 0);
        let p4 = Point::new(6, 7);

        assert!(e1.contains_point(p0).inclusive_bounds());
        assert!(e1.contains_point(p1).inclusive_bounds());
        assert!(e1.contains_point(p2).inclusive_bounds());
        assert!(!e1.contains_point(p3).inclusive_bounds());
        assert!(!e1.contains_point(p4).inclusive_bounds());

        let e1 = Edge::new((0, 0), (1, 1));
        let p0 = Point::new(2, 2);
        assert!(!e1.contains_point(p0).inclusive_bounds());
    }

    #[test]
    fn test_side_of() {
        let e1 = Edge::new((1, 0), (4, 4));
        let p1 = Point::new(-10, 0);
        let p2 = Point::new(10, -10);
        let p3 = Point::new(1, 0);

        assert_eq!(e1.side_of(p1), Side::Left);
        assert_eq!(e1.side_of(p2), Side::Right);
        assert_eq!(e1.side_of(p3), Side::Center);
    }

    #[test]
    fn test_crossed_by() {
        let e1 = Edge::new((1, 0), (4, 4));
        let e2 = Edge::new((1 + 1, 0), (4 + 1, 4));
        let e3 = Edge::new((1, 0), (4, 5));
        let e4 = Edge::new((2, -2), (0, 0));

        // Coincident lines
        assert!(e1.crossed_by_line(&e1).inclusive_bounds());

        // Parallel but not coincident.
        assert!(e1.is_parallel(&e2));
        assert!(!e1.crossed_by_line(&e2).inclusive_bounds());

        // Crossing lines.
        assert!(e1.crossed_by_line(&e3).inclusive_bounds());

        // crossed_by is not commutative
        assert!(!e1.crossed_by_line(&e4).inclusive_bounds());
        assert!(e4.crossed_by_line(&e1).inclusive_bounds());
    }

    #[test]
    fn test_intersect() {
        let e1 = Edge::new((0, 0), (4, 4));
        let e3 = Edge::new((0, -1), (-1, 1));

        assert!(!e1.crossed_by_line(&e3).inclusive_bounds());
        assert!(e3.crossed_by_line(&e1).inclusive_bounds());

        // Bounding boxes overlap but edges are not touching.
        let e1 = Edge::new((0, 0), (8, 8));
        let e2 = Edge::new((4, 0), (3, 1));
        assert!(!e1.is_rectilinear());
        assert!(!e2.is_rectilinear());
        assert!(e1.bounding_box().touches(&e2.bounding_box()));
        assert!(e1.lines_intersect(&e2));
    }

    #[test]
    fn test_coincident() {
        let e1 = Edge::new((0, 0), (2, 2));
        let e2 = Edge::new((1, 1), (3, 3));
        assert!(e1.is_coincident(&e2));
        assert!(e2.is_coincident(&e1));

        let e1 = Edge::new((0, 0), (3, 3));
        let e2 = Edge::new((1, 1), (2, 2));
        assert!(e1.is_coincident(&e2));
        assert!(e2.is_coincident(&e1));

        let e1 = Edge::new((0, 0), (1, 1));
        let e2 = Edge::new((1, 1), (2, 2));
        assert!(!e1.is_coincident(&e2));
        assert!(!e2.is_coincident(&e1));
    }
}