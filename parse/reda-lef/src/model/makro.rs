use std::{collections::HashMap, fmt, str::FromStr};
use derive_new::new;
use reda_geometry::shape::{Point, PointList, Rect};
use super::{LefOrient, LefPropertyValue, LefSiteReference, LefSize, LefSymmetry};

/// DENSITY specification for a MACRO.
#[derive(Clone, Debug)]
pub struct LefMacroDensity {
    pub layer_name: String,
    pub rectangles: Vec<Rect<f64>>,
}

/// Geometry class for PIN PORT geometries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LefGeometryClass {
    /// No special class.
    None,
    /// Core geometry.
    Core,
    /// Bump geometry.
    Bump,
}

impl FromStr for LefGeometryClass {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NONE" => Ok(Self::None),
            "CORE" => Ok(Self::Core),
            "BUMP" => Ok(Self::Bump),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefGeometryClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("NONE"),
            Self::Core => f.write_str("CORE"),
            Self::Bump => f.write_str("BUMP"),
        }
    }
}

/// MACRO definition.
/// ```lef
/// MACRO macroName
///     [CLASS
///        { COVER [BUMP]
///        | RING
///        | BLOCK [BLACKBOX | SOFT]
///        | PAD [INPUT | OUTPUT |INOUT | POWER | SPACER | AREAIO]
///        | CORE [FEEDTHRU | TIEHIGH | TIELOW | SPACER | ANTENNACELL | WELLTAP]
///        | ENDCAP {PRE | POST | TOPLEFT | TOPRIGHT | BOTTOMLEFT | BOTTOMRIGHT}
///        }
///     ;]
///     [FOREIGN foreignCellName [pt [orient]] ;] ...
///     [ORIGIN pt ;]
///     [EEQ macroName ;]
///     [SIZE width BY height ;]
///     [SYMMETRY {X | Y | R90} ... ;]
///     [SITE siteName [sitePattern] ;] ...
///     [PIN statement] ...
///     [OBS statement] ...
///     [DENSITY statement] ...
///     [PROPERTY propName propVal ;] ...
/// END macroName
/// ```
#[derive(Clone, Debug, Default)]
pub struct LefMacro {
    /// Name of the macro.
    pub name: String,
    /// Class of the macro.
    pub class: Option<LefMacroClass>,
    /// Disable shifting of masks.
    /// When set, shifting of macro pin mask assignments to other masks is not allowed.
    /// Used for technologies that use multi-mask patterning.
    pub fixed_mask: bool,
    /// Name of the corresponding cell layout in the GDS/OASIS file.
    /// Associated with an offset and orientation.
    pub foreign: Vec<LefMacroForegin>,
    /// Coordinate of the origin of the macro. Default is (0, 0).
    /// A placement of a cell in DEF is given by the location of the origin.
    pub origin: Point<f64>,
    /// Name of electrically equivalent macro.
    pub eeq: Option<String>,
    /// Width and height of the macro.
    pub size: Option<LefSize<f64>>,
    /// Symmetry of the macro. Tells how the macro can be mirrored and rotated.
    pub symmetry: LefSymmetry,
    /// SITE associated with the macro. Normal macros have only one associated site.
    pub sites: Vec<LefSiteReference>,
    /// Definitions of the electrical pins of the macro.
    pub pins: Vec<LefMacroPin>,
    /// Obstructions (blockages).
    pub obs: Vec<LefLayerGeometries>,
    /// Density specifications.
    pub density: Vec<LefMacroDensity>,
    /// Additional properties of the macro.
    pub properties: HashMap<String, LefPropertyValue>,
}

#[derive(Clone, Debug, Default, new)]
pub struct LefMacroForegin {
    pub cell_name: String,
    pub position: Point<f64>,
    pub orient: LefOrient,
}

