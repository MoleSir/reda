mod sim;
mod meas;

pub use sim::*;
pub use meas::*;

use super::ToSpice;

pub struct IncludeCommand(pub String);

impl ToSpice for IncludeCommand {
    fn to_spice(&self) -> String {
        format!(".include {}", self.0)
    }
}