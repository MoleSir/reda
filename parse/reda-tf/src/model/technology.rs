use std::collections::HashMap;
use derive_builder::Builder;

use super::TfValue;

#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(into))]
pub struct TfTechnology {
    pub name: String,
    pub dielectric: f64,
    pub grid_resolution: u32,
    pub time_unit: String,
    pub time_precision: u32,
    pub length_unit: String,
    pub length_precision: u32,
    pub voltage_unit: String,
    pub voltage_precision: u32,
    pub current_unit: String,
    pub current_precision: u32,
    pub power_unit: String,
    pub power_precision: u32,
    pub resistance_unit: String,
    pub resistance_precision: u32,
    pub capacitance_unit: String,
    pub capacitance_precision: u32,
    pub inductance_unit: String,
    pub inductance_precision: u32,
    pub min_baseline_temperature: u32,
    pub nom_baseline_temperature: u32,
    pub max_baseline_temperature: u32,

    pub attrs: HashMap<String, TfValue>,
}

impl TfTechnology {
    pub fn attr(&self, name: &str) -> Option<&TfValue> {
        self.attrs.get(name)
    }
}