/// PIN definition of a MACRO.
/// ```lef
/// [PIN pinName
///     [TAPERRULE ruleName ;]
///     [DIRECTION {INPUT | OUTPUT [TRISTATE] | INOUT | FEEDTHRU} ;]
///     [USE { SIGNAL | ANALOG | POWER | GROUND | CLOCK } ;]
///     [NETEXPR "netExprPropName defaultNetName" ;]
///     [SUPPLYSENSITIVITY powerPinName ;]
///     [GROUNDSENSITIVITY groundPinName ;]
///     [SHAPE {ABUTMENT | RING | FEEDTHRU} ;]
///     [MUSTJOIN pinName ;]
///     {PORT
///         [CLASS {NONE | CORE | BUMP} ;]
///         {layerGeometries} ...
///     END} ...
///     [PROPERTY propName propVal ;] ...
///     [ANTENNAPARTIALMETALAREA value [LAYER layerName] ;] ...
///     [ANTENNAPARTIALMETALSIDEAREA value [LAYER layerName] ;] ...
///     [ANTENNAPARTIALCUTAREA value [LAYER layerName] ;] ...
///     [ANTENNADIFFAREA value [LAYER layerName] ;] ...
///     [ANTENNAMODEL {OXIDE1 | OXIDE2 | OXIDE3 | OXIDE4} ;] ...
///     [ANTENNAGATEAREA value [LAYER layerName] ;] ...
///     [ANTENNAMAXAREACAR value LAYER layerName ;] ...
///     [ANTENNAMAXSIDEAREACAR value LAYER layerName ;] ...
///     [ANTENNAMAXCUTCAR value LAYER layerName ;] ...
/// END pinName]
/// ```
#[derive(Clone, Debug, Default)]
pub struct LefMacroPin {
    /// Name of the pin.
    pub name: String,
    /// Name of the NONDEFAULTRULE to be used when tapering wires to this pin.
    pub taper_rule: Option<String>,
    /// Signal direction.
    pub direction: Option<LefPinDirection>,
    /// Type of the signal for this pin.
    pub signal_use: Option<LefSignalUse>,
    /// Specify special connection requirements of the pin.
    pub shape_type: Option<LefPinShape>,
    /// Name of another pin that must be connected to this pin.
    pub must_join: Option<String>,
    /// Net expression for the pin.
    pub net_expr: Option<String>,
    /// Supply sensitivity for the pin.
    pub supply_sensitivity: Option<String>,
    /// Ground sensitivity for the pin.
    pub ground_sensitivity: Option<String>,
    /// Port
    pub port: Vec<LefLayerGeometries>,
}

/// Signal direction of a pin.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefPinDirection {
    /// INPUT
    Input,
    /// OUTPUT. Is TRISTATE when the boolean flag is set.
    Output(bool),
    /// INOUT: Both input and output.
    Inout,
    /// FEEDTHRU: Pin crosses the cell. Direct electrical connection.
    Feedthru,
}

impl FromStr for LefPinDirection {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "INPUT" => Ok(Self::Input),
            "OUTPUT" => Ok(Self::Output(false)),
            "INOUT" => Ok(Self::Inout),
            "FEEDTHRU" => Ok(Self::Feedthru),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefPinDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input => f.write_str("INPUT"),
            Self::Output(false) => f.write_str("OUTPUT"),
            Self::Output(true) => f.write_str("OUTPUT TRISTATE"),
            Self::Inout => f.write_str("INOUT"),
            Self::Feedthru => f.write_str("FEEDTHRU"),
        }
    }
}

