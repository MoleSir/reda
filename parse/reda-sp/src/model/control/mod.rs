mod sim;
mod meas;

use reda_unit::Temperature;
pub use sim::*;
pub use meas::*;

use super::ToSpice;

pub struct IncludeCommand(pub String);

pub struct TemperatureCommand(pub Temperature);

impl ToSpice for IncludeCommand {
    fn to_spice(&self) -> String {
        format!(".include \"{}\"", self.0)
    }
}

impl ToSpice for TemperatureCommand {
    fn to_spice(&self) -> String {
        format!(".TEMP {}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::{IncludeCommand, ToSpice};

    #[test]
    fn test_include() {
        println!("{}", IncludeCommand("bitcell.sp".into()).to_spice())
    }
}