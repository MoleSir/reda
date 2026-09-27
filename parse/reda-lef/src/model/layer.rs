use std::{collections::HashMap, fmt, str::FromStr};
use super::LefPropertyValue;

/// MINSTEP rule for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefMinstepRule {
    pub min_step: f64,
    pub min_type: Option<LefMinstepType>,
    pub length_sum: Option<f64>,
    pub max_edges: Option<u32>,
    pub min_adj_length: Option<f64>,
    pub min_bet_length: Option<f64>,
    pub x_same_corners: bool,
}

#[derive(Clone, Debug)]
pub enum LefMinstepType {
    InsidCorner,
    OutsidCorner,
    Step,
}

impl FromStr for LefMinstepType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "INSIDECORNER" => Ok(Self::InsidCorner),
            "OUTSIDECORNER" => Ok(Self::OutsidCorner),
            "STEP" => Ok(Self::Step),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefMinstepType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsidCorner => f.write_str("INSIDECORNER"),
            Self::OutsidCorner => f.write_str("OUTSIDECORNER"),
            Self::Step => f.write_str("STEP"),
        }
    }
}

/// MINIMUMCUT rule for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefMinimumCutRule {
    pub num_cuts: u32,
    pub min_width: f64,
    pub within: Option<f64>,
    pub connection: Option<LefMinimumCutConnection>,
    pub length_within: Option<(f64, f64)>,
}

#[derive(Clone, Debug)]
pub enum LefMinimumCutConnection {
    FromAbove,
    FromBelow,
}

impl FromStr for LefMinimumCutConnection {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "FROMABOVE" => Ok(Self::FromAbove),
            "FROMBELOW" => Ok(Self::FromBelow),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefMinimumCutConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromAbove => f.write_str("FROMABOVE"),
            Self::FromBelow => f.write_str("FROMBELOW"),
        }
    }
}

/// PROTRUSIONWIDTH rule for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefProtrusionWidth {
    pub width1: f64,
    pub width2: f64,
    pub length: f64,
}

/// AC current density for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefACCurrentDensity {
    pub peak: f64,
    pub frequency: Vec<(f64, f64)>,
    pub width_table: Vec<(f64, f64)>,
    pub cut_area_table: Vec<(f64, f64)>,
}

/// Antenna model rule for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefAntennaModelRule {
    pub oxide: String,
    pub area_ratio: Option<f64>,
    pub diff_area_ratio: Option<f64>,
    pub cum_area_ratio: Option<f64>,
    pub cum_diff_area_ratio: Option<f64>,
    pub area_factor: Option<f64>,
    pub side_area_ratio: Option<f64>,
    pub diff_side_area_ratio: Option<f64>,
    pub cum_side_area_ratio: Option<f64>,
    pub cum_diff_side_area_ratio: Option<f64>,
    pub side_area_factor: Option<f64>,
}

/// Cut layer spacing table.
#[derive(Clone, Debug, Default)]
pub struct LefCutSpacingTable {
    pub cut_withins: Vec<f64>,
    pub ortho_spacings: Vec<f64>,
}

/// ARRAYSPACING for a cut layer.
#[derive(Clone, Debug, Default)]
pub struct LefArraySpacing {
    pub long_array: bool,
    pub via_width: f64,
    pub cut_spacing: f64,
    pub array_cuts: Vec<(i32, f64)>,
}

#[derive(Clone, Debug)]
pub enum LefLayer {
    /// MASTERSLICE (poly) layer.
    Masterslice(LefMastersliceLayer),
    /// CUT layer.
    Cut(LefCutLayer),
    /// ROUTING layer.
    Routing(LefRoutingLayer),
    /// IMPLANT
    Implant(LefImplantLayer),
}

impl LefLayer {
    /// Get the name of the layer.
    pub fn name(&self) -> &String {
        match self {
            LefLayer::Masterslice(l) => &l.name,
            LefLayer::Cut(l) => &l.name,
            LefLayer::Routing(l) => &l.name,
            LefLayer::Implant(l) => &l.name,
        }
    }
}

