use std::collections::HashMap;
use derive_new::new;
use reda_geometry::shape::Rect;
use super::{LefPropertyValue, LefXy};

/// ```lef
/// VIA viaName [DEFAULT]
///     { VIARULE viaRuleName ;
///         CUTSIZE xSize ySize ;
///         LAYERS botMetalLayer cutLayer topMetalLayer ;
///         CUTSPACING xCutSpacing yCutSpacing ;
///         ENCLOSURE xBotEnc yBotEnc xTopEnc yTopEnc ;
///         [ROWCOL numCutRows numCutCols ;]
///         [ORIGIN xOffset yOffset ;]
///         [OFFSET xBotOffset yBotOffset xTopOffset yTopOffset ;]
///         [PATTERN cutPattern ;]
///     }
///     | {[RESISTANCE resistValue ;]
///        {LAYER layerName ;
///         { RECT pt pt ;
///         | POLYGON pt pt pt ...;} ...
///        } ...
///       }
///     [PROPERTY propName propVal ;] ...
/// END viaName
/// ```
#[derive(Clone, Debug)]
pub enum LefVia {
    Generated(LefGeneratedVia),
    Fix(LefFixVia),
}

#[derive(Clone, Debug, Default)]
pub struct LefGeneratedVia {
    /// Default via to be used for routing between the adjacent layers.
    pub is_default: bool,
    /// Via Rule name
    pub viarule_name: String,
    /// CUTSIZE xSize ySize ;
    pub cut_size: LefXy<f64>, 
    /// LAYERS botMetalLayer cutLayer topMetalLayer ;
    pub layers: LefViaLayers,
    /// CUTSPACING xCutSpacing yCutSpacing ;
    pub cut_spacing: LefXy<f64>, 
    /// ENCLOSURE xBotEnc yBotEnc xTopEnc yTopEnc ;
    pub enclosure: LefViaBotTop,
    /// [ROWCOL numCutRows numCutCols ;]
    pub rowcol: LefViaRowCol,
    /// [ORIGIN xOffset yOffset ;]
    pub origin: LefXy<f64>, 
    /// [OFFSET xBotOffset yBotOffset xTopOffset yTopOffset ;]
    pub offset: LefViaBotTop,
    /// [PATTERN cutPattern ;]
    pub pattern: Option<String>,
    /// Additional properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

#[derive(Clone, Debug, Default)]
pub struct LefFixVia {
    /// Default via to be used for routing between the adjacent layers.
    pub is_default: bool,
    /// Electrical resistance of the via.
    pub resistance: Option<f64>,
    /// Layers and shapes of the via geometry.
    pub geometry: HashMap<String, Vec<LefViaShape>>,
    /// Additional properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

/// VIARULE viaRuleName GENERATE [DEFAULT]
///     LAYER routingLayerName ;
///         ENCLOSURE overhang1 overhang2 ;
///         [WIDTH minWidth TO maxWidth ;]
///     LAYER routingLayerName ;
///         ENCLOSURE overhang1 overhang2 ;
///         [WIDTH minWidth TO maxWidth ;]
///     LAYER cutLayerName ;
///         RECT pt pt ;
///         SPACING xSpacing BY ySpacing ;
///         [RESISTANCE resistancePerCut ;]
/// END viaRuleName
#[derive(Clone, Debug, Default)]
pub struct LefViaRule {
    pub is_default: bool,
    /// Via generate rule which was used to generate this via.
    pub rule_name: String,
    /// Bottom, cut and top layer.
    pub layers: LefViaLayers,
    /// ENCLOSURE overhang1 overhang2 ;
    pub bottom_enclosure: LefViaEnclosure,
    pub top_enclosure: LefViaEnclosure,
    /// [WIDTH minWidth TO maxWidth ;]
    pub bottom_width: Option<LefViaRuleWidth>,
    pub top_width: Option<LefViaRuleWidth>,
    /// cut rectangle
    pub rect: Rect<f64>,
    /// center-to-center spacing in the x and y dimensions to create an array of contact cuts.
    pub spacing: LefViaSpacing,
    /// [RESISTANCE resistancePerCut ;]
    pub resistance: Option<f64>,
}

/// Either a rectangle or a polygon.
#[derive(Clone, Debug)]
pub enum LefViaShape {
    /// Axis-aligned rectangle.
    Rect(Rect<f64>),
    /// Polygon.
    Polygon(Vec<(f64, f64)>),
}

#[derive(Clone, Debug, Default, new)]
pub struct LefViaLayers {
    pub bottom: String,
    pub cut: String,
    pub top: String,
}

#[derive(Clone, Debug, Default)]
pub struct LefViaSpacing {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Default, new)]
pub struct LefViaBotTop {
    pub x_bottom: f64,
    pub y_bottom: f64,
    pub x_top: f64,
    pub y_top: f64,
}

#[derive(Clone, Debug, Default, new)]
pub struct LefViaRuleWidth {
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug, Default, new)]
pub struct LefViaEnclosure {
    pub overhang1: f64,
    pub overhang2: f64,
}

#[derive(Clone, Debug, new)]
pub struct LefViaRowCol {
    pub num_cut_rows: usize,
    pub num_cut_cols: usize,
}

impl Default for LefViaRowCol {
    fn default() -> Self {
        Self::new(1, 1)
    }
}