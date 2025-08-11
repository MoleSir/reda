use derive_builder::Builder;
use crate::{GdsDbCoord, GdsTransform};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsAref {
    #[builder(default)]
    pub elf_flags: Option<i16>,
    #[builder(default)]
    pub plex: Option<i32>,

    pub s_name: String,
    
    #[builder(default)]
    pub transform: Option<GdsTransform>,
    
    pub col: i16,
    pub row: i16,

    pub xy: Vec<GdsDbCoord>,
}
