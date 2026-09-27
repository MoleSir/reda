use derive_builder::Builder;
use crate::{GdsCoord, GdsTransform};

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

    pub position: GdsCoord,
}

impl GdsAref {
    pub fn new(ref_name: impl Into<String>, row: i16, col: i16, position: impl Into<GdsCoord>, transform: Option<GdsTransform>) -> Self {
        Self {
            elf_flags: None,
            plex: None,
            s_name: ref_name.into(),
            transform,
            col, row,
            position: position.into()
        }
    }

    pub fn position(&self) -> GdsCoord {
        self.position
    }

    pub fn magnification(&self) -> f64 {
        self.transform.map(|t| t.magnification()).unwrap_or(1.0)
    }

    pub fn angle(&self) -> f64 {
        self.transform.map(|t| t.angle()).unwrap_or(0.0)
    }
}