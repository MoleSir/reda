use num_traits::NumCast;

use crate::traits::{Cast, MapPoint, TryBoundingBox};

use super::{CoordType, Edge, Point, Rect};
use std::cmp::PartialEq;
use std::slice::Iter;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Polyline<T> {
    pub points: Vec<Point<T>>,
    pub normalized: bool,
}

#[macro_export]
macro_rules! polyline {
    ($($x:expr),*) => {  };
}

impl<T> Polyline<T> {
    pub fn from_points(points: impl Into<Vec<Point<T>>>) -> Self {
        Self {
            points: points.into(),
            normalized: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            points: Vec::new(),
            normalized: false,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn iter(&self) -> Iter<Point<T>> {
        self.points.iter()
    }

    pub fn points(&self) -> &[Point<T>] {
        &self.points
    }

    /// Get index of previous vertex.
    pub fn prev_index(&self, i: usize) -> usize {
        match i {
            0 => self.points.len() - 1,
            x => x - 1,
        }
    }

    /// Get index of next vertex.
    pub fn next_index(&self, i: usize) -> usize {
        match i {
            _ if i == self.points.len() - 1 => 0,
            x => x + 1,
        }
    }
}

impl<T: PartialOrd> Polyline<T> {
    pub fn new(points: Vec<Point<T>>) -> Self {
        Self::from_points(points).normalized()
    }

    /// Rotate the vertices to get the lexicographically smallest polygon.
    /// Does not change the orientation.
    pub fn normalize(&mut self) {
        if !self.normalized {
            let rotated = 
                |rot: usize| self.points.iter()
                    .cycle()
                    .skip(rot)
                    .take(self.points.len());
            
            let best_rot = (0..self.points.len()).skip(1).fold(0, |best_rot, rot| {
                if rotated(rot).lt(rotated(best_rot)) {
                    rot
                } else {
                    best_rot
                }
            });
            self.points.rotate_left(best_rot);
        }
        self.normalized = true;
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<T: Copy> Polyline<T> {
    pub fn iter_cycle(&self) -> impl Iterator<Item = &Point<T>> {
        self.points.iter().cycle().take(self.points.len() + 1)
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = Edge<T>> + '_ {
        self.iter()
            .zip(self.iter_cycle().skip(1))
            .map(|(a, b)| Edge::new(a, b))
    }

    pub fn edges(&self) -> Vec<Edge<T>> {
        self.edges_iter().collect()
    }

    pub fn from_rect(rect: &Rect<T>) -> Self {
        Self {
            points: vec![
                rect.lower_left(),
                rect.lower_right(),
                rect.upper_right(),
                rect.upper_left(),
            ],
            normalized: true,
        }
    }
}

impl<T, P> FromIterator<P> for Polyline<T> 
where 
    T: Copy,
    P: Into<Point<T>>
{
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        let points: Vec<Point<T>> = iter.into_iter().map(|x| x.into()).collect();
        assert!(
            points.len() >= 2,
            "A polygon needs to have at least two points."
        );
        Self::from_points(points)
    }
}

impl<'a, T> IntoIterator for &'a Polyline<T> {
    type Item = &'a Point<T>;
    type IntoIter = std::slice::Iter<'a, Point<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

impl<I, T, P> From<I> for Polyline<T> 
where 
    T: Copy + PartialOrd,
    I: IntoIterator<Item = P>,
    Point<T>: From<P>,
{
    fn from(iter: I) -> Self {
        let points: Vec<Point<T>> = iter.into_iter().map(|x| x.into()).collect();
        Polyline::from_points(points)
    }
}

impl<T: Copy> MapPoint<T> for Polyline<T> {
    fn transform<F>(&self, transformate: F) -> Self
    where 
        F: Fn(Point<T>) -> Point<T> 
    {
        let points: Vec<_> = self.points.iter().map(|&p| transformate(p)).collect();
        Self::from_points(points)    
    }
}

impl<T: Copy + PartialOrd> TryBoundingBox<T> for Polyline<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        if !self.is_empty() {
            let mut x_min = self.points[0].x;
            let mut x_max = x_min;
            let mut y_min = self.points[0].y;
            let mut y_max = y_min;

            for p in self.iter().skip(1) {
                if p.x < x_min {
                    x_min = p.x;
                }
                if p.x > x_max {
                    x_max = p.x;
                }
                if p.y < y_min {
                    y_min = p.y;
                }
                if p.y > y_max {
                    y_max = p.y;
                }
            }

            Some(Rect::new((x_min, y_min), (x_max, y_max)))
        } else {
            None
        }
    }
}

impl<T: CoordType + NumCast, Dst: CoordType + NumCast> Cast<T, Dst> for Polyline<T> {
    type Output = Polyline<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        let new_points: Option<Vec<_>> = self.points.iter().map(|p| p.try_cast()).collect();
        new_points.map(|p| Polyline::new(p))
    }
}