/// Design rules for a MASTERSLICE or OVERLAP layer.
/// Master slice layers are usually polysilicon layers and are typically used when a MACRO has
/// pins on the poly layer.
#[derive(Clone, Debug, Default)]
pub struct LefMastersliceLayer {
    /// Name of the masterslice layer.
    pub name: String,
    /// Number of masks used for double- or triple-patterning.
    pub mask_num: Option<u32>,

    /// Custom properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

/// Design rules for a CUT (via) layer.
#[derive(Clone, Debug, Default)]
pub struct LefCutLayer {
    /// Name of the cut layer.
    pub name: String,
    /// Number of masks used for double- or triple-patterning.
    pub mask_num: Option<u32>,
    /// Minimum spacing rules between cuts of same or different nets.
    pub spacing: Vec<LefCutSpacingRule>,
    /// Spacing table to be used on this cut layer.
    pub spacing_table: Option<LefCutSpacingTable>,
    /// Array spacing for this cut layer.
    pub array_spacing: Option<LefArraySpacing>,
    /// Minimum width of a cut in microns.
    /// Usually this is the only allowed size of a cut.
    pub width: Option<f64>,
    /// Enclosure rules that must be met.
    pub enclosure: Vec<LefEnclosureRule>,
    /// Preferred enclosure rules that can be used to improve yield but must not necessarily
    /// be met.
    pub prefer_enclosure: Vec<LefEnclosureRule>,
    /// Resistance per cut.
    pub resistance: Option<f64>,
    /// Custom properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

#[derive(Clone, Debug, Default)]
pub struct LefImplantLayer {
    /// Name of the layer.
    pub name: String,
    /// Minimum width of a cut in microns.
    pub width: Option<f64>,
    /// Custom properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

/// Design rules for a routing layer.
#[derive(Clone, Debug)]
pub struct LefRoutingLayer {
    /// Name of the routing layer.
    pub name: String,
    /// Number of masks used for double- or triple-patterning.
    pub mask_num: Option<u32>,
    /// Preferred routing direction.
    pub direction: LefRoutingDirection,
    /// Routing pitch in x and y direction in microns.
    pub pitch: (f64, f64),
    /// Routing pitch for diagonal directions in microns.
    pub diag_pitch: Option<(f64, f64)>,
    /// Default wire width in microns.
    pub width: f64,
    /// Offset
    pub offset: Option<(f64, f64)>,
    /// Default width for diagonal wires in microns.
    pub diag_width: Option<f64>,
    /// Default spacing for diagonal wires in microns.
    pub diag_spacing: Option<f64>,
    /// Minimum edge length for diagonal wires in microns.
    pub diag_min_edge_length: Option<f64>,
    /// Minimum area for shapes on this layer.
    pub min_area: Option<f64>,
    /// Minimal rectangles that must fit in each shape on this layer.
    /// At least one needs to fit for each shape.
    /// Tuples of `(minimal width, minimal length)`.
    pub min_size: Vec<(f64, f64)>,
    /// Minimal edge length for shapes (MINSTEP rules).
    pub min_step: Vec<LefMinstepRule>,
    /// Spacing rules.
    pub spacing: Vec<LefSpacingRules>,
    /// Spacing tables for spacing between wires.
    pub spacing_table: Option<LefSpacingTable>,
    /// Length of extension of a wire over a via. The extension must be at least half of the
    /// wire width.
    pub wire_extension: Option<f64>,
    /// Minimal number of cuts of a via depending on the width of the wire.
    pub minimum_cut: Vec<LefMinimumCutRule>,
    /// Maximum wire width in microns.
    pub max_width: Option<f64>,
    /// Minimum wire width in microns.
    pub min_width: Option<f64>,
    /// Minimum area of holes in metal shapes.
    /// `(min area [um^2], width [um])` tuples.
    /// If a width is specified the rule only applies if at least one of the wires around the hole
    /// has a larger width.
    pub min_enclosed_area: Vec<(f64, Option<f64>)>,
    /// Width of a protrusion.
    pub protrusion_width: Option<LefProtrusionWidth>,
    /// Sheet resistance `[Ohm/square]`.
    pub resistance: Option<f64>,
    /// Specify wire-to-ground capacitance per square unit in `[pF/um^2]`.
    pub capacitance: Option<f64>,
    /// Distance from top of ground plane to bottom of this interconnect layer.
    pub height: Option<f64>,
    /// Thickness of the layer in microns.
    pub thickness: Option<f64>,
    /// Amount of loss in width of wires caused by the etching process.
    pub shrinkage: Option<f64>,
    /// Account for increase in capacitance caused by close wires.
    /// Default is 1.
    pub cap_multiplier: u32,
    /// `[pF/um]`.
    pub edge_capacitance: Option<f64>,
    /// Maximum allowed metal density in percent.
    pub minimum_density: Option<f64>,
    /// Minimum allowed metal density in percent.
    pub maximum_density: Option<f64>,
    /// Length and width of the density check window.
    pub density_check_window: Option<(f64, f64)>,
    /// Stepping distance for metal density checks.
    pub density_check_step: Option<f64>,
    /// Spacing between metal fills and active geometries.
    pub fill_active_spacing: Option<f64>,
    /// AC current density information.
    pub ac_current_density: Option<LefACCurrentDensity>,
    /// Average DC current density information.
    /// Stored as a `(wire width, current density)` table.
    /// If only a default value is specified for all widths
    /// it is stored as a single entry for wire width `0`: `(0, default_current_density)`.
    /// Unit: `[mA/um]`
    pub dc_current_density: Vec<(f64, f64)>,
    /// Minimum area of the antenna diode.
    pub antenna_area: Option<f64>,
    /// Minimum length of the antenna diode.
    pub antenna_length: Option<f64>,
    /// Antenna model rules for this layer.
    pub antenna_models: Vec<LefAntennaModelRule>,

