mod base;

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use base::{DefOrient, DefPinDirection, DefPropertyType, DefPropertyValue};
use reda_geometry::shape::{Point, PointList, Polyline, Rect};

use crate::read::DefReadError;

/// Integer type used for mask numbers.
pub type MaskNum = u8;
/// Integer coordinate type.
pub type Coord = f64;

/// Representation of a design as defined in DEF.
#[derive(Clone, Debug)]
pub struct Def {
    /// Version of the DEF syntax.
    pub version: Option<String>,
    /// Characters that are used to mark bus bit indices. Default are `[` and `]`.
    pub busbitchars: (char, char),
    /// Character used as path separator. Default is `/`.
    pub dividerchar: char,
    /// Name of the design.
    pub design_name: Option<String>,
    // DESIGN
    /// Name of the technology.
    pub technology: Option<String>,
    /// Distance units per micron.
    /// Values supported by LEF are: 100, 200, 1000, 2000, 10000, 20000
    /// The database unit value in LEF must be greater or equal to the one in DEF to avoid round-off errors.
    pub units: u32,
    /// Arbitrary text.
    pub history: Vec<String>,
    /// Definitions of custom properties used in the design.
    pub property_definitions: HashMap<String, DefPropertyDefinition>,
    /// Shape of the die.
    pub die_area: Option<PointList<Coord>>,
    /// Rows stored by their name.
    pub rows: HashMap<String, DefRow>,
    /// Routing grid for standard-cell based designs.
    pub tracks: Vec<DefTracks>,
    /// Geometry definitions of the vias used in the design.
    pub vias: HashMap<String, ViaDefinition>,
    /// Regions are named geometrical locations that can be used to place components.
    pub regions: HashMap<String, DefRegion>,
    /// Components of the design. This includes placed and unplaced cells and macros.
    pub components: Vec<DefComponent>,
    /// Definitions of external pins of the design. Associates external pin names with internal nets and
    /// pin geometries.
    pub pins: Vec<Pin>,
    /// Definitions of pin properties.
    /// Placement and routing blockages.
    pub blockages: Vec<DefBlockage>,
    /// Definitions of special nets.
    /// Special nets include nets like include power, clock, RF, and analog signals.
    /// They should not be touched by the signal router.
    pub special_nets: HashMap<String, ()>,
    /// Definitions of normal signal nets.
    pub nets: Vec<Net>,
    /// Scan-chain definitions.
    pub scan_chains: (),
    /// Groups of components.
    pub groups: HashMap<String, Vec<DefGroup>>,
    /// BEGINEXT blocks with non-standard extensions of the DEF format.
    pub extensions: (),
}

impl Default for Def {
    fn default() -> Self {
        Def {
            version: Some("5.8".into()),
            busbitchars: ('[', ']'),
            dividerchar: '/',
            design_name: None,
            technology: None,
            units: 0,
            history: Default::default(),
            property_definitions: Default::default(),
            die_area: Default::default(),
            rows: Default::default(),
            tracks: Default::default(),
            vias: Default::default(),
            regions: Default::default(),
            components: Default::default(),
            pins: Default::default(),
            blockages: Default::default(),
            special_nets: Default::default(),
            nets: Default::default(),
            scan_chains: Default::default(),
            groups: Default::default(),
            extensions: Default::default(),
        }
    }
}

/// Holds either the value of the SPACING argument or DESIGNRULEWIDTH argument of a geometrical
/// layer as used in the LAYER definition in PIN or OBS.
#[derive(Clone, Debug)]
pub enum DefSpacingOrDesignRuleWidth {
    /// Minimal allowed spacing between this shape and other shapes.
    MinSpacing(Coord),
    /// Effective design rule width.
    DesignRuleWidth(Coord),
}

/// Either a metal/via blockage or a component placement blockage..
#[derive(Clone, Debug)]
pub enum DefBlockage {
    /// Block component placement.
    PlacementBlockage(DefPlacementBlockage),
    /// Metal or via blockage.
    LayerBlockage(DefLayerBlockage),
}

/// Either a rectangle or a polygon.
#[derive(Clone, Debug)]
pub enum DefRectOrPolygon {
    /// Axis-aligned rectangle.
    Rect(Rect<Coord>),
    /// Polygon.
    Polygon(PointList<Coord>),
}

