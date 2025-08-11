use derive_builder::Builder;
use crate::{GdsDbCoord, GdsTransform};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsSref {
    #[builder(default)]
    pub elf_flags: Option<i16>,
    #[builder(default)]
    pub plex: Option<i32>,

    pub s_name: String,
    
    #[builder(default)]
    pub transform: Option<GdsTransform>,
    
    pub xy: Vec<GdsDbCoord>,
}
