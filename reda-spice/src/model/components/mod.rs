mod basic;
mod diode;
mod bjt;
mod mosfet;
mod model;

pub use basic::*;
pub use diode::*;
pub use bjt::*;
pub use mosfet::*;
pub use model::*;

use super::ToSpice;

#[derive(Debug, Clone)]
pub enum Component {
    R(Resistor),
    C(Capacitor),
    L(Inductor),
    D(Diode),
    Q(BJT),
    M(MosFET),
}

impl ToSpice for Component {
    fn to_spice(&self) -> String {
        match &self {
            &Self::R(c) => c.to_spice(),
            &Self::C(c) => c.to_spice(), 
            &Self::L(c) => c.to_spice(), 
            &Self::D(c) => c.to_spice(), 
            &Self::Q(c) => c.to_spice(), 
            &Self::M(c) => c.to_spice(), 
        }
    }
}