    /// Custom properties.
    pub properties: HashMap<String, LefPropertyValue>,
}

/// ENCLOSURE rules for a CUT (via) layer.
#[derive(Clone, Debug)]
pub struct LefEnclosureRule {
    /// Rule applies for the routing layer above.
    pub above: bool,
    /// Rule applies for the routing layer below.
    pub below: bool,
    /// Adjacent routing layers must overhang on the two opposing sides (in x-direction (?)).
    pub overhang1: f64,
    /// Adjacent routing layers must overhang on the two opposing sides (in y-direction (?)).
    pub overhang2: f64,
    /// Rule only applies if the width of the adjacent shape of the routing layer is greater or equal
    /// than `min_width`.
    /// Default is 0.
    pub min_width: f64,
    /// TODO
    /// Don't use the WIDTH rule when another via is present to the current via within this distance.
    pub except_extracut_within: f64,
    /// Rule only applies if the total length of the longest overhangs is greater or equal
    /// to `min_length`.
    /// The overhang length is measured from the via cut center.
    /// Default is 0.
    pub min_length: f64,
}

impl Default for LefEnclosureRule {
    fn default() -> Self {
        Self {
            above: true,
            below: true,
            overhang1: 0.0,
            overhang2: 0.0,
            min_width: 0.0,
            except_extracut_within: 0.0,
            min_length: 0.0,
        }
    }
}

/// SPACING rules for a CUT (via) layer.
#[derive(Clone, Debug)]
pub struct LefCutSpacingRule {
    /// Spacing between cuts.
    pub spacing: f64,
    /// Measure the spacing from center of the cut to the center of another cut
    /// instead of from edge to edge.
    /// This is enabled by default.
    pub center_to_center: bool,
    /// Tell if this spacing rule applies for same-net cuts.
    pub same_net: bool,
}

impl Default for LefCutSpacingRule {
    fn default() -> Self {
        Self {
            spacing: 0.0,
            center_to_center: true,
            same_net: false,
        }
    }
}

impl Default for LefRoutingLayer {
    /// Custom implementation of the `Default` trait for `RoutingLayer.
    fn default() -> Self {
        Self {
            name: Default::default(),
            mask_num: Default::default(),
            direction: LefRoutingDirection::Vertical,
            pitch: (0.0, 0.0),
            diag_pitch: Default::default(),
            width: 0.0,
            offset: Default::default(),
            diag_width: Default::default(),
            diag_spacing: Default::default(),
            diag_min_edge_length: Default::default(),
            min_area: Default::default(),
            min_size: Default::default(),
            min_step: Default::default(),
            spacing: Default::default(),
            spacing_table: Default::default(),
            wire_extension: Default::default(),
            minimum_cut: Default::default(),
            max_width: Default::default(),
            min_width: Default::default(),
            min_enclosed_area: Default::default(),
            protrusion_width: Default::default(),
            resistance: Default::default(),
            capacitance: Default::default(),
            height: Default::default(),
            thickness: Default::default(),
            shrinkage: Default::default(),
            // Take multiplicative identity as default for the capacitance multiplier.
            cap_multiplier: 1,
            edge_capacitance: Default::default(),
            minimum_density: Default::default(),
            maximum_density: Default::default(),
            density_check_window: Default::default(),
            density_check_step: Default::default(),
            fill_active_spacing: Default::default(),
            ac_current_density: Default::default(),
            dc_current_density: Default::default(),
            antenna_area: Default::default(),
            antenna_length: Default::default(),
            antenna_models: Default::default(),
            properties: Default::default(),
        }
    }
}


/// Preferred routing direction on a routing layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LefRoutingDirection {
    /// Vertical routing direction.
    Vertical,
    /// Horizontal routing direction.
    Horizontal,
    /// 45 degree routing direction.
    Diag45,
    /// 135 degree routing direction.
    Diag135,
}

impl FromStr for LefRoutingDirection {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "VERTICAL" => Ok(Self::Vertical),
            "HORIZONTAL" => Ok(Self::Horizontal),
            "DIAG45" => Ok(Self::Diag45),
            "DIAG135" => Ok(Self::Diag135),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LefRoutingDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vertical => f.write_str("VERTICAL"),
            Self::Horizontal => f.write_str("HORIZONTAL"),
            Self::Diag45 => f.write_str("DIAG45"),
            Self::Diag135 => f.write_str("DIAG135"),
        }
    }
}

/// Spacing rules for a routing layer.
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct LefSpacingRules {
    pub min_spacing: f64,
    pub spacing_type: Option<LefSpacingType>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum LefSpacingType {
    Range {
        min_width: f64,
        max_width: f64,
        spacing_range_type: Option<LefSpacingRangeType>,
    },
    EndOfLine {
        eol_width: f64,
        eol_widthing: f64,
    },
    /// Rule applies only for two shapes of the same net.
    SameNet {
        /// PGONLY, same-net rule applies for power and ground nets only.
        power_ground_only: bool,
    },
    NotchLength {
        min_notch_length: f64,
    },
    EndOfNotchWidth {
        end_of_notch_width: f64,
        min_notch_spacing: f64,
        min_notch_length: f64,
    },
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum LefSpacingRangeType {
    UseLengthThreshold,
    Influence { influence_length: f64 },
}

/// SPACINGTABLE, spacing rules for a routing layer.
#[derive(Clone, Debug, Default)]
pub struct LefSpacingTable {
    /// Indices of the table columns.
    pub parallel_run_lengths: Vec<f64>,
    /// Indices of the table rows.
    pub widths: Vec<f64>,
    /// Table values
    pub spacings: Vec<Vec<f64>>,
}