/// Define a region on a layer that is blocked from being used.
#[derive(Clone, Debug, Default)]
pub struct DefLayerBlockage {
    /// Layer of the blockage.
    pub layer: String,
    /// Block insertion of slots.
    pub slots: bool,
    /// Block insertion of metal fills.
    pub fills: bool,
    /// TBD
    pub pushdown: bool,
    /// Blockage does not concern power and ground nets, only signals.
    pub except_pg_net: bool,
    /// TBD
    pub component: Option<String>,
    /// TBD
    pub spacing: Option<Coord>,
    /// Either minimal allowed spacing or an effective width.
    pub spacing_or_designrule_width: Option<DefSpacingOrDesignRuleWidth>,
    /// Mask number.
    pub mask_num: Option<MaskNum>,
    /// Geometry of the blockage area.
    pub blockage_shapes: Vec<DefRectOrPolygon>,
}

/// Specify the type of a soft blockage.
#[derive(Clone, Debug)]
pub enum DefPlacementBlockageType {
    /// Initial placement shall not use the blocked area, but later stages can use it.
    Soft,
    /// Give a maximal usage density for the initial placement in this area.
    /// The density is given in percent and must have a value in the range `[0.0, ..., 100.0]`.
    Partial(f64),
}

/// Define a region where placement of components is not allowed.
#[derive(Clone, Debug, Default)]
pub struct DefPlacementBlockage {
    /// Type of the blockage (placement blockage or routing/metal/via blockage).
    pub blockage_type: Option<DefPlacementBlockageType>,
    /// Blockage was pushed down through hierarchy from a component on a higher level.
    pub pushdown: bool,
    /// Name of the associated component.
    pub component: Option<String>,
    /// Rectangles that cover the blocked region.
    pub rects: Vec<Rect<Coord>>,
}

/// Property definition used in DEF.
#[derive(Clone, Debug)]
pub struct DefPropertyDefinition {
    /// DEF object type associated with this property.
    pub object_type: DefPropertyObjectType,
    /// Data type of the property value.
    pub property_type: DefPropertyType,
    /// Optional min and max values.
    pub range: Option<(DefPropertyValue, DefPropertyValue)>,
    /// Default value of such a property.
    pub default_value: Option<DefPropertyValue>,
}

/// Type of parent object of the property.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefPropertyObjectType {
    /// Property belongs to a component.
    Component,
    /// Property belongs to a pin of a component.
    ComponentPin,
    /// Property belongs to the design.
    Design,
    /// Property belongs to a group.
    Group,
    /// Property belongs to a net.
    Net,
    /// Property belongs to a 'non-default-rule'.
    NonDefaultRule,
    /// Property belongs to a region.
    Region,
    /// Property belongs to a row.
    Row,
    /// Property belongs to a special net.
    SpecialNet,
}

impl FromStr for DefPropertyObjectType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "COMPONENT" => Ok(Self::Component),
            "COMPONENTPIN" => Ok(Self::ComponentPin),
            "DESIGN" => Ok(Self::Design),
            "GROUP" => Ok(Self::Group),
            "NET" => Ok(Self::Net),
            "NONDEFAULTRULE" => Ok(Self::NonDefaultRule),
            "REGION" => Ok(Self::Region),
            "ROW" => Ok(Self::Row),
            "SPECIALNET" => Ok(Self::SpecialNet),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DefPropertyObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component => f.write_str("COMPONENT"),
            Self::ComponentPin => f.write_str("COMPONENTPIN"),
            Self::Design => f.write_str("DESIGN"),
            Self::Group => f.write_str("GROUP"),
            Self::Net => f.write_str("NET"),
            Self::NonDefaultRule => f.write_str("NONDEFAULTRULE"),
            Self::Region => f.write_str("REGION"),
            Self::Row => f.write_str("ROW"),
            Self::SpecialNet => f.write_str("SPECIALNET"),
        }
    }
}

