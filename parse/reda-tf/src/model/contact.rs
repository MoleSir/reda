use std::collections::HashMap;
use derive_builder::Builder;
use super::TfValue;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TfContact {
    pub name: String,
    pub contact_id: u32,

    pub cut_layer: String,
    pub lower_layer: String,
    pub upper_layer: String,
    
    pub cut_width: f64,
    pub cut_height: f64,

    pub upper_layer_enc_width: f64,
    pub upper_layer_enc_height: f64,
    pub lower_layer_enc_width: f64,
    pub lower_layer_enc_height: f64,
    
    pub min_cut_spacing: f64,

    pub attrs: HashMap<String, TfValue>,
}