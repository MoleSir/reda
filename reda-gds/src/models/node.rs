use derive_builder::Builder;
use crate::GdsDbCoord;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsNode {
    #[builder(default)]
    pub elf_flags: Option<i16>,
    #[builder(default)]
    pub plex: Option<i32>,

    pub layer: i16,
    pub node_type: i16,
    pub xy: Vec<GdsDbCoord>,
}