/// Geometrical shapes on a named layer as used in MACRO PIN and OBS definitions.
/// ```lef
/// { LAYER layerName
///   [EXCEPTPGNET]
///   [SPACING minSpacing | DESIGNRULEWIDTH value] ;
///   [WIDTH width ;]
///   { PATH pt ... ;
///   | PATH ITERATE pt ... stepPattern ;
///   | RECT pt pt ;
///   | RECT ITERATE pt pt stepPattern ;
///   | POLYGON pt pt pt pt ... ;
///   | POLYGON ITERATE pt pt pt pt ... stepPattern ;
///   } ...
///   | VIA pt viaName ;
///   | VIA ITERATE pt viaName stepPattern ;
/// } ...
/// ```
#[derive(Clone, Debug, Default)]
pub struct LefLayerGeometries {
    /// Name of the layer.
    pub layer_name: String,
    /// Obstruction blocks signal routing but not power or ground routing.
    pub except_pg_net: bool,
    /// CLASS (NONE, CORE, BUMP) for pin geometries.
    pub class: Option<LefGeometryClass>,
    /// Either minimal allowed spacing or an effective width.
    pub spacing_or_designrule_width: Option<LefSpacingOrDesignRuleWidth>,
    /// Specify the width to be used for PATH. If not specified the default with for this layer is used.
    pub width: Option<f64>,
    /// Geometrical shapes (PATH, RECT, POLYGON). Together with a repetition pattern.
    pub geometries: Vec<LefGeometry>,
    /// Specify vias to be placed with their locations.
    pub vias: Vec<LefViaUse>,
}

/// ```lef
/// [CLASS
///    { COVER [BUMP]
///    | RING
///    | BLOCK [BLACKBOX | SOFT]
///    | PAD [INPUT | OUTPUT |INOUT | POWER | SPACER | AREAIO]
///    | CORE [FEEDTHRU | TIEHIGH | TIELOW | SPACER | ANTENNACELL | WELLTAP]
///    | ENDCAP {PRE | POST | TOPLEFT | TOPRIGHT | BOTTOMLEFT | BOTTOMRIGHT}
///    }
/// ;]
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefMacroClass {
    /// Macro with fixed position.
    /// Commonly used for power routing. COVER does not contain active devices.
    /// A COVER class can have the sub-class BUMP. Typically BUMP cells have
    /// geometries only on the topmost 'bump' layer.
    COVER(bool),
    /// Big macro with internal power mesh.
    RING,
    /// Predefined macro.
    BLOCK(Option<LefMacroClassBlockType>),
    /// I/O pad.
    PAD(Option<LefMacroClassPadType>),
    /// Standard-cell macro used inside the core area.
    CORE(Option<LefMacroClassCoreType>),
    /// Start or end of core rows. Typically used to connect to the power grid.
    ENDCAP(Option<LefMacroClassEndcapType>),
}

impl FromStr for LefMacroClass {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = input.split_whitespace().collect();
        match parts.len() {
            0 => Err("empty string".into()),
            1 => {
                match parts[0] {
                    "COVER" => Ok(Self::COVER(true)),
                    "RING" => Ok(Self::RING),
                    "BLOCK" => Ok(Self::BLOCK(Default::default())),
                    "PAD" => Ok(Self::PAD(Default::default())),
                    "CORE" => Ok(Self::CORE(Default::default())),
                    "ENDCAP" => Ok(Self::ENDCAP(Default::default())),
                    _ => Err(format!("unimplment: {}", input)),
                }
            }
            2 => {
                match parts[0] {
                    "COVER" => Ok(Self::COVER(false)),
                    "RING" => Err(format!("RING not support sub class '{}'", parts[1])),
                    "BLOCK" => match LefMacroClassBlockType::from_str(parts[1]) {
                        Ok(v) => Ok(Self::BLOCK(Some(v))),
                        Err(_) => Err(format!("invalid macro class block type '{}'", parts[1])),
                    }
                    "PAD" => match LefMacroClassPadType::from_str(parts[1]) {
                        Ok(v) => Ok(Self::PAD(Some(v))),
                        Err(_) => Err(format!("invalid macro class pad type '{}'", parts[1])),
                    }
                    "CORE" => match LefMacroClassCoreType::from_str(parts[1]) {
                        Ok(v) => Ok(Self::CORE(Some(v))),
                        Err(_) => Err(format!("invalid macro class core type '{}'", parts[1])),
                    }
                    "ENDCAP" => match LefMacroClassEndcapType::from_str(parts[1]) {
                        Ok(v) => Ok(Self::ENDCAP(Some(v))),
                        Err(_) => Err(format!("invalid macro class endcap type '{}'", parts[1])),
                    }
                    _ => Err(format!("unimplment: {}", input)),
                }
            }
            _ => Err(format!("bad format: {}", input)),
        }
    }
}

