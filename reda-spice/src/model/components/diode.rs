use derive_builder::Builder;

use crate::ToSpice;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Diode {
    pub name: String,        // Dname
    pub node_pos: String,    // N+
    pub node_neg: String,    // N-
    pub model_name: String,  // MODName
}

impl ToSpice for Diode {
    fn to_spice(&self) -> String {
        format!(
            "{} {} {} {}",
            self.name,
            self.node_pos,
            self.node_neg,
            self.model_name
        )
    }
}