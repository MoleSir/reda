use crate::{GdsDbCoord, GdsTransform};
use derive_builder::Builder;
use super::{GdsPathType, GdsPresentation};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GdsText {
    #[builder(default)]
    pub elf_flags: Option<i16>,
    #[builder(default)]
    pub plex: Option<i32>,

    pub layer: i16,
    pub text_type: i16,
    pub xy: Vec<GdsDbCoord>,
    pub string: String,

    #[builder(default)]
    pub presentation: Option<GdsPresentation>,
    #[builder(default)]
    pub path_type: GdsPathType,
    #[builder(default)]
    pub width: Option<i32>,

    #[builder(default)]
    pub transform: Option<GdsTransform>,
}
