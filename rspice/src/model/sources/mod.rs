mod ac;
mod sine;
mod pulse;
mod pwl;

pub use ac::*;
pub use sine::*;
pub use pulse::*;
pub use pwl::*;
use runit::{Current, Voltage};

#[derive(Debug, Clone)]
pub struct Source {
    pub name: String, 
    pub node_pos: String, 
    pub node_neg: String, 
    pub value: SourceValue,
}

#[derive(Debug, Clone)]
pub enum SourceValue {
    DcVoltage(Voltage),
    DcCurrent(Current),
    AcVoltage(AcVoltage),
    AcCurrent(AcCurrent),
    Sin(SineVoltage),
    Pwl(PwlVoltage),
    Pulse(PulseVoltage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Voltage,
    Current,
}

impl Source {
    pub fn to_spice(&self) -> String {
        let (kind, value_spice) = match &self.value {
            SourceValue::DcVoltage(v) => (SourceKind::Voltage, format!("DC {}", v.to_string())),
            SourceValue::DcCurrent(c) => (SourceKind::Current, format!("DC {}", c.to_string())),
            SourceValue::AcVoltage(ac) => (SourceKind::Voltage, ac.to_spice()),
            SourceValue::AcCurrent(ac) => (SourceKind::Current, ac.to_spice()),
            SourceValue::Sin(sin) => (SourceKind::Voltage, sin.to_spice()),
            SourceValue::Pulse(pulse) => (SourceKind::Voltage, pulse.to_spice()),
            SourceValue::Pwl(pwl) => (SourceKind::Voltage, pwl.to_spice()),
        };

        let kind = if kind == SourceKind::Voltage { 'V' } else { 'I' };

        format!("{}{} {} {} {}", kind, self.name, self.node_pos, self.node_neg, value_spice)
    }
}