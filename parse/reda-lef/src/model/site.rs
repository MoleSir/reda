use std::{fmt, str::FromStr};

use super::{LefOrient, LefSize, LefSymmetry};

/// SITE definition.
/// ```lef
/// SITE siteName
///     CLASS {PAD | CORE} ;
///     [SYMMETRY {X | Y | R90} ... ;]
///     [ROWPATTERN {previousSiteName siteOrient} ... ;]
///     SIZE width BY height ;
/// END siteName
/// ```
#[derive(Clone, Debug, Default)]
pub struct LefSite {
    /// Name of the site.
    pub name: String,
    /// Dimensions of the site.
    pub size: LefSize<f64>,
    /// Define orientations of the site that are considered equivalent.
    /// This is used for example to specify whether cell flipping inside
    /// a row is allowed.
    pub symmetry: LefSymmetry,
    /// Specify site type (IO or CORE).
    pub class: LefSiteClass,
    /// Construct this site as a composition of previously defined sites.
    /// List of tuples: (previous site, orientation)
    pub row_pattern: Vec<(String, LefOrient)>,
}

/// [SITE siteName [sitePattern] ;]
#[derive(Clone, Debug, Default)]
pub struct LefSiteReference {
    /// site name 
    pub name: String,
    pub pattern: Option<LefSiteRefPattern>,
}

#[derive(Clone, Debug, Default)]
pub struct LefSiteRefPattern {
    pub x_origin: f64,
    pub y_origin: f64,
    pub site_orient: LefOrient,
    pub step_pattern: Option<LefSiteRefStepPattern>
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LefSiteRefStepPattern {
    pub x_step: f64,
    pub y_step: f64,
    pub x_start: f64,
    pub y_start: f64,
}

/// Specify the type of a site: Either IO site (PAD) or core site (CORE).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefSiteClass {
    /// A core site.
    CORE,
    /// An IO site.
    PAD,
}

impl Default for LefSiteClass {
    fn default() -> Self {
        Self::CORE
    }
}

impl FromStr for LefSiteClass {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "CORE" => Ok(Self::CORE),
            "PAD" => Ok(Self::PAD),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefSiteClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CORE => f.write_str("CORE")?,
            Self::PAD => f.write_str("PAD")?,
        };
        Ok(())
    }
}