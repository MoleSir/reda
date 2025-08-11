use crate::{GdsDateTime, GdsBoundary, GdsPath, GdsSref, GdsAref, GdsText};
use super::{GdsBox, GdsNode};

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
}