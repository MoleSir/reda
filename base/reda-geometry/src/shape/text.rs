use num_traits::NumCast;

use crate::shape::Point;
use crate::traits::{BoundingBox, Cast, MapPoint, TryBoundingBox};
use std::fmt;
use std::hash::Hash;
use std::ops::Deref;

use super::Rect;

pub trait TextType: Eq + Hash + Clone + fmt::Debug {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Text<T, S = String> {
    location: Point<T>,
    text: S,
}

impl<T, S: TextType> Text<T, S> {
    /// Create a new text object.
    pub fn new(text: S, location: Point<T>) -> Self {
        Text { location, text }
    }

    /// Get a reference to the text string.
    pub fn text(&self) -> &S {
        &self.text
    }
}

impl<T: Copy, S: TextType> Text<T, S> {
    /// Get location of the text label.
    #[inline]
    pub fn location(&self) -> Point<T> {
        self.location
    }

    /// Get x-coordinate of the label location.
    #[inline]
    pub fn x(&self) -> T {
        self.location.x
    }

    /// Get y-coordinate of the label location.
    #[inline]
    pub fn y(&self) -> T {
        self.location.y
    }
}

/// Display format of the text label.
impl<T, S> fmt::Display for Text<T, S>
where
    T: fmt::Display,
    S: TextType + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text({}, {})", self.text, self.location)
    }
}

impl<T, S> Deref for Text<T, S>
where
    S: Deref<Target = String>,
{
    type Target = String;

    /// Dereference to String.
    fn deref(&self) -> &Self::Target {
        self.text.deref()
    }
}

impl<T, S> MapPoint<T> for Text<T, S>
where
    T: Copy,
    S: Clone,
{
    /// Point wise transformation.
    #[inline]
    fn transform<F>(&self, transformation: F) -> Self
    where
        F: Fn(Point<T>) -> Point<T>,
    {
        Text {
            location: transformation(self.location),
            text: self.text.clone(),
        }
    }
}

impl<T: Copy + PartialOrd, S> BoundingBox<T> for Text<T, S> {
    fn bounding_box(&self) -> Rect<T> {
        Rect::new(self.location, self.location)
    }
}

impl<T: Copy + PartialOrd, S> TryBoundingBox<T> for Text<T, S> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        Some(Rect::new(self.location, self.location))
    }
}

impl<T, Dst, S> Cast<T, Dst> for Text<T, S>
where
    T: Copy + NumCast,
    Dst: Copy + NumCast,
    S: Clone,
{
    type Output = Text<Dst, S>;

    fn try_cast(&self) -> Option<Self::Output> {
        self.location.try_cast().map(|loc| Text {
            location: loc,
            text: self.text.clone(),
        })
    }
}