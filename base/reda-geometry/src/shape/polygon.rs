use itertools::Itertools;
use num_traits::NumCast;

use crate::traits::{Cast, MapPoint, TryBoundingBox};

use super::{CoordType, Edge, Point, Polyline, Rect};

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub struct Polygon<T> {
    pub exterior: Polyline<T>,
    pub interiors: Vec<Polyline<T>>, 
}

impl<T> Polygon<T> {
    pub fn new<E, I>(exterior: E, holes: Vec<I>) -> Self 
    where 
        E: Into<Polyline<T>>,
        I: Into<Polyline<T>>,
    {
        Self {
            exterior: exterior.into(),
            interiors: holes.into_iter().map(|i| i.into()).collect()
        }
    }

    pub fn new_exterior(exterior: impl Into<Vec<Point<T>>>) -> Self {
        Self {
            exterior: Polyline::from_points(exterior),
            interiors: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            exterior: Polyline::empty(),
            interiors: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.exterior.len()
    }

    pub fn is_empty(&self) -> bool {
        self.exterior.is_empty()
    }
}   

impl<T: Copy> Polygon<T> {
    pub fn exterior_edges(&self) -> Vec<Edge<T>> {
        self.exterior.edges()
    }

    pub fn exterior_edges_iter(&self) -> impl Iterator<Item = Edge<T>> + '_ {
        self.exterior.edges_iter()
    }

    pub fn all_edges(&self) -> Vec<Edge<T>> {
        self.all_edges_iter().collect()
    }

    pub fn all_edges_iter(&self) -> impl Iterator<Item = Edge<T>> + '_ {
        self.exterior.edges_iter().chain(
            self.interiors
                .iter()
                .flat_map(|interior| interior.edges_iter())
        )
    }
}

impl<T: PartialOrd> Polygon<T> {
    pub fn new_normalized<E, I>(exterior: E, holes: Vec<I>) -> Self 
    where 
        E: Into<Polyline<T>>,
        I: Into<Polyline<T>>,
    {
        Self::new(exterior, holes).normalized()
    }
    
    pub fn new_exterior_normalized(exterior: impl Into<Vec<Point<T>>>) -> Self {
        Self::new_exterior(exterior).normalized()
    }

    /// Reorder vertices and holes to get the lexicographically smallest representation of this polygon.
    /// Does not change the orientations.
    pub fn normalize(&mut self) {
        self.exterior.normalize();
        self.interiors.iter_mut().for_each(|p| p.normalize());
        self.interiors.sort_by(|a, b| {
            a.points()
                .partial_cmp(b.points())
                .unwrap_or(std::cmp::Ordering::Less)
        })
    }

    /// Reorder vertices and holes to get the lexicographically smallest representation of this polygon.
    /// Does not change the orientations.
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<'a, T, P> From<&'a [P]> for Polygon<T>
where
    T: CoordType,
    Point<T>: From<&'a P>,
{
    fn from(vec: &'a [P]) -> Self {
        Self {
            exterior: vec.into(),
            interiors: Vec::new(),
        }
    }
}

impl<T, P> From<Vec<P>> for Polygon<T>
where
    T: Copy + PartialOrd,
    Point<T>: From<P>,
{
    fn from(vec: Vec<P>) -> Self {
        Self {
            exterior: vec.into(),
            interiors: Vec::new(),
        }
    }
}

impl<T> From<Polyline<T>> for Polygon<T> {
    fn from(polyline: Polyline<T>) -> Self {
        Self {
            exterior: polyline,
            interiors: Vec::new(),
        }
    }
}

impl<T> From<&Polyline<T>> for Polygon<T> 
where 
    T: Clone
{
    fn from(polyline: &Polyline<T>) -> Self {
        Self {
            exterior: polyline.clone(),
            interiors: Vec::new(),
        }
    }
}

impl<T> From<Rect<T>> for Polygon<T>
where
    T: Copy,
{
    fn from(rect: Rect<T>) -> Self {
        Self::from(&rect)
    }
}

impl<T> From<&Rect<T>> for Polygon<T>
where
    T: Copy,
{
    fn from(rect: &Rect<T>) -> Self {
        Polyline::from_rect(rect).into()
    }
}

impl<T: Copy> MapPoint<T> for Polygon<T> {
    fn transform<F>(&self, transformate: F) -> Self
    where 
        F: Fn(Point<T>) -> Point<T> 
    {
        Self {
            exterior: self.exterior.transform(&transformate),
            interiors: self.interiors.iter().map(|p| p.transform(&transformate)).collect(),
        }    
    }
}

impl<T: Copy + PartialOrd> TryBoundingBox<T> for Polygon<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        self.exterior.try_bounding_box()  
    }
}

impl<T: CoordType + NumCast, Dst: CoordType + NumCast> Cast<T, Dst>
    for Polygon<T>
{
    type Output = Polygon<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        if let Some(new_hull) = self.exterior.try_cast() {
            let new_holes: Vec<_> = self
                .interiors
                .iter()
                .map(|hole| hole.try_cast())
                .while_some()
                .collect();
            if new_holes.len() == self.interiors.len() {
                Some(Polygon::new(new_hull, new_holes))
            } else {
                None
            }
        } else {
            None
        }
    }
}