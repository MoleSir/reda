use crate::{GdsDateTime, GdsBoundary, GdsPath, GdsSref, GdsAref, GdsText};
use super::{GdsBox, GdsCoord, GdsNode, GdsTransform};

#[derive(Debug, Default, Clone)]
pub struct GdsStructure {
    pub name: String,
    pub create_date: GdsDateTime,
    pub modify_date: GdsDateTime,
    pub boundarys: Vec<GdsBoundary>,
    pub paths: Vec<GdsPath>,
    pub srefs: Vec<GdsSref>,
    pub arefs: Vec<GdsAref>,
    pub texts: Vec<GdsText>,
    pub nodes: Vec<GdsNode>,
    pub boxes: Vec<GdsBox>,
}

impl GdsStructure {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn add_rectangle(&mut self, layer: i16, leftdown: impl Into<GdsCoord>, rightup: impl Into<GdsCoord>) {
        self.boundarys.push(GdsBoundary::rect(layer, leftdown, rightup));
    }

    pub fn add_text(&mut self, layer: i16, offset: impl Into<GdsCoord>, text: impl Into<String>) {
        self.texts.push(GdsText::new(layer, offset, text));
    }

    pub fn add_path(&mut self, layer: i16, coords: impl Into<Vec<GdsCoord>>, width: i32) {
        self.paths.push(GdsPath::new(layer, coords, width));
    }

    pub fn add_sref(&mut self, ref_name: impl Into<String>, coord: impl Into<GdsCoord>, transform: Option<GdsTransform>) {
        self.srefs.push(GdsSref::new(ref_name, coord, transform));
    }

    pub fn add_aref(&mut self, ref_name: impl Into<String>, row: i16, col: i16, coord: impl Into<GdsCoord>, transform: Option<GdsTransform>) {
        self.arefs.push(GdsAref::new(ref_name, row, col, coord, transform));
    }
}