/// Instantiation of a component in DEF.
#[derive(Clone, Debug, Default)]
pub struct DefComponent {
    /// Name of the component instance.
    pub name: String,
    /// Name of the component template/model.
    pub model_name: String,
    /// Name of the electrically equivalent master.
    pub eeq_master: Option<String>,
    /// Tells where this component has been created.
    pub source: ComponentSource,
    /// Placement location of the component.
    /// `(location, orientation, is fixed)`
    pub position: Option<(Point<i32>, DefOrient, bool)>,
    /// Placement halo. Defines a placement blockage around the component.
    /// If `is_soft` is set, then the blockage does not need to be respected after the initial placement.
    /// `(is_soft, left, bottom, right, top)`.
    pub halo: Option<(bool, i32, i32, i32, i32)>,
    /// Routing halo. TODO.
    /// Structure is `(haloDist, minLayer, maxLayer)`.
    pub route_halo: Option<(i32, String, String)>,
    /// Weight of the component placement. Tells how costly a relocation of the component is.
    pub weight: u32,
    /// Name of the region where this component should be placed.
    pub region: Option<String>,
    /// Custom properties.
    pub properties: HashMap<String, DefPropertyValue>,
}

/// Source of a component.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ComponentSource {
    /// Component comes from the original netlist. This is the default value.
    Netlist,
    /// Physical component that connects only to power and ground nets.
    /// (Filler cells, well-taps, decoupling capacitors).
    Dist,
    /// Component generated by the user for some other reason.
    User,
    /// Component was inserted to meet timing constraints.
    Timing,
}

impl Default for ComponentSource {
    fn default() -> Self {
        Self::Netlist
    }
}

impl FromStr for ComponentSource {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "NETLIST" => Ok(Self::Netlist),
            "DIST" => Ok(Self::Dist),
            "USER" => Ok(Self::User),
            "TIMING" => Ok(Self::Timing),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ComponentSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Netlist => f.write_str("NETLIST"),
            Self::Dist => f.write_str("DIST"),
            Self::User => f.write_str("USER"),
            Self::Timing => f.write_str("TIMING"),
        }
    }
}

/// Source of a net.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NetSource {
    /// Physical component that connects only to power and ground nets.
    /// (Filler cells, well-taps, decoupling capacitors).
    Dist,
    /// Physical component that connects only to power and ground nets.
    /// (Filler cells, well-taps, decoupling capacitors).
    Netlist,
    /// Net belongs to a scan-chain.
    Test,
    /// Component was inserted to meet timing constraints.
    Timing,
    /// Component generated by the user for some other reason.
    User,
}

impl Default for NetSource {
    fn default() -> Self {
        Self::Netlist
    }
}

impl FromStr for NetSource {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "DIST" => Ok(Self::Dist),
            "NETLIST" => Ok(Self::Netlist),
            "TEST" => Ok(Self::Test),
            "TIMING" => Ok(Self::Timing),
            "USER" => Ok(Self::User),
            _ => Err(()),
        }
    }
}

impl fmt::Display for NetSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dist => f.write_str("DIST"),
            Self::Netlist => f.write_str("NETLIST"),
            Self::Test => f.write_str("TEST"),
            Self::Timing => f.write_str("TIMING"),
            Self::User => f.write_str("USER"),
        }
    }
}

/// Routing pattern of a net.
/// Default: `Steiner`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NetPattern {
    /// Minimize skews in timing delays for clock nets.
    Balanced,
    /// Minimize net length.
    Steiner,
    /// Minimize delays for global nets.
    Trunk,
    /// For ECL designs.
    WiredLogic,
}

impl Default for NetPattern {
    fn default() -> Self {
        Self::Steiner
    }
}

impl FromStr for NetPattern {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "BALANCED" => Ok(Self::Balanced),
            "STEINER" => Ok(Self::Steiner),
            "TRUNK" => Ok(Self::Trunk),
            "WIREDLOGIC" => Ok(Self::WiredLogic),
            _ => Err(()),
        }
    }
}

impl fmt::Display for NetPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Balanced => f.write_str("BALANCED"),
            Self::Steiner => f.write_str("STEINER"),
            Self::Trunk => f.write_str("TRUNK"),
            Self::WiredLogic => f.write_str("WIREDLOGIC"),
        }
    }
}

/// An internal pin or external pin of another component where a net can be attached.
#[derive(Clone, Debug)]
pub enum NetTerminal {
    /// Pin of a component instance.
    ComponentPin {
        // TODO: Use string interning here.
        /// Name of the component instance.
        component_name: String,
        /// Name of the pin.
        pin_name: String,
    },
    /// Name of an IO pin of the design.
    IoPin(String),
}

/// TBD
#[derive(Clone, Debug)]
pub struct Mustjoin {
    /// Name of the component.
    pub component_name: String,
    /// Name of a pin that belongs to the MUSTJOIN net.
    pub pin_name: String,
}

