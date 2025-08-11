mod base;
mod makro;
mod layer;

use std::{path::Path, str::FromStr};

pub use base::*;
pub use layer::*;
pub use makro::*;

use derive_builder::Builder;

use crate::io::read::{self, LefReadError};

/*
    [VERSION statement]
    [BUSBITCHARS statement]
    [DIVIDERCHAR statement]
    [UNITS statement]
    [MANUFACTURINGGRID statement]
    [USEMINSPACING statement]
    [CLEARANCEMEASURE statement ;]
    [PROPERTYDEFINITIONS statement]
    [FIXEDMASK ;]
    [LAYER (Nonrouting) statement
    | LAYER (Routing) statement] ...
    [MAXVIASTACK statement]
    [VIA statement] ...
    [VIARULE statement] ...
    [VIARULE GENERATE statement] ...
    [NONDEFAULTRULE statement] ...
    [SITE statement] ...
    [BEGINEXT statement] ...
    [END LIBRARY]
*/
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefTechLibrary {
    pub version: f64,
    pub busbitchar: String,
    pub dividechar: String,
    pub units: LefUnits,
    #[builder(default)]
    pub manufacturing_grid: Option<f64>,
    #[builder(default)]
    pub use_min_spacing: Option<LefUseMinSpacing>,
    #[builder(default)]
    pub layers: Vec<LefLayer>,
}

impl LefTechLibrary {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, LefReadError> {
        let path = path.as_ref();
        let s = std::fs::read_to_string(path)?;
        Self::from_str(&s)
    }
}

impl FromStr for LefTechLibrary {
    type Err = LefReadError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match read::tech_library(s) {
            Ok((_, lib)) => Ok(lib),
            Err(e) => Err(LefReadError::Parse(e.to_string())),
        }
    }
}

/*
    [VERSION statement]
    [BUSBITCHARS statement]
    [DIVIDERCHAR statement]
    [VIA statement] ...
    [SITE statement]
    [MACRO statement
    [PIN statement] ...
    [OBS statement ...] ] ...
    [BEGINEXT statement] ...
    [END LIBRARY]
*/
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefCellLibrary {
    pub version: f64,
    pub busbitchar: String,
    pub dividechar: String,
    pub vias: Vec<LefVia>,
    
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::LefTechLibrary;

    #[test]
    fn test_lef_tech_read() {
        match LefTechLibrary::read_from("./data/freesdk45_tech.lef") {
            Ok(lib) => {},
            Err(e) => println!("{}", e),
        }
    }
}