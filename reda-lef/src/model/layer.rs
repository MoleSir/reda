use derive_builder::Builder;


#[derive(Debug, Clone)]
pub enum LefLayer {
    Cut(LefCutLayer),
    Implant(LefImplantLayer),
    Routing(LefRoutingLayer),
    Special(LefSpecialLayer),
}

// ===========================

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefRoutingLayer {
    pub name: String,
    #[builder(default)]
    pub mask: Option<u32>,
    pub direction: LefRoutingDirection,
    pub pitch: LefPitch,
    pub width: f64,
    #[builder(default)]
    pub area: Option<f64>,
    #[builder(default)]
    pub spacing_rules: Vec<LefRoutingSpacing>,
    #[builder(default)]
    pub max_width: Option<f64>,
    #[builder(default)]
    pub min_width: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub enum LefPitch {
    Uniform(f64),
    XY(f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub enum LefRoutingDirection {
    Horizontal,
    Vertical,
    Diag45,
    Diag135,
}

#[derive(Debug, Clone)]
pub struct LefRoutingSpacing {
    pub min_spacing: f64,
}

// ===========================

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefImplantLayer {
    pub name: String,
    #[builder(default)]
    pub mask: Option<u32>,
    #[builder(default)]
    pub width: Option<f64>,
    #[builder(default)]
    pub spacings: Vec<LefImplantSpacing>,
    #[builder(default)]
    pub properties: Vec<(String, String)>,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefImplantSpacing {
    pub min_spacing: f64,
    #[builder(default)]
    pub layer: Option<String>,
}

// ===========================


#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefCutLayer {
    pub name: String,
    #[builder(default)]
    pub mask: Option<u32>,
    #[builder(default)]
    pub width: Option<f64>,
    #[builder(default)]
    pub spacing: Vec<LefCutSpacing>,
    #[builder(default)]
    pub enclosures: Vec<LefEnclosure>,
}

#[derive(Debug, Clone)]
pub struct LefCutSpacing {
    pub cut_spacing: f64,
    pub center_to_center: bool,
    pub same_net: bool,
    pub constraint: Option<LefCutSpacingConstraint>,
}

#[derive(Debug, Clone)]
pub enum LefCutSpacingConstraint {
    Layer { name: String, stack: bool },
    AdjacentCuts { count: u8, within: f64, except_same_pg_net: bool },
    ParallelOverlap,
    Area(f64),
}

#[derive(Debug, Clone)]
pub struct LefEnclosure {
    pub above: bool,
    pub overhang1: f64,
    pub overhang2: f64,
    pub condition: Option<LefEnclosureCondition>,
}

#[derive(Debug, Clone)]
pub enum LefEnclosureCondition {
    Width { min_width: f64, except_extra_cut: Option<f64> },
    Length(f64),
}

// ===========================

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct LefSpecialLayer {
    pub name: String,
    pub layer_type: LefSpecialLayerType,
    #[builder(default)]
    pub mask: Option<u32>,
    #[builder(default)]
    pub properties: Vec<(String, String)>,
    #[builder(default)]
    pub lef58_type: Option<Lef58Type>,
    #[builder(default)]
    pub lef58_trimmed_metal: Option<Lef58TrimmedMetal>,
}

#[derive(Debug, Clone)]
pub enum LefSpecialLayerType {
    MasterSlice,
    Overlap,
}

#[derive(Debug, Clone)]
pub enum Lef58Type {
    NWell,
    PWell,
    AboveDieEdge,
    BelowDieEdge,
    Diffusion,
    TrimPoly,
    TrimMetal,
    Region,
}

#[derive(Debug, Clone)]
pub struct Lef58TrimmedMetal {
    pub metal_layer: String,
    pub mask: Option<u32>,
}