// /// A net either has a name or it is declared as MUSTJOIN.
// #[derive(Clone, Debug)]
// pub enum NameOrMustjoin {
//     /// Name of the net.
//     Name(String),
//     Mustjoin {
//         /// Name of the component.
//         component_name: String,
//         /// Name of a pin that belongs to the MUSTJOIN net.
//         pin_name: String,
//     }
// }

/// Wiring class. Tells whether a wiring can be changed by tools, manually or not at all.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WiringClass {
    /// Route cannot be changed at all.
    COVER,
    /// Route cannot be changed by automatic tools, only by interactive commands.
    FIXED,
    /// Route can be changed by automatic tool.
    /// A net that is routed must also specify the layer name.
    ROUTED,
    /// Last wide segment is not shielded.
    NOSHIELD,
}

impl FromStr for WiringClass {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "COVER" => Ok(Self::COVER),
            "FIXED" => Ok(Self::FIXED),
            "ROUTED" => Ok(Self::ROUTED),
            "NOSHIELD" => Ok(Self::NOSHIELD),
            _ => Err(()),
        }
    }
}

impl fmt::Display for WiringClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::COVER => f.write_str("COVER"),
            Self::FIXED => f.write_str("FIXED"),
            Self::ROUTED => f.write_str("ROUTED"),
            Self::NOSHIELD => f.write_str("NOSHIELD"),
        }
    }
}

/// Represent an element of regular (non-special) wiring.
#[derive(Clone, Debug)]
pub enum RegularWiringElement {
    /// A point in the euclidean plane.
    Point {
        /// x-coordinate
        x: Coord,
        /// y-coordinate
        y: Coord,
        /// Optional extension.
        ext_value: Option<Coord>,
    },
    /// Via.
    Via {
        /// Name of the via.
        via_name: String,
    },
    /// Mask number.
    MaskNum(u32),
    /// Via-mask number.
    ViaMaskNum(DefViaMaskNum),
    /// Virtual wiring element. A point in the euclidean plane.
    Virtual {
        /// x-coordinate
        x: Coord,
        /// y-coordinate
        y: Coord,
    },
    /// An axis aligned rectangle. TODO
    Rect(),
}

/// 'Vertex' in the routing path.
#[derive(Clone, Debug)]
pub enum RoutingPoint {
    /// A simple vertex of a path.
    Point {
        /// Location of the path vertex.
        point: Point<Coord>,
        /// Extension of the wire past the endpoint. Default is half the wire width.
        ext_value: Option<Coord>,
        /// Mask-number of the following path segment.
        mask_num: Option<MaskNum>,
    },
    /// A via, placed at the location of the last path vertex.
    Via {
        /// Name of the via to be used.
        via_name: String,
        /// Rotation of the via.
        orient: Option<DefOrient>,
        /// Mask-number of the via geometries.
        via_mask_num: Option<DefViaMaskNum>,
    },
    /// A rectangular shape with absolute coordinates.
    Rect {
        /// Rectangle. Absolute coordinates.
        /// DEF stores coordinates relative to the last path point, but here the coordinates are already resolved.
        rect: Rect<Coord>,
        /// Mask-number of the rectangle.
        mask_num: Option<MaskNum>,
    },
    /// A 'virtual' path vertex. The path segment leading to the virtual vertex is not materialized.
    /// This is equal to ending the path at the last point and starting a new path at the virtual vertex, hence
    /// allows to create discontinuous paths.
    Virtual(Point<Coord>),
}

/// Wiring statement which makes up the [`RegularWiring`].
#[derive(Clone, Debug)]
pub struct RegularWiringStatement {
    /// Layer where the wire starts.
    pub start_layer_name: String,
    /// Name of the taper rule to be used.
    pub taper_rule: Option<String>,
    /// Routing style number.
    pub style_num: u32,
    // TODO: Check this.
    /// Routing path.
    pub routing_points: Vec<RoutingPoint>,
}

/// Representation of regular wiring.
#[derive(Clone, Debug)]
pub struct RegularWiring {
    /// Wiring class.
    pub class: WiringClass,
    /// The wiring segments which make this wiring.
    pub wiring: Vec<RegularWiringStatement>,
}

