mod layer;
pub use layer::*;
mod via;
pub use via::*;
mod makro;
pub use makro::*;
mod base;
pub use base::*;
mod site;
pub use site::*;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Top-level structure of a LEF library.
/// 
/// ```lef
/// [VERSION statement]
/// [BUSBITCHARS statement]
/// [DIVIDERCHAR statement]
/// [UNITS statement]
/// [MANUFACTURINGGRID statement]
/// [USEMINSPACING statement]
/// [CLEARANCEMEASURE statement ;]
/// [PROPERTYDEFINITIONS statement]
/// [LAYER (Nonrouting) statement
///  | LAYER (Routing) statement] ...
/// [MAXVIASTACK statement]
/// [VIA statement] ...
/// [VIARULE statement] ...
/// [VIARULE GENERATE statement] ...
/// [NONDEFAULTRULE statement] ...
/// [SITE statement] ...
/// [BEGINEXT statement] ...
/// [END LIBRARY]
/// ```
#[derive(Default, Clone, Debug)]
pub struct LefTechnology {
    /// LEF version.
    pub version: Option<f64>,
    /// Characters used to indicate bus bits. Default is `[` and `]`.
    pub busbitchars: (char, char),
    /// Character used as path separator. Default is `/`.
    pub dividerchar: char,
    /// Units used in this library.
    pub units: LefUnits,
    /// Grid for geometrical alignment. Cells and shapes snap to locations on this grid.
    pub manufacturing_grid: Option<f64>,
    /// Type of distance measure (Euclidean: `dx^2 + dy^2`, MaxXY: `max(dx, dy)`)
    pub clearance_measure: LefClearanceMeasure,
    /// Layer definitions (masterslice, cut, routing, ...).
    pub layers: Vec<LefLayer>,
    /// Definitions of fixed VIAs by name.
    pub vias: HashMap<String, LefVia>,
    pub via_rules: HashMap<String, LefViaRule>,
    /// All SITE definitions by name.
    pub sites: HashMap<String, LefSite>,
    /// Prop define
    pub properties_definitions: Vec<LefPropertyDefinition>
}

/// ```lef
/// [VERSION statement]
/// [BUSBITCHARS statement]
/// [DIVIDERCHAR statement]
/// [VIA statement] ...
/// [SITE statement]
/// [MACRO statement
///     [PIN statement] ...
///     [OBS statement ...] ] ...
/// [BEGINEXT statement] ...
/// [END LIBRARY]
/// ```
#[derive(Default, Clone, Debug)]
pub struct LefCellLibrary {
    /// LEF version.
    pub version: Option<f64>,
    /// Characters used to indicate bus bits. Default is `[` and `]`.
    pub busbitchars: (char, char),
    /// Character used as path separator. Default is `/`.
    pub dividerchar: char,
    /// Definitions of fixed VIAs by name.
    pub vias: HashMap<String, LefVia>,
    /// site use by name.
    pub sites: HashMap<String, LefSite>,
    /// macro definitions of the library.
    pub macros: HashMap<String, LefMacro>,
}

impl LefTechnology {
    pub fn new() -> Self {
        Self {
            version: None,
            busbitchars: ('[', ']'),
            dividerchar: '/',
            ..Default::default()
        }
    }
}

impl LefCellLibrary {
    pub fn new() -> Self {
        Self {
            version: None,
            busbitchars: ('[', ']'),
            dividerchar: '/',
            ..Default::default()
        }
    }
}

/// Type of distance measurement
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LefClearanceMeasure {
    /// Take maximum of x or y distance.
    Maxxy,
    /// `sqrt(x^2 + y^2)`
    Euclidean,
}

impl Default for LefClearanceMeasure {
    fn default() -> Self {
        Self::Euclidean
    }
}

impl FromStr for LefClearanceMeasure {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "MAXXY" => Ok(Self::Maxxy),
            "EUCLIDEAN" => Ok(Self::Euclidean),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefClearanceMeasure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Maxxy => f.write_str("MAXXY"),
            Self::Euclidean => f.write_str("EUCLIDEAN"),
        }
    }
}
