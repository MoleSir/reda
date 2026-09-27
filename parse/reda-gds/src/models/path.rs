use derive_builder::Builder;

use crate::GdsCoord;

use super::GdsPathType;

#[derive(Debug, Default, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct GdsPath {
    #[builder(default)]
    pub elf_flags: Option<i16>,

    #[builder(default)]
    pub plex: Option<i32>,

    pub layer: i16,
    pub data_type: i16,

    #[builder(default)]
    pub path_type: GdsPathType,
    #[builder(default)]
    pub width: Option<i32>,

    pub xy: Vec<GdsCoord>,
    
    #[builder(default)]
    pub purpose_layer: Option<i16>,
}

impl GdsPath {
    pub fn new(layer: i16, coords: impl Into<Vec<GdsCoord>>, width: i32) -> Self {
        GdsPathBuilder::default()
            .layer(layer)
            .data_type(0i16)
            .width(width)
            .xy(coords)
            .build()
            .unwrap()
    }
}