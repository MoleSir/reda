use std::collections::HashMap;
use derive_builder::Builder;
use reda_unit::{Length, Number};

use crate::ToSpice;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct MosFET {
    pub name: String,        // Mname
    pub drain: String,       // ND
    pub gate: String,        // NG
    pub source: String,      // NS
    pub bulk: String,        // bulk）
    pub model_name: String,  // ModName
    pub length: Length,      // L=VAL
    pub width: Length,       // W=VAL
    #[builder(default)]
    pub parameters: HashMap<String, Number>,
}

impl ToSpice for MosFET {
    fn to_spice(&self) -> String {
        let mut line = format!(
            "M{} {} {} {} {} {} L={} W={}",
            self.name,
            self.drain,
            self.gate,
            self.source,
            self.bulk,
            self.model_name,
            self.length,
            self.width
        );
        for (k, v) in &self.parameters {
            line.push_str(&format!(" {}={}", k, v));
        }
        line
    }
}

#[derive(Debug, Clone)]
pub enum MOSFETKind {
    NMOS,
    PMOS,
}