impl fmt::Display for LefMacroClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::COVER(bump) => {
                f.write_str("COVER")?;
                if *bump {
                    f.write_str("BUMP")?;
                }
            }
            Self::RING => f.write_str("RING")?,
            Self::BLOCK(sub_class) => {
                f.write_str("BLOCK")?;
                if let Some(sub_class) = sub_class {
                    sub_class.fmt(f)?;
                }
            }
            Self::PAD(sub_class) => {
                f.write_str("PAD")?;
                if let Some(sub_class) = sub_class {
                    sub_class.fmt(f)?;
                }
            }
            Self::CORE(sub_class) => {
                f.write_str("CORE")?;
                if let Some(sub_class) = sub_class {
                    sub_class.fmt(f)?;
                }
            }
            Self::ENDCAP(sub_class) => {
                f.write_str("ENDCAP")?;
                if let Some(sub_class) = sub_class {
                    sub_class.fmt(f)?;
                }
            }
        }
        Ok(())
    }
}


/// Subclass of the BLOCK macro class.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefMacroClassBlockType {
    /// A block which may only contain a SIZE statements for size estimation.
    /// A blackbox block is missing the implementation of the sub-block.
    BLACKBOX,
    /// A cell with partial implementation of the sub-block.
    SOFT,
}

impl FromStr for LefMacroClassBlockType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "BLACKBOX" => Ok(Self::BLACKBOX),
            "SOFT" => Ok(Self::SOFT),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefMacroClassBlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BLACKBOX => f.write_str("BLACKBOX"),
            Self::SOFT => f.write_str("SOFT"),
        }
    }
}

/// Subclass of the PAD macro class.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefMacroClassPadType {
    /// Input pad.
    INPUT,
    /// Output pad.
    OUTPUT,
    /// Inout pad.
    INOUT,
    /// Power pad.
    POWER,
    /// Spacer in the pad ring.
    SPACER,
    /// Area for I/O drivers with out connection to a bump. They need routing to a
    /// CLASS COVER BUMP macro for proper connection with the IC package.
    AREAIO,
}

impl FromStr for LefMacroClassPadType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "INPUT" => Ok(Self::INPUT),
            "OUTPUT" => Ok(Self::OUTPUT),
            "INOUT" => Ok(Self::INOUT),
            "POWER" => Ok(Self::POWER),
            "SPACER" => Ok(Self::SPACER),
            "AREAIO" => Ok(Self::AREAIO),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefMacroClassPadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::INPUT => f.write_str("INPUT"),
            Self::OUTPUT => f.write_str("OUTPUT"),
            Self::INOUT => f.write_str("INOUT"),
            Self::POWER => f.write_str("POWER"),
            Self::SPACER => f.write_str("SPACER"),
            Self::AREAIO => f.write_str("AREAIO"),
        }
    }
}

/// Subclass of the CORE macro class.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefMacroClassCoreType {
    /// Connect to another cell.
    FEEDTHRU,
    /// Logical one.
    TIEHIGH,
    /// Logical zero.
    TIELOW,
    /// Spacer/fill cell.
    SPACER,
    /// Antenna diode.
    ANTENNACELL,
    /// Well-tap cell.
    WELLTAP,
}

impl FromStr for LefMacroClassCoreType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "FEEDTHRU" => Ok(Self::FEEDTHRU),
            "TIEHIGH" => Ok(Self::TIEHIGH),
            "TIELOW" => Ok(Self::TIELOW),
            "SPACER" => Ok(Self::SPACER),
            "ANTENNACELL" => Ok(Self::ANTENNACELL),
            "WELLTAP" => Ok(Self::WELLTAP),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefMacroClassCoreType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FEEDTHRU => f.write_str("FEEDTHRU"),
            Self::TIEHIGH => f.write_str("TIEHIGH"),
            Self::TIELOW => f.write_str("TIELOW"),
            Self::SPACER => f.write_str("SPACER"),
            Self::ANTENNACELL => f.write_str("ANTENNACELL"),
            Self::WELLTAP => f.write_str("WELLTAP"),
        }
    }
}

