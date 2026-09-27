pub mod base;
pub mod shape;
pub mod prelude;
pub mod traits;
pub mod transform;
pub mod repetition;

use num_traits::{Num, NumCast};
use shape::{CoordType, Edge, Path, Point, Polygon, Polyline, Rect, Text};
use traits::{Cast, MapPoint, TryBoundingBox};
use transform::Transform;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Geometry<T> {
    Point(Point<T>),
    Edge(Edge<T>),
    Rect(Rect<T>),
    Polyline(Polyline<T>),
    Polygon(Polygon<T>),
    Path(Path<T>),
    Text(Text<T>),
}

macro_rules! geometry_from {
    ( $t:tt ) => {
        impl<T> From<$t<T>> for Geometry<T> {
            fn from(x: $t<T>) -> Geometry<T> {
                Geometry::$t(x)
            }
        }
    };
}

geometry_from!(Point);
geometry_from!(Edge);
geometry_from!(Rect);
geometry_from!(Polyline);
geometry_from!(Polygon);
geometry_from!(Path);
geometry_from!(Text);

impl<T: CoordType> Geometry<T> {
    pub fn transformed(&self, transform: &Transform<T>) -> Self {
        let trans = |p| transform.transform_point(p);
        match self {
            Geometry::Point(p) => transform.transform_point(*p).into(),
            Geometry::Edge(g) => g.transform(trans).into(),
            Geometry::Rect(g) => g.transform(trans).into(),
            Geometry::Polyline(g) => g.transform(trans).into(),
            Geometry::Polygon(g) => g.transform(trans).into(),
            Geometry::Path(p) => p.transform(transform).into(),
            Geometry::Text(g) => g.transform(trans).into(),
        }
    }
}

impl<T: Copy + PartialOrd + Num> TryBoundingBox<T> for Geometry<T> {
    fn try_bounding_box(&self) -> Option<Rect<T>> {
        match self {
            Geometry::Point(p) => p.try_bounding_box(),
            Geometry::Edge(e) => e.try_bounding_box(),
            Geometry::Rect(e) => e.try_bounding_box(),
            Geometry::Polyline(e) => e.try_bounding_box(),
            Geometry::Polygon(e) => e.try_bounding_box(),
            Geometry::Path(p) => p.try_bounding_box(),
            Geometry::Text(t) => t.try_bounding_box(),
        }
    }
}

impl<T: CoordType + NumCast, Dst: CoordType + NumCast> Cast<T, Dst>
    for Geometry<T>
{
    type Output = Geometry<Dst>;

    fn try_cast(&self) -> Option<Self::Output> {
        match self {
            Geometry::Point(p) => p.try_cast().map(|s| s.into()),
            Geometry::Edge(e) => e.try_cast().map(|s| s.into()),
            Geometry::Rect(r) => r.try_cast().map(|s| s.into()),
            Geometry::Polyline(p) => p.try_cast().map(|s| s.into()),
            Geometry::Polygon(p) => p.try_cast().map(|s| s.into()),
            Geometry::Path(p) => p.try_cast().map(|s| s.into()),
            Geometry::Text(t) => t.try_cast().map(|s| s.into()),
        }
    }
}
