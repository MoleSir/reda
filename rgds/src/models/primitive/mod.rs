mod presentation;
mod transform;

pub use presentation::*;
pub use transform::*;

use std::fmt;
use chrono::{Datelike, Timelike, Local};

/// This record contains a value that describes the type of path endpoints. The value is
/// - 0 for square-ended paths that endflush with their endpoints
/// - 1 for round-ended paths
/// - 2 for square-ended paths that extend a half-width beyond their endpoints
#[derive(Debug, Clone, Copy)]
pub enum GdsPathType {
    SquareEnd = 0,
    RoundEnd = 1,
    SquareEndExtend = 2,
}

impl Default for GdsPathType {
    fn default() -> Self {
        Self::SquareEnd
    }
}

impl GdsPathType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::SquareEnd),
            1 => Some(Self::RoundEnd),
            2 => Some(Self::SquareEndExtend),
            _ => None,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Self::SquareEnd => 0,
            Self::RoundEnd => 1,
            Self::SquareEndExtend => 2,
        }
    }
}

impl std::fmt::Display for GdsPathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            GdsPathType::SquareEnd => "SquareEnd (0)",
            GdsPathType::RoundEnd => "RoundEnd (1)",
            GdsPathType::SquareEndExtend => "SquareEndExtend (2)",
        };
        write!(f, "{}", description)
    }
}

/// Defines the format of a Stream tape in two bytes. The possible values are:
/// 1. for GDSII Archive format
/// 2. for GDSII Filtered format
/// 3. for EDSM Archive format
/// 4. for EDSHI Filtered forrnat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdsFormat {
    /// GDSII Archive format
    GdsiiArchive = 0,
    /// GDSII Filtered format
    GdsiiFiltered = 1,
    /// EDSM Archive format
    EdsmArchive = 2,
    /// EDSHI Filtered format
    EdshiFiltered = 3,
}

impl GdsFormat {
    /// Converts a `u16` value into a `GdsFormat`, if valid.
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(GdsFormat::GdsiiArchive),
            1 => Some(GdsFormat::GdsiiFiltered),
            2 => Some(GdsFormat::EdsmArchive),
            3 => Some(GdsFormat::EdshiFiltered),
            _ => None,
        }
    }

    /// Converts a `GdsFormat` into a `u16` value.
    pub fn to_u16(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone)]
pub struct GdsDateTime {
    pub year: i16,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    pub minute: i16,
    pub second: i16,
}

impl Default for GdsDateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl GdsDateTime {
    pub fn now() -> Self {
        let now = Local::now();
        GdsDateTime {
            year: now.year() as i16,
            month: now.month() as i16,
            day: now.day() as i16,
            hour: now.hour() as i16,
            minute: now.minute() as i16,
            second: now.second() as i16,
        }
    }
}

impl fmt::Display for GdsDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

#[derive(Debug, Clone)]
pub struct GdsDbCoord {
    pub x: i32,
    pub y: i32,
}

impl GdsDbCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for GdsDbCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}