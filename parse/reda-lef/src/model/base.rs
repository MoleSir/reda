use std::{fmt, str::FromStr};

use derive_new::new;

#[derive(Debug, Clone, Copy, Default, new)]
pub struct LefXy<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, Default, new)]
pub struct LefSize<T> {
    pub width: T,
    pub height: T,
}

//======================================================================================//
//            Units
//======================================================================================//

/// Units used in the library.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct LefUnits {
    /// Time in nano seconds.
    pub time_ns: u64,
    /// Capacitance in pico farads.
    pub capacitance_pf: u64,
    /// Resistance in ohms.
    pub resistance_ohms: u64,
    /// Power in milli watts.
    pub power_mw: u64,
    /// Current in milli amperes.
    pub current_ma: u64,
    /// Voltage in volts.
    pub voltage_v: u64,
    /// Length in micro meters.
    pub database_microns: u64,
    /// Frequency in mega hertz.
    pub frequency_mega_hz: u64,
}

//======================================================================================//
//            Transform
//======================================================================================//

/// Macro orientations that can be used by the placer.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct LefSymmetry {
    /// Mirroring macro at x-axis.
    pub x: bool,
    /// Mirroring macro at y-axis.
    pub y: bool,
    /// Rotating by 90 degrees. Intended for pad cells only.
    pub r90: bool,
}

impl LefSymmetry {
    /// Create a new symmetry definition.
    pub fn new(x: bool, y: bool, r90: bool) -> Self {
        Self { x, y, r90 }
    }

    /// Mirror symmetry at x-axis.
    pub fn x() -> Self {
        Self::new(true, false, false)
    }
    /// Mirror symmetry at y-axis.
    pub fn y() -> Self {
        Self::new(false, true, false)
    }
    /// Rotation by 90 degrees.
    pub fn r90() -> Self {
        Self::new(false, false, true)
    }

    /// Take the union of the both symmetry definitions.
    pub fn union(self, other: Self) -> Self {
        Self {
            x: self.x | other.x,
            y: self.y | other.y,
            r90: self.r90 | other.r90,
        }
    }
}

impl FromStr for LefSymmetry {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Self::x()),
            "Y" => Ok(Self::y()),
            "R90" => Ok(Self::r90()),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefSymmetry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x {
            f.write_str("X")?;
        }
        if self.y {
            f.write_str("Y")?;
        }
        if self.r90 {
            f.write_str("R90")?;
        }

        Ok(())
    }
}

/// Orientation, consists of rotation and mirroring.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LefOrient {
    /// North.
    N,
    /// South.
    S,
    /// East.
    E,
    /// West.
    W,
    /// Flipped North.
    FN,
    /// Flipped South.
    FS,
    /// FLipped East.
    FE,
    /// Flipped West.
    FW,
}

impl Default for LefOrient {
    fn default() -> Self {
        Self::N
    }
}

impl LefOrient {
    /// Decompose into a non-flipped orientation and a flag telling whether the orientation is flipped
    /// or not.
    /// Flipping is applied after rotation. Flips happen about the y-axis (switching left-right).
    ///
    /// Returns `(orientation, is_flipped)`.
    pub fn decomposed(&self) -> (Self, bool) {
        match self {
            LefOrient::FN => (LefOrient::N, true),
            LefOrient::FS => (LefOrient::S, true),
            LefOrient::FE => (LefOrient::E, true),
            LefOrient::FW => (LefOrient::W, true),
            other => (*other, false),
        }
    }

    /// Returns the flipped orientation.
    /// For example turns a `N` into a `FN`.
    pub fn flipped(&self) -> Self {
        use LefOrient::*;
        match self {
            N => FN,
            S => FS,
            E => FE,
            W => FW,
            FN => N,
            FS => S,
            FE => E,
            FW => W,
        }
    }
}

impl FromStr for LefOrient {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            "FN" => Ok(Self::FN),
            "FS" => Ok(Self::FS),
            "FE" => Ok(Self::FE),
            "FW" => Ok(Self::FW),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefOrient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::N => f.write_str("N"),
            Self::S => f.write_str("S"),
            Self::E => f.write_str("E"),
            Self::W => f.write_str("W"),
            Self::FN => f.write_str("FN"),
            Self::FS => f.write_str("FS"),
            Self::FE => f.write_str("FE"),
            Self::FW => f.write_str("FW"),
        }
    }
}

//======================================================================================//
//            Property
//======================================================================================//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LefPropertyType {
    Library,
    Macro,
    Layer,
}

#[derive(Debug, Clone)]
pub struct LefPropertyDefinition {
    pub prop_type: LefPropertyType,
    pub name: String,
    pub data_type: LefPropertyDataType,
}

/// Data type of a property value.
#[derive(Clone, Debug)]
pub enum LefPropertyDataType {
    /// Integer number.
    Integer,
    /// Floating point number.
    Real,
    /// String.
    String,
}

/// Value of a LEF/DEF property.
#[derive(Clone, Debug)]
pub enum LefPropertyValue {
    /// Integer.
    Int(i32),
    /// Floating point number.
    Real(f64),
    /// Quoted ASCII string.
    String(String),
}

impl FromStr for LefPropertyDataType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "INTEGER" => Ok(Self::Integer),
            "REAL" => Ok(Self::Real),
            "STRING" => Ok(Self::String),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefPropertyDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => f.write_str("INTEGER"),
            Self::Real => f.write_str("REAL"),
            Self::String => f.write_str("STRING"),
        }
    }
}

impl fmt::Display for LefPropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LefPropertyValue::Int(v) => write!(f, "{}", v),
            LefPropertyValue::Real(v) => write!(f, "{}", v),
            LefPropertyValue::String(v) => write!(f, r#""{}""#, v),
        }
    }
}

impl FromStr for LefPropertyType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "layer" => Ok(Self::Layer),
            "macro" => Ok(Self::Macro),
            "library" => Ok(Self::Library),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefPropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Library => f.write_str("LIBRARY"),
            Self::Macro => f.write_str("MACRO"),
            Self::Layer => f.write_str("LAYER"),
        }
    }
}