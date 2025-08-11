use derive_builder::Builder;

use crate::GdsDbCoord;

use super::GdsPathType;

#[derive(Debug, Default, Clone, Builder)]
#[builder(setter(strip_option))]
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

    pub xy: Vec<GdsDbCoord>,
    
    #[builder(default)]
    pub purpose_layer: Option<i16>,
}