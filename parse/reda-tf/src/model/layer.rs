use std::{collections::HashMap, ops::{Deref, DerefMut}};
use derive_builder::Builder;

use super::{TfColor, TfValue};

#[derive(Debug, Clone)]
pub enum TfLayer {
    Masterslice(TfMastersliceLayer),
    Route(TfRouteLayer),
    Cut(TfCutLayer),
}

impl TfLayer {
    pub fn name(&self) -> &str {
        match self {
            TfLayer::Route(l) => &l.name,
            TfLayer::Cut(l) => &l.name,
            TfLayer::Masterslice(l) => &l.name,
        }
    }

    pub fn mask_name(&self) -> &str {
        match self {
            TfLayer::Route(l) => &l.mask_name,
            TfLayer::Cut(l) => &l.mask_name,
            TfLayer::Masterslice(l) => &l.mask_name,
        }
    }

    pub fn layer_number(&self) -> u32 {
        match self {
            TfLayer::Route(l) => l.layer_number,
            TfLayer::Cut(l) => l.layer_number,
            TfLayer::Masterslice(l) => l.layer_number,
        }
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfLayerBase {
    pub mask_name: String,
    pub layer_number: u32,
    pub visible: bool,
    pub selectable: bool,
    pub blink: bool,
    #[builder(default)]
    pub is_defaultlayer: bool,
    pub line_style: String,
    pub pattern: String,
    pub color: TfColor,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfMastersliceLayer {
    pub name: String,
    pub base: TfLayerBase,

    pub pitch: f64,
    pub default_width: f64,
    pub min_width: f64,
    pub min_spacing: f64,

    pub attrs: HashMap<String, TfValue>,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfRouteLayer {
    pub name: String,
    pub base: TfLayerBase,

    pub pitch: f64,
    pub default_width: f64,
    pub min_width: f64,
    pub min_spacing: f64,

    pub attrs: HashMap<String, TfValue>,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfCutLayer {
    pub name: String,
    pub base: TfLayerBase,

    pub pitch: f64,
    pub default_width: f64,
    pub min_width: f64,
    pub min_spacing: f64,

    pub attrs: HashMap<String, TfValue>,
}

#[derive(Debug, Clone)]
pub struct TfEndLine2Neighbor {
    pub line2neighbor_threshold: f64,
    pub line2neighbor_minspacing: f64,
    pub line2neighbor_side_minspacing: f64,
    pub line2neighbor_corner_keepout_width: f64,
    pub line2neighbor_side_keepout_length: f64,
}

#[derive(Debug, Clone)]
pub struct TfUnit {
    pub min_resistance: f64,
    pub nom_resistance: f64,
    pub max_resistance: f64,

    pub min_capacitance: f64,
    pub nom_capacitance: f64,
    pub max_capacitance: f64,

    pub min_thickness: f64,
    pub nom_thickness: f64,
    pub max_thickness: f64,
}

#[derive(Debug, Clone)]
pub struct TfFatTbl {
    pub fat_tbl_dimension: u32,
    pub fat_tbl_threshold: Vec<f64>,
    pub fat_tbl_parallel_length: Vec<f64>,
    pub fat_tbl_spacing: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TfCutFatExtTbl {
    pub fat_tbl_extension_dimension: u32,
    pub fat_tbl_extension_threshold: Vec<f64>,
    pub fat_tbl_extension_range_dimension: u32,
    pub fat_tbl_extension_range: Vec<f64>,
    pub fat_tbl_extension_area_dimension: u32,
    pub fat_tbl_extension_area_threshold: Vec<f64>,
    pub fat_tbl_extension_contact_number: Vec<u32>,
    pub fat_tbl_extension_mincuts: Vec<u32>,
}

macro_rules! define_deref_for_base {
    ($layer_type:ident) => {
        impl Deref for $layer_type {
            type Target = TfLayerBase;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl DerefMut for $layer_type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        impl $layer_type {
            pub fn attr(&self, name: &str) -> Option<&TfValue> {
                self.attrs.get(name)
            }
        }
        
    };
}

define_deref_for_base!(TfMastersliceLayer);
define_deref_for_base!(TfRouteLayer);
define_deref_for_base!(TfCutLayer);