/// Definition of a net and possibly its routes.
#[derive(Clone, Debug)]
pub struct Net {
    /// Name of the net.
    /// Must be generated for MUSTJOIN nets.
    /// 'MUSTJOIN' is an invalid net name.
    pub name: Option<String>,
    /// Net name is generated for MUSTJOIN nets.
    pub mustjoin: Option<Mustjoin>,
    /// Terminals connected to the net.
    pub terminals: Vec<NetTerminal>,
    /// Names of special nets that are used to shield this net.
    /// The shield nets must be defined earlier in 'SPECIALNETS'.
    pub shield_nets: Vec<String>,
    pub vpin: HashMap<String, ()>,
    pub subnets: HashMap<String, ()>,
    /// Crosstalk class number.
    /// Default is `0` which will not be written to DEF.
    /// Should be a value from 0 to 200.
    pub xtalk_class: u16,
    /// Use another width rule than the default rule defined in the LEF WIDTH statement for the routing layer.
    pub non_default_rule: Option<String>,
    /// Specify the physical wiring.
    pub regular_wiring: Vec<RegularWiring>,
    /// Source from where the net was created.
    pub source: NetSource,
    /// Fixed bump: TBD
    pub fixed_bump: bool,
    /// Frequency of the net.
    /// Used as a hint for the router.
    pub frequency: Option<f64>,
    /// If this net results from partitioning another net, then
    /// this refers to the original net.
    pub original: Option<String>,
    /// Usage type of the net.
    pub net_use: DEFSignalUse,
    /// Desired routing pattern for the net.
    pub pattern: NetPattern,
    /// Estimated wire capacitance of this net.
    pub est_cap: Option<f64>,
    /// Weight of the net. Nets with high weight should be tried to keep short by routing tools.
    /// Default = 1.
    pub weight: u32,
    /// Additional properties.
    pub properties: HashMap<String, DefPropertyValue>,
}

/// Custom implementation of default because `weight` deviates from the default of `u32`.
impl Default for Net {
    fn default() -> Self {
        Net {
            name: Default::default(),
            mustjoin: Default::default(),
            terminals: Default::default(),
            shield_nets: Default::default(),
            vpin: Default::default(),
            subnets: Default::default(),
            xtalk_class: Default::default(),
            non_default_rule: Default::default(),
            regular_wiring: Default::default(),
            source: Default::default(),
            fixed_bump: Default::default(),
            frequency: Default::default(),
            original: Default::default(),
            net_use: Default::default(),
            pattern: Default::default(),
            est_cap: Default::default(),
            weight: 1,
            properties: Default::default(),
        }
    }
}

/// Definiton of a special net and possibly its routes.
#[derive(Clone, Debug, Default)]
pub struct SpecialNet {}

/// External pin definition in DEF.
/// Associates an external pin name with the internal net name.
#[derive(Clone, Debug, Default)]
pub struct Pin {
    /// Name of the external pin.
    pub pin_name: String,
    /// Name of the internal net.
    pub net_name: String,
    /// Mark the pin as 'special'. Special pins are to be routed with a special router with special wiring.
    pub special: bool,
    /// Signal direction of the pin. Typically this is specified in the timing library, not in DEF.
    pub direction: Option<DefPinDirection>,
    /// TBD
    pub net_expr: Option<String>,
    /// Net name where this pin should be connected if it is tied HIGH (constant logical 1).
    pub supply_sensitivity: Option<String>,
    /// Net name where this pin should be connected if it is tied LOW (constant logical 0).
    pub ground_sensitivity: Option<String>,
    /// Type of the signal for this pin. Default is 'SIGNAL'.
    pub signal_use: DEFSignalUse,

    /// Anntenna rules. TODO
    pub antenna_rules: (),

    /// Definitions of physical shapes of the pin.
    pub ports: Vec<PinPort>,
}

/// Definition of the port of a pin.
#[derive(Clone, Debug, Default)]
pub struct PinPort {
    /// Shapes of this port.
    pub port_statements: Vec<PinPortStatement>,
}

