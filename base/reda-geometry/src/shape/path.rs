// Copyright (c) 2018-2020 Thomas Kramer.
// SPDX-FileCopyrightText: 2018-2022 Thomas Kramer
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A point string is a finite sequence of points.

use crate::base::Vec2;
use crate::shape::Edge;
use crate::shape::Point;
use crate::traits::Cast;
pub use crate::traits::CoordType;
use crate::traits::MapPoint;
use crate::traits::Rotate;
use crate::traits::TryBoundingBox;
use crate::transform::Transform;
use std::iter::FromIterator;
use std::slice::Iter;

use num_traits::Num;
use num_traits::{Float, NumCast};
use std::ops::Sub;

use super::Angle;
use super::Rect;

/// `Path` is essentially a chain of line segments but with a possibly a non-zero width.
/// It can be thought of the shape resulting by a stroke of a thick pen along the line segments.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Path<T> {
    /// The vertices of the path which define the sequence of line segments.
    pub points: PointList<T>,
    /// Width of the path.
    pub width: T,
    /// Type of the path endings.
    pub path_type: PathEndType<T>,
}

/// Encoding for the type of the beginning and end of the path.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PathEndType<T> {
    Flat,
    Extended(T, T),
    Round,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointList<T> {
    /// The points defining this point string.
    pub points: Vec<Point<T>>,
}

impl<T> Path<T> {
    pub fn new(points: impl Into<PointList<T>>, width: T, path_type: PathEndType<T>) -> Self {
        Self { points: points.into(), width, path_type }
    }

    pub fn flat(points: impl Into<PointList<T>>, width: T) -> Self {
        Self::new(points, width, PathEndType::Flat)
    }

    pub fn round(points: impl Into<PointList<T>>, width: T) -> Self {
        Self::new(points, width, PathEndType::Round)
    }
}

impl<T> PointList<T> {
    /// Get the number of vertices.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if string has zero length.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

impl<T: Copy> PointList<T> {
    /// Create new point string by taking vertices from a type that implements `Into<PointList<T>>`.
    pub fn new<I>(i: I) -> Self
    where
        I: Into<Self>,
    {
        i.into()
    }

    /// Shortcut for `self.points.iter()`.
    pub fn iter(&self) -> Iter<'_, Point<T>> {
        self.points.iter()
    }

    /// Get the sequence of edges of the point string starting from the first point to the last.
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::shape::PointList;
    /// use reda_geometry::shape::Edge;
    /// let coords = vec![(0, 0), (1, 0), (2, 0)];
    ///
    /// let point_string = PointList::new(coords);
    ///
    /// let edges: Vec<_> = point_string.edges().collect();
    ///
    /// assert_eq!(edges, vec![Edge::new((0, 0), (1, 0)), Edge::new((1, 0), (2, 0))]);
    /// ```
    pub fn edges(&self) -> impl Iterator<Item = Edge<T>> + '_ {
        self.iter()
            .zip(self.iter().skip(1))
            .map(|(a, b)| Edge::new(a, b))
    }

    /// Same as `edges` but in reverse order.
    /// Get the sequence of edges of the point string starting from the last point to the first.
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::shape::PointList;
    /// use reda_geometry::shape::Edge;
    /// let coords = vec![(0, 0), (1, 0), (2, 0)];
    ///
    /// let point_string = PointList::new(coords);
    ///
    /// let edges: Vec<_> = point_string.edges_reversed().collect();
    ///
    /// assert_eq!(edges, vec![Edge::new((2, 0), (1, 0)), Edge::new((1, 0), (0, 0))]);
    /// ```
    pub fn edges_reversed(&self) -> impl Iterator<Item = Edge<T>> + '_ {
        self.iter()
            .rev()
            .zip(self.iter().rev().skip(1))
            .map(|(a, b)| Edge::new(a, b))
    }
}

impl<T: Copy + Sub<Output = T> + NumCast> PointList<T> {
    /// Compute geometrical length of the path defined by the point string.
    /// # Examples
    ///
    /// ```
    /// use reda_geometry::shape::PointList;
    /// let coords = vec![(0, 0), (1, 0), (2, 0)];
    ///
    /// let point_string = PointList::new(coords);
    ///
    /// assert_eq!(point_string.path_length::<f64>(), 2.0);
    /// ```
    pub fn path_length<F: Float>(&self) -> F {
        self.iter()
            .zip(self.iter().skip(1))
            .map(|(a, b)| a.distance(b))
            .fold(F::zero(), |a, b| a + b)
    }
}

