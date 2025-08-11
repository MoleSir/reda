use derive_builder::Builder;
use reda_unit::{Capacitance, Inductance, Resistance};

use crate::ToSpice;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Resistor {
    pub name: String,
    pub node_pos: String,
    pub node_neg: String,
    pub resistance: Resistance,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Capacitor {
    pub name: String,
    pub node_pos: String,
    pub node_neg: String,
    pub capacitance: Capacitance,
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Inductor {
    pub name: String,
    pub node_pos: String,
    pub node_neg: String,
    pub inductance: Inductance,
}

impl ToSpice for Resistor {
    fn to_spice(&self) -> String {
        format!(
            "R{} {} {} {}",
            self.name,
            self.node_pos,
            self.node_neg,
            self.resistance.value()
        )
    }
}

impl ToSpice for Capacitor {
    fn to_spice(&self) -> String {
        format!(
            "C{} {} {} {}",
            self.name,
            self.node_pos,
            self.node_neg,
            self.capacitance
        )
    }
}

impl ToSpice for Inductor {
    fn to_spice(&self) -> String {
        format!(
            "L{} {} {} {}",
            self.name,
            self.node_pos,
            self.node_neg,
            self.inductance
        )
    }
}