/// Definition of the shapes of a pin port.
#[derive(Clone, Debug)]
pub enum PinPortStatement {
    /// Rectangular shape of the pin port.
    Layer {
        /// Name of the layer.
        layer_name: String,
        /// Mask number.
        mask_num: Option<MaskNum>,
        /// SPACING: minimum spacing between other routing shapes an this pin.
        /// DESIGNRULEWIDTH: effective width of this pin used for calculating spacing.
        spacing_or_width: Option<DefSpacingOrDesignRuleWidth>,
        /// Rectangular shape on this port.
        rect: Rect<Coord>,
    },
    /// Polygon shape of the pin port.
    Polygon {
        /// Name of the layer.
        layer_name: String,
        /// Mask number.
        mask_num: Option<MaskNum>,
        /// SPACING: minimum spacing between other routing shapes an this pin.
        /// DESIGNRULEWIDTH: effective width of this pin used for calculating spacing.
        spacing_or_width: Option<DefSpacingOrDesignRuleWidth>,
        /// Polygon shape on this port.
        polygon: Polyline<Coord>,
    },
    /// Via which is part of the pin port.
    Via {
        /// Name of the via.
        via_name: String,
        /// Mask number.
        mask_num: Option<DefViaMaskNum>,
        /// Location of the via.
        location: Point<Coord>,
    },
}

/// Signal usage type of a pin in DEF.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DEFSignalUse {
    /// Digital data signal.
    SIGNAL,
    /// Supply net.
    POWER,
    /// Ground net.
    GROUND,
    /// Clock signal.
    CLOCK,
    /// Tie-signal.
    TIEOFF,
    /// Analog signal.
    ANALOG,
    /// Scan chain signal.
    SCAN,
    /// Reset signal.
    RESET,
}

impl Default for DEFSignalUse {
    fn default() -> Self {
        Self::SIGNAL
    }
}

impl FromStr for DEFSignalUse {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "SIGNAL" => Ok(Self::SIGNAL),
            "POWER" => Ok(Self::POWER),
            "GROUND" => Ok(Self::GROUND),
            "CLOCK" => Ok(Self::CLOCK),
            "TIEOFF" => Ok(Self::TIEOFF),
            "ANALOG" => Ok(Self::ANALOG),
            "SCAN" => Ok(Self::SCAN),
            "RESET" => Ok(Self::RESET),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DEFSignalUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SIGNAL => f.write_str("SIGNAL"),
            Self::POWER => f.write_str("POWER"),
            Self::GROUND => f.write_str("GROUND"),
            Self::CLOCK => f.write_str("CLOCK"),
            Self::TIEOFF => f.write_str("TIEOFF"),
            Self::ANALOG => f.write_str("ANALOG"),
            Self::SCAN => f.write_str("SCAN"),
            Self::RESET => f.write_str("RESET"),
        }
    }
}

/// Mask number of a via for multi patterning.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DefViaMaskNum {
    /// Mask number of top layer.
    pub top_mask_num: MaskNum,
    /// Mask number of cut layer.
    pub cut_mask_num: MaskNum,
    /// Mask number of bottom layer.
    pub bottom_mask_num: MaskNum,
}

impl FromStr for DefViaMaskNum {
    type Err = DefReadError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != 3 {
            Err(DefReadError::InvalidLiteral(input.to_string()))
        } else {
            Ok(Self {
                top_mask_num: MaskNum::from_str_radix(&input[0..1], 16).map_err(|e| DefReadError::ParseIntError(e))?,
                cut_mask_num: MaskNum::from_str_radix(&input[1..2], 16).map_err(|e| DefReadError::ParseIntError(e))?,
                bottom_mask_num: MaskNum::from_str_radix(&input[2..3], 16).map_err(|e| DefReadError::ParseIntError(e))?,
            })
        }
    }
}

impl fmt::Display for DefViaMaskNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:#0x}{:#0x}{:#0x}",
            self.top_mask_num, self.cut_mask_num, self.bottom_mask_num
        )
    }
}

/// Defines a physical area that can be used to place components or groups.
#[derive(Clone, Debug, Default)]
pub struct DefRegion {
    /// Physical region defined as a set of rectangles.
    pub regions: Vec<Rect<Coord>>,
    /// Type of the region constraint.
    /// Default: Assigned cells are placed in the region. Other cells can be placed in the region as well.
    pub region_type: Option<DefRegionType>,
    /// Properties of the region.
    pub properties: (),
}

/// Type of the region. A region defines a physical are where components should be placed.
/// Fence regions force components to be placed inside, guides are not hard constraints.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DefRegionType {
    /// Hard constraint. Only components assigned to this region are allowed to be placed here.
    Fence,
    /// Soft constraint (can be violated if necessary).
    Guide,
}

