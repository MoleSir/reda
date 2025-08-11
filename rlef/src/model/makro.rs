
pub struct LefMacro {
    pub name: String,
    pub class: u8,
    pub foreign_cell: (String, Option<(f64, f64)>, Option<u8>),
    pub origin: (f64, f64),
    pub eeq_macro: Option<String>,
    pub size: (f64, f64),
    pub symmetry: Vec<u8>,
    pub site: Vec<LefMacroSite>,
    pub pin: Vec<LefMacroPin>,
    pub obs: Option<Vec<LefPortShape>>,
    pub density: Option<LefMacroDensity>,
}

pub struct LefMacroSite {
    pub name: String,
    pub pattern: Option<String>,
}

pub struct LefMacroPin {
    pub pin_name: String,
    pub taper_rule: Option<String>,
    pub direction: u8,
    pub use_type: u8,
    pub net_expr: Option<String>,
    pub ground_sensitivity: Option<String>,
    pub supply_sensitivity: Option<String>,
    pub mustjoin: Option<String>,
    pub shape: Option<u8>,
    pub pin_port: Vec<LefPortShape>, // (class,MacroPortObj) // assume only one port in each pin
                                  // pub pin_antenna: Option<MacroPinAntenna>,
}

pub struct LefPortShape {
    pub layer_name: String, // layer name
    pub geometries: Vec<LefPortGeometry>,
}

pub enum LefPortGeometry {
    Path(Vec<(f64, f64)>),
    Rect(((f64, f64), (f64, f64))),
    Polygon(Vec<(f64, f64)>),
    Via((String, (f64, f64))),
}

pub struct LefMacroDensity {
    pub layer_name: String,
    pub rect_region: Vec<(((f64, f64), (f64, f64)), f64)>,
}
