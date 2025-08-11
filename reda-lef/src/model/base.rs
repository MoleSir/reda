
#[derive(Debug, Clone, Default)]
pub struct LefUnits {
    pub time: Option<f64>,         // NANOSECONDS
    pub capacitance: Option<f64>,  // PICOFARADS
    pub resistance: Option<f64>,   // OHMS
    pub power: Option<f64>,        // MILLIWATTS
    pub current: Option<f64>,      // MILLIAMPS
    pub voltage: Option<f64>,      // VOLTS
    pub database_microns: Option<u32>, // LEFconvertFactor
    pub frequency: Option<f64>,    // MEGAHERTZ
}


#[derive(Debug, Clone, Copy)]
pub enum LefUseMinSpacing {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LefClearanceMeasure {
    MaxXY,
    Euclidean,
}

impl Default for LefClearanceMeasure {
    fn default() -> Self {
        Self::Euclidean
    }
}

#[derive(Debug, Clone)]
pub struct LefVia {
    pub name: String,
    pub is_default: bool,
    pub rule: Option<LefViaRule>,
    pub layers: Vec<LefViaLayer>,
    pub properties: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct LefViaRule {
    pub rule_name: String,
    pub cut_size: (f64, f64),
    pub layers: (String, String, String),
    pub cut_spacing: (f64, f64),
    pub enclosure: (f64, f64, f64, f64),
    pub row_col: Option<(u32, u32)>,
    pub origin: Option<(f64, f64)>,
    pub offset: Option<(f64, f64, f64, f64)>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LefViaLayer {
    pub layer_name: String,
    pub shapes: Vec<LefViaGeometry>,
}

#[derive(Debug, Clone)]
pub enum LefViaGeometry {
    Rect {
        mask: Option<u32>,
        lower_left: (f64, f64),
        upper_right: (f64, f64),
    },
    Polygon {
        mask: Option<u32>,
        points: Vec<(f64, f64)>,
    },
}
