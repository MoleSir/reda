mod components;
mod sources;
mod control;
mod subckt;
mod value;

pub use components::*;
use reda_unit::{Number, Suffix, Unit, UnitNumber};
pub use sources::*;
pub use control::*;
pub use subckt::*;
pub use value::*;

use std::path::Path;
use crate::parse::{load_spice, SpiceReadError};

#[derive(Debug, Default)]
pub struct Spice {
    pub components: Vec<Component>,
    pub sources: Vec<Source>,
    pub simulation: Vec<SimCommand>,
    pub measures: Vec<MeasureCommand>,
    pub subckts: Vec<Subckt>,
    pub instances: Vec<Instance>,
    pub model: Vec<Model>,
}

impl Spice {
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, SpiceReadError> {
        load_spice(path)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

pub trait ToSpice {
    fn to_spice(&self) -> String;
}

impl ToSpice for Number {
    fn to_spice(&self) -> String {
        if self.suffix == Suffix::Mega {
            format!("{}{}", self.value, "Meg")
        } else {
            self.to_string()
        }
    }
}

impl<U: Unit> ToSpice for UnitNumber<U> {
    fn to_spice(&self) -> String {
        format!("{}{}", self.value().to_spice(), U::name())
    }
}