/// Create a point string from something that can be turned into an iterator of values convertible to [`Point`]s.
impl<I, T, P> From<I> for PointList<T>
where
    T: Copy,
    I: IntoIterator<Item = P>,
    Point<T>: From<P>,
{
    fn from(iter: I) -> Self {
        let points: Vec<Point<T>> = iter.into_iter().map(|x| x.into()).collect();

        PointList { points }
    }
}

/// Create a point string from a iterator of values convertible to [`Point`]s.
impl<T, P> FromIterator<P> for PointList<T>
where
    T: Copy,
    P: Into<Point<T>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
    {
        let points: Vec<Point<T>> = iter.into_iter().map(|x| x.into()).collect();

        PointList { points }
    }
}

impl<T: Copy + PartialOrd + Num> TryBoundingBox<T> for Path<T>
where
    T: Copy + PartialOrd,
{
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        let bbox = self.points.try_bounding_box();
        let _1 = T::one();
        let _2 = _1 + _1;
        bbox.map(|bbox| {
            // Enlarge it by width/2 in all directions to make sure the path is fully contained
            // in the bounding box.
            let (p1, p2) = (bbox.lower_left(), bbox.upper_right());
            let w_half = (self.width + _1) / _2;
            let width = Vec2::new(w_half, w_half);
            Rect::new(p1 - width, p2 + width)
        })
    }
}

impl<T> TryBoundingBox<T> for PointList<T>
where
    T: Copy + PartialOrd,
{
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        if self.points.is_empty() {
            None
        } else {
            let mut x_min = self.points[0].x;
            let mut x_max = x_min;
            let mut y_min = self.points[0].y;
            let mut y_max = y_min;

            for p in self.iter().skip(1) {
                let (x, y) = p.into();
                if x < x_min {
                    x_min = x;
                }
                if x > x_max {
                    x_max = x;
                }
                if y < y_min {
                    y_min = y;
                }
                if y > y_max {
                    y_max = y;
                }
            }

            Some(Rect::new((x_min, y_min), (x_max, y_max)))
        }
    }
}

impl<T> MapPoint<T> for PointList<T>
where
    T: Copy,
{
    fn transform<F: Fn(Point<T>) -> Point<T>>(&self, tf: F) -> Self {
        let points = self.points.iter().map(|&p| tf(p)).collect();
        PointList { points }
    }
}

impl<T: CoordType> Path<T> {
    pub fn rotate(&self, angle: Angle) -> Self {
        Path {
            points: self.points.rotate(angle),
            width: self.width,
            path_type: self.path_type,
        }
    }

    pub fn transform(&self, tf: &Transform<T>) -> Self {
        Self {
            points: self.points.transform(|p| tf.transform_point(p)),
            width: tf.transform_distance(self.width),
            path_type: self.path_type,
        }
    }
}

impl<T: Copy + NumCast, Dst: CoordType + NumCast> Cast<T, Dst> for PointList<T> {
    type Output = PointList<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        let new_points: Option<Vec<_>> = self.points.iter().map(|p| p.try_cast()).collect();
        new_points.map(|p| PointList::new(p))
    }
}

impl<T: CoordType + NumCast, Dst: CoordType + NumCast> Cast<T, Dst> for Path<T> {
    type Output = Path<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        let new_width = Dst::from(self.width);
        let new_points = self.points.try_cast();
        let new_path_type = match self.path_type {
            PathEndType::Extended(begin_ext, end_ext) => {
                let new_begin_ext = Dst::from(begin_ext);
                let new_end_ext = Dst::from(end_ext);
                match (new_begin_ext, new_end_ext) {
                    (Some(b), Some(e)) => Some(PathEndType::Extended(b, e)),
                    _ => None,
                }
            }
            PathEndType::Flat => Some(PathEndType::Flat),
            PathEndType::Round => Some(PathEndType::Round),
        };

        match (new_width, new_points, new_path_type) {
            (Some(width), Some(points), Some(path_type)) => Some(Path {
                points,
                width,
                path_type,
            }),
            _ => {
                // Failed to cast some values.
                None
            }
        }
    }
}