impl FromStr for DefRegionType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "FENCE" => Ok(Self::Fence),
            "GUIDE" => Ok(Self::Guide),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DefRegionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fence => f.write_str("FENCE"),
            Self::Guide => f.write_str("GUIDE"),
        }
    }
}

/// Define a group of components.
/// Optionally restrict the component locations to a region.
#[derive(Clone, Debug, Default)]
pub struct DefGroup {
    /// Names or name patterns of the components that belong to this group.
    pub component_names: Vec<String>,
    /// Name of the region in which the group must lie.
    /// Region restrictions defined by the component overwrite the restrictions defined by its group.
    pub region_name: Option<String>,
    /// User defined properties.
    pub properties: HashMap<String, DefPropertyValue>,
}

/// Definition of a standard-cell row.
#[derive(Clone, Debug, Default)]
pub struct DefRow {
    /// Name of the site where this row is located.
    pub site_name: String,
    /// Origin of the row.
    pub orig: (Coord, Coord),
    /// Orientation of all sites in the row.
    pub site_orient: DefOrient,
    /// Specify the repetition pattern of the referenced site.
    pub step_pattern: DefRowStepPattern,
    /// Properties associated with the row.
    pub properties: HashMap<String, DefPropertyValue>,
}

/// Repetition pattern for rows.
#[derive(Clone, Debug)]
pub struct DefRowStepPattern {
    /// Number of repetitions in x-direction. At least one of `num_x` or `num_y` must be `1`.
    pub num_x: u32,
    /// Number of repetitions in y-direction. At least one of `num_x` or `num_y` must be `1`.
    pub num_y: u32,
    /// Step size for x and y direction. By default
    /// the step size equals the size of the referenced site
    /// such that sites are arranged without overlap nor gap between.
    pub step: Option<(Coord, Coord)>,
}

impl Default for DefRowStepPattern {
    fn default() -> Self {
        Self {
            num_x: 1,
            num_y: 1,
            step: None,
        }
    }
}

/// Definition routing tracks as a grid of equally spaced lines.
#[derive(Clone, Debug, Default)]
pub struct DefTracks {
    /// Direction of the track. Can be horizontal when 'Y' is defined in DEF (true) or vertical with 'X' in DEF (false).
    pub is_horizontal: bool,
    /// Distance of the track line to the origin.
    /// For horizontal tracks this is the vertical offset (y coordinate),
    /// for vertical tracks this is the horizontal offset (y coordinate).
    pub start: Coord,
    /// Number of tracks for the grid. Must be larger than 0.
    pub num_tracks: u32,
    /// Spacing between tracks.
    pub step: Coord,
    /// An optional tuple `( mask number, SAMEMASK )`.
    /// mask number: Mask of the first track as used for double or triple patterning.
    /// Usually the mask is cycled for all subsequent tracks.
    /// SAMEMASK: Use the same mask number for all tracks.
    pub mask: Option<(MaskNum, bool)>,
    /// Routing layers to be used for this track. Possibly more than one.
    pub layers: Vec<String>,
}

/// Definition of via geometries or rules to generate a via.
#[derive(Clone, Debug)]
pub enum ViaDefinition {
    /// Explicit definition of the via by a set of geometrical shapes.
    ViaGeometry(Vec<ViaGeometry>),
    /// Implicit definition by design rules for the via.
    ViaRule,
}

/// Geometrical shapes of a via.
#[derive(Clone, Debug)]
pub struct ViaGeometry {
    /// Name of the
    pub layer: String,
    /// Mask number.
    pub mask_num: Option<MaskNum>,
    /// Rectangle or polygon shape.
    pub shape: DefRectOrPolygon,
}

/// Rule for via creation.
#[derive(Clone, Debug)]
pub struct ViaRule {
    /// Width and height of the via cut.
    pub cut_size: (Coord, Coord),
    /// Bottom metal layer of the via.
    pub bot_metal_layer: String,
    /// Layer of the via cut.
    pub cut_layer: String,
    /// Top metal layer of the via.
    pub top_metal_layer: String,
    /// Spacing in x and y directions.
    pub cut_spacing: (Coord, Coord),
    /// Via enclosure: bottom x, bottom y, top x, top y.
    pub enclosure: (Coord, Coord, Coord, Coord),
    num_cut_rows: u32,
    num_cut_cols: u32,
    origin: Point<Coord>,
    offset: (),
    cut_pattern: (),
}