/// Subclass of the ENDCAP macro class.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefMacroClassEndcapType {
    /// Start of the row (left).
    PRE,
    /// End of the row (right)
    POST,
    /// I/O corner cell on top left.
    TOPLEFT,
    /// I/O corner cell on top right.
    TOPRIGHT,
    /// I/O corner cell on bottom left.
    BOTTOMLEFT,
    /// I/O corner cell on bottom right.
    BOTTOMRIGHT,
}

impl FromStr for LefMacroClassEndcapType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "PRE" => Ok(Self::PRE),
            "POST" => Ok(Self::POST),
            "TOPLEFT" => Ok(Self::TOPLEFT),
            "TOPRIGHT" => Ok(Self::TOPRIGHT),
            "BOTTOMLEFT" => Ok(Self::BOTTOMLEFT),
            "BOTTOMRIGHT" => Ok(Self::BOTTOMRIGHT),
            _ => Err(()),
        }
    }
}

impl  fmt::Display for LefMacroClassEndcapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PRE => f.write_str("PRE"),
            Self::POST => f.write_str("POST"),
            Self::TOPLEFT => f.write_str("TOPLEFT"),
            Self::TOPRIGHT => f.write_str("TOPRIGHT"),
            Self::BOTTOMLEFT => f.write_str("BOTTOMLEFT"),
            Self::BOTTOMRIGHT => f.write_str("BOTTOMRIGHT"),
        }
    }
}

/// Type of the signal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LefSignalUse {
    /// Data signal.
    Signal,
    /// Analog signal.
    Analog,
    /// Power supply.
    Power,
    /// Ground.
    Ground,
    /// Clock signal.
    Clock,
}

impl FromStr for LefSignalUse {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "SIGNAL" => Ok(Self::Signal),
            "ANALOG" => Ok(Self::Analog),
            "POWER" => Ok(Self::Power),
            "GROUND" => Ok(Self::Ground),
            "CLOCK" => Ok(Self::Clock),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefSignalUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signal => f.write_str("SIGNAL"),
            Self::Analog => f.write_str("ANALOG"),
            Self::Power => f.write_str("POWER"),
            Self::Ground => f.write_str("GROUND"),
            Self::Clock => f.write_str("CLOCK"),
        }
    }
}


/// Type of the pin shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LefPinShape {
    ///
    Abutment,
    ///
    Ring,
    ///
    Feedthru,
}

impl FromStr for LefPinShape {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ABUTMENT" => Ok(Self::Abutment),
            "RING" => Ok(Self::Ring),
            "FEEDTHRU" => Ok(Self::Feedthru),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefPinShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abutment => f.write_str("ABUTMENT"),
            Self::Ring => f.write_str("RING"),
            Self::Feedthru => f.write_str("FEEDTHRU"),
        }
    }
}

/// Holds either the value of the SPACING argument or DESIGNRULEWIDTH argument of a geometrical
/// layer as used in the LAYER definition in PIN or OBS.
#[derive(Clone, Debug)]
pub enum LefSpacingOrDesignRuleWidth {
    /// Minimal allowed spacing between this shape and other shapes.
    MinSpacing(f64),
    /// Effective design rule width.
    DesignRuleWidth(f64),
}

/// Either a path, rectangle or polygon.
#[derive(Clone, Debug)]
pub enum LefGeometry {
    Path(PointList<f64>),
    PathIter((PointList<f64>, LefGeometryIter)),
    Rect(Rect<f64>),
    RectIter((Rect<f64>, LefGeometryIter)),
    Polygon(PointList<f64>),
    PolygonIter((PointList<f64>, LefGeometryIter)),
}

#[derive(Clone, Debug, new)]
pub struct LefViaUse {
    pub position: Point<f64>,
    pub via_name: String,
    pub iter: Option<LefGeometryIter>
}

#[derive(Clone, Debug, new)]
pub struct LefGeometryIter {
    pub num_x: usize,
    pub num_y: usize,
    pub space_x: f64,
    pub space_y: f64,
}