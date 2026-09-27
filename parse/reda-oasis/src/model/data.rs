use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;

#[derive(Debug, PartialEq, Clone)]
pub enum Real {
    PositiveWhole(BigUint),
    NegativeWhole(BigUint),
    PositiveReciprocal(BigUint),
    NegativeReciprocal(BigUint),
    PositiveRatio(BigUint, BigUint),
    NegativeRatio(BigUint, BigUint),
    SinglePrecisionFloat(f32),
    DoublePrecisionFloat(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction1 {
    Positive, // east/north
    Negative, // west/south
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction2 {
    East,
    North,
    West,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction3 {
    East,
    North,
    West,
    South,
    Northeast,
    Northwest,
    Southwest,
    Southeast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta1 { 
    pub magnitude: BigUint, 
    pub direction: Direction1
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta2 { 
    pub magnitude: BigUint, 
    pub direction: Direction2
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delta3 { 
    pub magnitude: BigUint, 
    pub direction: Direction3
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GDelta {
    G1{ magnitude: BigUint, direction: Direction3 },
    G2{ x: BigUint, x_dir: Direction1, y: BigUint, y_dir: Direction1 }
}

pub trait Delta2D {
    fn delta_x(&self) -> BigInt;
    fn delta_y(&self) -> BigInt;
}

impl Delta2D for Delta2 {
    fn delta_x(&self) -> BigInt {
        match self.direction {
            Direction2::East  => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction2::West  => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction2::North => BigInt::zero(),
            Direction2::South => BigInt::zero(),
        }
    }

    fn delta_y(&self) -> BigInt {
        match self.direction {
            Direction2::North => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction2::South => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction2::East  => BigInt::zero(),
            Direction2::West  => BigInt::zero(),
        }
    }
}

impl Delta2D for Delta3 {
    fn delta_x(&self) -> BigInt {
        match self.direction {
            Direction3::East       => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::West       => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction3::North | Direction3::South => BigInt::zero(),
            Direction3::Northeast  => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::Northwest  => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction3::Southeast  => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::Southwest  => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
        }
    }

    fn delta_y(&self) -> BigInt {
        match self.direction {
            Direction3::North      => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::South      => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction3::East | Direction3::West => BigInt::zero(),
            Direction3::Northeast  => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::Northwest  => BigInt::from_biguint(Sign::Plus, self.magnitude.clone()),
            Direction3::Southeast  => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
            Direction3::Southwest  => BigInt::from_biguint(Sign::Minus, self.magnitude.clone()),
        }
    }
}

impl Delta2D for GDelta {
    fn delta_x(&self) -> BigInt {
        match self {
            GDelta::G1 { magnitude, direction } => {
                match direction {
                    Direction3::East | Direction3::Northeast | Direction3::Southeast => 
                        BigInt::from_biguint(Sign::Plus, magnitude.clone()),
                    Direction3::West | Direction3::Northwest | Direction3::Southwest => 
                        BigInt::from_biguint(Sign::Minus, magnitude.clone()),
                    Direction3::North | Direction3::South => BigInt::zero(),
                }
            },
            GDelta::G2 { x, x_dir, .. } => {
                let sign = if *x_dir == Direction1::Negative { Sign::Minus } else { Sign::Plus };
                BigInt::from_biguint(sign, x.clone())
            }
        }
    }

    fn delta_y(&self) -> BigInt {
        match self {
            GDelta::G1 { magnitude, direction } => {
                match direction {
                    Direction3::North | Direction3::Northeast | Direction3::Northwest => 
                        BigInt::from_biguint(Sign::Plus, magnitude.clone()),
                    Direction3::South | Direction3::Southeast | Direction3::Southwest => 
                        BigInt::from_biguint(Sign::Minus, magnitude.clone()),
                    Direction3::East | Direction3::West => BigInt::zero(),
                }
            },
            GDelta::G2 { y, y_dir, .. } => {
                let sign = if *y_dir == Direction1::Negative { Sign::Minus } else { Sign::Plus };
                BigInt::from_biguint(sign, y.clone())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum OString {
    B(BString),
    A(AString),
    N(NString),
}

#[derive(Debug, Clone)]
pub struct BString(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct AString(pub String);

#[derive(Debug, Clone)]
pub struct NString(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Repetition {
    /// TYPE 0: 重用上一次描述
    Reuse,

    /// TYPE 1: N x M 均匀矩阵
    UniformMatrix {
        x_dimension: BigUint,
        y_dimension: BigUint,
        x_space: BigUint,
        y_space: BigUint,
    },

    /// TYPE 2: N 元素均匀水平向量
    UniformRow {
        x_dimension: BigUint,
        x_space: BigUint,
    },

    /// TYPE 3: M 元素均匀垂直向量
    UniformColumn {
        y_dimension: BigUint,
        y_space: BigUint,
    },

    /// TYPE 4: N 元素非均匀水平向量
    VariableRow {
        x_dimension: BigUint,
        x_spaces: Vec<BigUint>,
    },

    /// TYPE 5: TYPE 4 + grid
    VariableRowGrid {
        x_dimension: BigUint,
        x_spaces: Vec<BigUint>,
        grid: BigUint,
    },

    /// TYPE 6: M 元素非均匀垂直向量
    VariableColumn {
        y_dimension: BigUint,
        y_spaces: Vec<BigUint>,
    },

    /// TYPE 7: TYPE 6 + grid
    VariableColumnGrid {
        y_dimension: BigUint,
        y_spaces: Vec<BigUint>,
        grid: BigUint,
    },

    /// TYPE 8: N x M 矩阵，可对角位移
    MatrixDisplacement {
        n_dimension: BigUint,
        m_dimension: BigUint,
        n_displacement: GDelta,
        m_displacement: GDelta,
    },

    /// TYPE 9: P 元素向量，可对角位移
    VectorDisplacement {
        dimension: BigUint,
        displacement: GDelta,
    },

    /// TYPE 10: P 元素向量，非均匀二维位移
    VariableVector {
        dimension: BigUint,
        displacements: Vec<GDelta>,
    },

    /// TYPE 11: TYPE 10 + grid
    VariableVectorGrid {
        dimension: BigUint,
        displacements: Vec<GDelta>,
        grid: BigUint,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointList {
    /// Alternating horizontal/vertical 1-deltas, first delta horizontal
    AlternatingHV(Vec<Delta1>),

    /// Alternating vertical/horizontal 1-deltas, first delta vertical
    AlternatingVH(Vec<Delta1>),

    /// Manhattan 2-deltas (horizontal or vertical relative moves)
    Manhattan(Vec<Delta2>),

    /// Octangular 3-deltas (8 directions relative moves)
    Octangular(Vec<Delta3>),

    /// Any-angle g-deltas (arbitrary relative moves)
    AnyAngle(Vec<GDelta>),

    /// Any-angle g-deltas with cumulative vector (initial vector (0,0))
    CumulativeAnyAngle(Vec<GDelta>),
}
