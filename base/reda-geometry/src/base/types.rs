use std::ops::{Add, Neg, Sub};
use num_traits::Signed;
/// Precision for distance related decisions.
pub const PREC_DISTANCE: DistanceType = 1e-5;
pub type FloatType = f64;
pub type DistanceType = FloatType;

/// Angle expressed as a multiple of 90 degrees.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum Angle {
    /// 0 Degrees.
    #[default]
    R0,
    /// 90 Degrees.
    R90,
    /// 180 Degrees.
    R180,
    /// 270 Degrees.
    R270,
}

impl Angle {
    pub fn to_int(&self) -> u32 {
        match self {
            Angle::R0 => 0,
            Angle::R90 => 1,
            Angle::R180 => 2,
            Angle::R270 => 3,
        }
    }

    pub fn from_int(a: u32) -> Self {
        match a % 4 {
            0 => Angle::R0,
            1 => Angle::R90,
            2 => Angle::R180,
            3 => Angle::R270,
            _ => unreachable!(),
        }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_int(self.to_int() + rhs.to_int())
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_int(self.to_int() + 4 - rhs.to_int())
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_int(4 - self.to_int())
    }
}

/// Location relative to a directed line or edge.
/// Something can be on the left of the line, right of the line or on top of the line (center).
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Side {
    Left,
    Center,
    Right,
}

impl Side {
    pub fn is_left(&self) -> bool {
        *self == Side::Left
    }

    pub fn is_right(&self) -> bool {
        *self == Side::Right
    }

    pub fn is_center(&self) -> bool {
        *self == Side::Center
    }

    pub fn other(&self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Center => Side::Center,
            Side::Right => Side::Left,
        }
    }
}

/// Relative orientation of two geometrical objects such as vectors.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Orientation {
    ClockWise,
    CounterClockWise,
    Straight,
}

impl Orientation {
    pub fn is_clock_wise(self) -> bool {
        self == Self::ClockWise
    }

    pub fn is_counter_clock_wise(self) -> bool {
        self == Self::CounterClockWise
    }

    pub fn is_straight(self) -> bool {
        self == Self::Straight
    }
}

/// This is a result type for containment checks.
/// * `No` Not inside.
/// * `OnBounds` Lies on the boundaries.
/// * `WithinBounds` Fully inside.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contains {
    No,
    OnBounds,
    WithinBounds,
}

impl Contains {
    pub fn is_within_bounds(&self) -> bool {
        matches!(self, Contains::WithinBounds)
    }

    /// Tells if the point is contained or lies on the bounds.
    pub fn inclusive_bounds(&self) -> bool {
        matches!(
            self,
            Contains::WithinBounds | Contains::OnBounds
        )
    }

    pub fn on_bounds(&self) -> bool {
        matches!(self, Contains::OnBounds)
    }

    pub fn is_no(&self) -> bool {
        matches!(self, Contains::No)
    }

    /// Returns the stronger result of the both.
    /// Ordering from weak to strong is `No`, `OnBounds`, `WithinBounds`
    pub fn max(self, other: Self) -> Self {
        match (self, other) {
            (Contains::WithinBounds, _) => Contains::WithinBounds,
            (_, Contains::WithinBounds) => Contains::WithinBounds,
            (Contains::OnBounds, _) => Contains::OnBounds,
            (_, Contains::OnBounds) => Contains::OnBounds,
            (Contains::No, _) => Contains::No,
        }
    }

    /// Returns the weaker result of the both.
    /// Ordering from weak to strong is `No`, `OnBounds`, `WithinBounds`
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (Contains::No, _) => Contains::No,
            (_, Contains::No) => Contains::No,
            (Contains::OnBounds, _) => Contains::OnBounds,
            (_, Contains::OnBounds) => Contains::OnBounds,
            (Contains::WithinBounds, _) => Contains::WithinBounds,
        }
    }
}

/// Direction in one dimension.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction1D {
    /// Towards negative coordinates.
    Low,
    /// Towards positive coordinates.
    High,
}

impl Direction1D {
    /// Return the opposite direction.
    pub fn reversed(self) -> Self {
        match self {
            Direction1D::Low => Direction1D::High,
            Direction1D::High => Direction1D::Low,
        }
    }

    /// Map `Low` to `-1` and `High` to `+1`.
    pub fn sign<N>(&self) -> N
    where
        N: Signed,
    {
        match self {
            Direction1D::Low => N::one().neg(),
            Direction1D::High => N::one(),
        }
    }
}

impl From<Direction2D> for Direction1D {
    /// Downcast 2D direction to 1D direction by preserving the sign.
    fn from(d2: Direction2D) -> Self {
        match d2 {
            Direction2D::Right | Direction2D::Up => Direction1D::High,
            Direction2D::Left | Direction2D::Down => Direction1D::Low,
        }
    }
}

/// Orientation in two dimensions along one of the axes.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Orientation2D {
    /// Oriented parallel to the x axis.
    Horizontal,
    /// Oriented parallel to the y axis.
    Vertical,
}

impl Orientation2D {
    /// Get the orthogonal orientation.
    pub fn other(self) -> Self {
        match self {
            Orientation2D::Horizontal => Self::Vertical,
            Orientation2D::Vertical => Self::Horizontal,
        }
    }

    /// Get the positive or negative direction aligned with this orientation.
    pub fn direction(self, d1: Direction1D) -> Direction2D {
        use Direction1D::*;
        use Direction2D::*;
        use Orientation2D::*;
        match (self, d1) {
            (Horizontal, Low) => Right,
            (Horizontal, High) => Left,
            (Vertical, Low) => Down,
            (Vertical, High) => Up,
        }
    }

    /// Check if orientation is vertical.
    pub fn is_vertical(&self) -> bool {
        self == &Self::Vertical
    }

    /// Check if orientation is horizontal.
    pub fn is_horizontal(&self) -> bool {
        self == &Self::Horizontal
    }
}

/// Directions along the coordinate axes in two dimensions.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction2D {
    /// Positive x direction.
    Left,
    /// Positive y direction.
    Up,
    /// Negative x direction.
    Right,
    /// Negative y direction.
    Down,
}

impl Direction2D {
    /// Get the opposite direction.
    pub fn reversed(self) -> Self {
        match self {
            Direction2D::Left => Self::Right,
            Direction2D::Up => Self::Down,
            Direction2D::Right => Self::Left,
            Direction2D::Down => Self::Up,
        }
    }

    /// Get the orientation of this direction.
    pub fn orientation(&self) -> Orientation2D {
        match self {
            Direction2D::Left | Direction2D::Right => Orientation2D::Horizontal,
            Direction2D::Up | Direction2D::Down => Orientation2D::Vertical,
        }
    }

    /// Get the sign of the direction.
    pub fn sign<N>(self) -> N
    where
        N: Signed,
    {
        let d1: Direction1D = self.into();
        d1.sign()
    }
}
