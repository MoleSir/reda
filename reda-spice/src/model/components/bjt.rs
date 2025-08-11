use derive_builder::Builder;

use crate::ToSpice;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct BJT {
    pub name: String,         // Qname
    pub collector: String,    // C
    pub base: String,         // B
    pub emitter: String,      // E
    pub model_name: String,   // BJT_modelName
}

impl ToSpice for BJT {
    fn to_spice(&self) -> String {
        format!(
            "Q{} {} {} {} {}",
            self.name,
            self.collector,
            self.base,
            self.emitter,
            self.model_name
        )
    }
}